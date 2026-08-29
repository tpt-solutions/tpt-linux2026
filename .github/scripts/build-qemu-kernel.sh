#!/usr/bin/env bash
# Build a QEMU-bootable kernel + minimal initramfs for the given kernel version
# (e.g. "6.6", "6.12", or "stable"). The result is written under
# .qemu-kernels/<version>/ as `bzImage` and `initramfs.cpio.gz`.
#
# The initramfs is assembled by build-initramfs.sh from the workspace test
# binaries so that `run-qemu-tests.sh` can execute the kernel integration tests.
set -euo pipefail

VERSION="${1:?usage: build-qemu-kernel.sh <version>}"
OUT_DIR="$(pwd)/.qemu-kernels/${VERSION}"
SRC_DIR="$(pwd)/.qemu-kernels/src-${VERSION}"
JOBS="$(nproc)"

mkdir -p "${OUT_DIR}" "${SRC_DIR}"

if [[ ! -d "${SRC_DIR}/linux" ]]; then
  case "${VERSION}" in
    stable) URL="https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/snapshot/linux-master.tar.gz" ;;
    *)      URL="https://cdn.kernel.org/pub/linux/kernel/v${VERSION%%.*}.x/linux-${VERSION}.tar.xz" ;;
  esac
  echo ">> fetching kernel ${VERSION} from ${URL}"
  curl -fsSL "${URL}" -o "${SRC_DIR}/kernel.tar.xz"
  tar -xf "${SRC_DIR}/kernel.tar.xz" -C "${SRC_DIR}"
fi

KDIR="${SRC_DIR}/linux"
cd "${KDIR}"

if [[ ! -f .config ]]; then
  make defconfig
  # Enable the BPF / LSM / userfaultfd / fanotify / audit features the crates need.
  scripts/config \
    -e BPF -e BPF_SYSCALL -e BPF_LSM -e DEBUG_INFO_BTF \
    -e USERFAULTFD -e FANOTIFY -e AUDIT -e AUDITSYSCALL \
    -e KPROBES -e KRETPROBES
fi

echo ">> building kernel ${VERSION}"
make -j"${JOBS}" bzImage

cp "arch/x86/boot/bzImage" "${OUT_DIR}/bzImage"
echo ">> wrote ${OUT_DIR}/bzImage"
