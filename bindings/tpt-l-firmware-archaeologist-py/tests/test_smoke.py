"""Smoke tests for the tpt-l-firmware-archaeologist Python bindings."""

import struct

from tpt_l_firmware_archaeologist_py import (
    parse_fadt,
    parse_firmware_volume,
    parse_pe_image,
    parse_fpt,
)


def _fadt_blob():
    buf = bytearray(64)
    buf[0:4] = b"FACP"
    buf[4:8] = struct.pack("<I", 64)
    buf[40:44] = struct.pack("<I", 0x1000)
    buf[44:48] = struct.pack("<I", 0x2000)
    buf[48] = 2
    return bytes(buf)


def test_parse_fadt():
    d = parse_fadt(_fadt_blob())
    assert d["dsdt"] == 0x2000
    assert d["firmware_ctrl"] == 0x1000


def test_parse_firmware_volume():
    buf = bytearray(256)
    struct.pack_into("<Q", buf, 32, 0x1000)
    buf[40:44] = struct.pack("<I", 0x4856465F)
    struct.pack_into("<H", buf, 48, 56)
    d = parse_firmware_volume(bytes(buf))
    assert d["header_length"] == 56


def test_parse_pe_image():
    buf = bytearray(0x100)
    buf[0:2] = b"MZ"
    struct.pack_into("<I", buf, 0x3C, 0x80)
    buf[0x80:0x84] = b"PE\0\0"
    struct.pack_into("<H", buf, 0x84, 0x8664)  # x86_64
    struct.pack_into("<H", buf, 0x86, 1)
    struct.pack_into("<H", buf, 0x94, 0xF0)
    struct.pack_into("<H", buf, 0x98, 0x20B)  # PE32+
    struct.pack_into("<I", buf, 0xA8, 0x1000)
    d = parse_pe_image(bytes(buf))
    assert d["machine"] == "x86_64"
    assert d["is_pe32_plus"] is True


def test_parse_fpt():
    buf = bytearray(28 + 2 * 32)
    buf[0:4] = struct.pack("<I", 0x5F465054)
    struct.pack_into("<I", buf, 4, 2)
    struct.pack_into("<H", buf, 12, 28)
    buf[28:32] = b"MEFS"
    struct.pack_into("<I", buf, 36, 0x1000)
    struct.pack_into("<I", buf, 40, 0x2000)
    d = parse_fpt(bytes(buf))
    assert d["num_entries"] == 2
