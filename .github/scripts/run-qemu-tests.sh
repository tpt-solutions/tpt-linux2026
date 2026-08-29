#!/usr/bin/env bash
# Boot the kernel built by build-qemu-kernel.sh under QEMU and run the workspace
# kernel-integration tests (those gated behind RUN_QEMU=1 / #[ignore]).
set -euo pipefail

VERSION="${1:?usage: run-qemu-tests.sh <version>}"
WORKSPACE="$(pwd)"
OUT_DIR="${WORKSPACE}/.qemu-kernels/${VERSION}"
IMG="${OUT_DIR}/bzImage"
INITRAMFS="${OUT_DIR}/initramfs.cpio.gz"

if [[ ! -f "${IMG}" ]]; then
  echo ">> kernel image missing, building it first"
  "${WORKSPACE}/.github/scripts/build-qemu-kernel.sh" "${VERSION}"
fi

# Build the test binaries and pack them into an initramfs containing a tiny
# init that executes `cargo test` for the kernel-gated tests.
echo ">> building integration test binaries"
cargo test --workspace --no-run
INIT_BUILD="${OUT_DIR}/initramfs-build"
rm -rf "${INIT_BUILD}" && mkdir -p "${INIT_BUILD}/bin" "${INIT_BUILD}/usr"

find target/debug/deps -maxdepth 1 -type f -executable -name '*-*[0-9a-f]*' \
  -exec cp -t "${INIT_BUILD}/bin" {} + 2>/dev/null || true

cat > "${INIT_BUILD}/init" <<'EOF'
#!/bin/sh
mount -t proc none /proc
mount -t sysfs none /sys
echo "tpt-linux2026 QEMU init running tests (kernel: ${TPT_KERNEL})"
for t in /bin/*; do
  case "$t" in *.so|*.rlib) continue ;; esac
  "$t" --ignored --test-threads=1 2>&1
done
poweroff -f
EOF
chmod +x "${INIT_BUILD}/init"

( cd "${INIT_BUILD}" && find . | cpio -o -H newc | gzip ) > "${INITRAMFS}"

echo ">> booting QEMU (kernel ${VERSION})"
qemu-system-x86_64 \
  -m 2G -smp 2 -nographic \
  -kernel "${IMG}" \
  -initrd "${INITRAMFS}" \
  -append "console=ttyS0 tpt.kernel=${VERSION} quiet" \
  -net none
