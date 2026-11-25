#!/usr/bin/env python3
"""Generate or copy a high-DPI PNG app icon for Android, iOS, macOS, and Web bundles."""

import math
import os
import shutil
import struct
import subprocess
import sys
import zlib


def write_png(path: str, width: int, height: int, rgba: bytes) -> None:
    os.makedirs(os.path.dirname(os.path.abspath(path)), exist_ok=True)

    def chunk(tag: bytes, data: bytes) -> bytes:
        return (
            struct.pack(">I", len(data))
            + tag
            + data
            + struct.pack(">I", zlib.crc32(tag + data) & 0xFFFFFFFF)
        )

    raw = bytearray()
    stride = width * 4
    for y in range(height):
        raw.append(0)
        raw.extend(rgba[y * stride : (y + 1) * stride])
    ihdr = struct.pack(">IIBBBBB", width, height, 8, 6, 0, 0, 0)
    with open(path, "wb") as f:
        f.write(
            b"\x89PNG\r\n\x1a\n"
            + chunk(b"IHDR", ihdr)
            + chunk(b"IDAT", zlib.compress(bytes(raw), 6))
            + chunk(b"IEND", b"")
        )


def render_default_icon(size: int) -> bytes:
    buf = bytearray(size * size * 4)
    cx = (size - 1) * 0.5
    cy = (size - 1) * 0.5
    inv = 1.0 / max(1.0, float(size))
    for y in range(size):
        ny = (y - cy) * inv * 2.0
        row = y * size * 4
        for x in range(size):
            nx = (x - cx) * inv * 2.0
            # Deep obsidian gradient background with subtle aurora halo
            grad = max(0.0, min(1.0, (nx + ny + 2.0) * 0.25))
            r = 9.0 + 18.0 * grad
            g = 13.0 + 16.0 * grad
            b = 26.0 + 36.0 * grad

            # Soft radial aurora glow behind emblem
            d_center = math.hypot(nx, ny)
            halo = max(0.0, 1.0 - d_center * 1.15) ** 2.2
            r += 48.0 * halo
            g += 110.0 * halo
            b += 165.0 * halo

            # Outer luminous ring
            ring = abs(d_center - 0.66)
            if ring < 0.045:
                k = max(0.0, 1.0 - ring / 0.045)
                t = max(0.0, min(1.0, (nx + 0.7) / 1.4))
                rr = 56.0 + (168.0 - 56.0) * t
                gg = 189.0 + (85.0 - 189.0) * t
                bb = 248.0 + (247.0 - 248.0) * t
                r = r * (1.0 - k) + rr * k
                g = g * (1.0 - k) + gg * k
                b = b * (1.0 - k) + bb * k

            # Crisp geometric "E" / prism bars centered in the icon
            in_left_spine = -0.34 <= nx <= -0.18 and -0.36 <= ny <= 0.36
            in_top_bar = -0.34 <= nx <= 0.34 and -0.36 <= ny <= -0.20
            in_mid_bar = -0.34 <= nx <= 0.24 and -0.08 <= ny <= 0.08
            in_bot_bar = -0.34 <= nx <= 0.34 and 0.20 <= ny <= 0.36
            if in_left_spine or in_top_bar or in_mid_bar or in_bot_bar:
                t = max(0.0, min(1.0, (nx + 0.34) / 0.68))
                r = 224.0 + (56.0 - 224.0) * t * 0.45
                g = 242.0 + (189.0 - 242.0) * t * 0.35
                b = 254.0

            # Bright diamond accent on top-right
            d_accent = abs(nx - 0.28) + abs(ny + 0.0)
            if d_accent < 0.11:
                r, g, b = 168.0, 85.0, 247.0

            idx = row + x * 4
            buf[idx] = int(max(0.0, min(255.0, r)))
            buf[idx + 1] = int(max(0.0, min(255.0, g)))
            buf[idx + 2] = int(max(0.0, min(255.0, b)))
            buf[idx + 3] = 255
    return bytes(buf)


def main() -> int:
    if len(sys.argv) < 3:
        sys.stderr.write("Usage: generate-icon.py <output_png> <size_px> [custom_source_png]\n")
        return 2
    out_path = sys.argv[1]
    size = max(16, min(1024, int(sys.argv[2])))
    custom_src = sys.argv[3] if len(sys.argv) > 3 and sys.argv[3] else os.environ.get("EUV_APP_ICON", "")
    if custom_src and os.path.isfile(custom_src):
        os.makedirs(os.path.dirname(os.path.abspath(out_path)), exist_ok=True)
        if shutil.which("sips"):
            subprocess.run(
                ["sips", "-z", str(size), str(size), "-s", "format", "png", custom_src, "--out", out_path],
                check=True,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
            )
            return 0
        shutil.copyfile(custom_src, out_path)
        return 0
    rgba = render_default_icon(size)
    write_png(out_path, size, size, rgba)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
