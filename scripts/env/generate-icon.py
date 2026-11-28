#!/usr/bin/env python3
"""Generate Euv Studio multi-size RGBA PNG app icons from the canonical README logo URL.

Usage:
    generate-icon.py <output_png> <size_px> [custom_source_png_or_url]
    generate-icon.py --all [--output-dir <dir>] [--source <png>] [--url <url>]
    generate-icon.py --svg <output_svg>
"""

from __future__ import annotations

import argparse
import gzip
import hashlib
import math
import os
import shutil
import struct
import subprocess
import sys
import tempfile
import urllib.request
import zlib
from pathlib import Path
from typing import Iterable, Optional, Sequence

# Canonical README logo URL shared with README.md and all crate READMEs.
DEFAULT_LOGO_URL: str = "https://ltpp.vip/github/pages/docs-pages/pages/img/euv.png"

# Official Euv Studio branding export dimensions (square RGBA transparent PNGs).
BRANDING_SIZES: tuple[int, ...] = (1024, 512, 256, 192, 180, 128, 64, 32)

# Canonical 1024x1024 coordinate constants.
CANVAS_SIZE: float = 1024.0
CANVAS_CENTER: float = 512.0

# Optical subject fill ratio when normalizing padding from a raw transparent source PNG (~89%).
SOURCE_FILL_RATIO: float = 0.89

# Official Emperor-Armor Cybernetic Chinese Dragon palette.
COLOR_SOLAR_BRIGHT: str = "#FFF2A6"
COLOR_IMPERIAL_GOLD: str = "#FFD24D"
COLOR_ROYAL_GOLD: str = "#EAB308"
COLOR_AMBER_GLOW: str = "#F59E0B"
COLOR_BURNISHED_GOLD: str = "#B45309"
COLOR_OBSIDIAN_TOP: str = "#222736"
COLOR_OBSIDIAN_DEEP: str = "#11141C"
COLOR_CRIMSON_BRIGHT: str = "#FF3B5C"
COLOR_CRIMSON_RUBY: str = "#E6192E"
COLOR_CRIMSON_DEEP: str = "#991122"
COLOR_SILVER_BRIGHT: str = "#F8FAFC"
COLOR_SILVER_MID: str = "#CBD5E1"
COLOR_SILVER_STEEL: str = "#64748B"


def _hex_to_rgb01(hex_color: str) -> tuple[float, float, float]:
    h = hex_color.lstrip("#")
    return (
        int(h[0:2], 16) / 255.0,
        int(h[2:4], 16) / 255.0,
        int(h[4:6], 16) / 255.0,
    )


def write_png(path: str, width: int, height: int, rgba: bytes) -> None:
    """Write an 8-bit RGBA PNG file deterministically with zero external dependencies."""
    parent = os.path.dirname(os.path.abspath(path))
    if parent:
        os.makedirs(parent, exist_ok=True)

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
            + chunk(b"IDAT", zlib.compress(bytes(raw), 9))
            + chunk(b"IEND", b"")
        )


def generate_logo_svg() -> str:
    """Return the canonical 1024x1024 transparent-background Euv Studio Emperor-Armor Dragon SVG."""
    return f"""<svg xmlns="http://www.w3.org/2000/svg" width="1024" height="1024" viewBox="0 0 1024 1024" fill="none">
  <title>Euv Studio</title>
  <defs>
    <linearGradient id="euvGoldArmor" x1="260" y1="140" x2="760" y2="860" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="{COLOR_SOLAR_BRIGHT}" />
      <stop offset="28%" stop-color="{COLOR_IMPERIAL_GOLD}" />
      <stop offset="68%" stop-color="{COLOR_ROYAL_GOLD}" />
      <stop offset="100%" stop-color="{COLOR_BURNISHED_GOLD}" />
    </linearGradient>
    <linearGradient id="euvGoldHoriz" x1="180" y1="512" x2="860" y2="512" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="{COLOR_IMPERIAL_GOLD}" />
      <stop offset="50%" stop-color="{COLOR_SOLAR_BRIGHT}" />
      <stop offset="100%" stop-color="{COLOR_ROYAL_GOLD}" />
    </linearGradient>
    <linearGradient id="euvWingSolarL" x1="72" y1="112" x2="430" y2="500" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="{COLOR_SOLAR_BRIGHT}" stop-opacity="0.96" />
      <stop offset="48%" stop-color="{COLOR_IMPERIAL_GOLD}" stop-opacity="0.90" />
      <stop offset="100%" stop-color="{COLOR_AMBER_GLOW}" stop-opacity="0.88" />
    </linearGradient>
    <linearGradient id="euvWingSolarR" x1="952" y1="100" x2="540" y2="520" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="{COLOR_SOLAR_BRIGHT}" stop-opacity="0.96" />
      <stop offset="48%" stop-color="{COLOR_IMPERIAL_GOLD}" stop-opacity="0.90" />
      <stop offset="100%" stop-color="{COLOR_AMBER_GLOW}" stop-opacity="0.88" />
    </linearGradient>
    <linearGradient id="euvObsidian" x1="360" y1="220" x2="720" y2="820" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="{COLOR_OBSIDIAN_TOP}" />
      <stop offset="55%" stop-color="#171B26" />
      <stop offset="100%" stop-color="{COLOR_OBSIDIAN_DEEP}" />
    </linearGradient>
    <linearGradient id="euvRubyGem" x1="320" y1="180" x2="620" y2="700" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="{COLOR_CRIMSON_BRIGHT}" />
      <stop offset="48%" stop-color="{COLOR_CRIMSON_RUBY}" />
      <stop offset="100%" stop-color="{COLOR_CRIMSON_DEEP}" />
    </linearGradient>
    <linearGradient id="euvSilverTalon" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" stop-color="{COLOR_SILVER_BRIGHT}" />
      <stop offset="52%" stop-color="{COLOR_SILVER_MID}" />
      <stop offset="100%" stop-color="{COLOR_SILVER_STEEL}" />
    </linearGradient>
    <filter id="euvSolarGlow" x="20" y="20" width="984" height="984" filterUnits="userSpaceOnUse" color-interpolation-filters="sRGB">
      <feDropShadow dx="0" dy="8" stdDeviation="10" flood-color="#11141C" flood-opacity="0.28" />
    </filter>
  </defs>

  <g filter="url(#euvSolarGlow)">
    <!-- Sweeping luminous golden tail plume behind right wing -->
    <path d="M 744 630 C 858 626 942 544 926 426 C 910 326 838 258 764 214 C 796 150 842 104 888 68 C 824 94 762 146 728 214 C 700 272 710 344 760 402 C 836 456 876 520 844 584 C 814 632 764 646 712 634 Z" fill="url(#euvWingSolarR)" stroke="{COLOR_BURNISHED_GOLD}" stroke-width="5" stroke-linejoin="round" />
    <path d="M 784 212 C 818 158 848 122 874 96 M 756 266 C 798 228 842 184 870 144" stroke="#FFFBEB" stroke-width="4.5" stroke-linecap="round" opacity="0.82" />

    <!-- Grand Left Solar-Gold Holographic Circuit Wing -->
    <path d="M 384 446 L 176 206 L 78 118 C 72 174 98 232 148 272 L 90 226 C 94 282 128 334 186 368 L 122 336 C 142 390 192 430 256 448 L 182 436 C 216 480 282 504 358 494 Z" fill="url(#euvWingSolarL)" stroke="{COLOR_AMBER_GLOW}" stroke-width="5" stroke-linejoin="round" />
    <!-- Left Wing Holographic Circuit Traces -->
    <path d="M 134 186 L 236 298 L 280 298 M 162 282 L 244 352 L 308 352 M 204 374 L 274 424 L 344 424" stroke="#FFFBEB" stroke-width="5" stroke-linecap="round" stroke-linejoin="round" opacity="0.86" />
    <circle cx="284" cy="298" r="7" fill="#FFFBEB" />
    <circle cx="312" cy="352" r="7" fill="#FFFBEB" />
    <circle cx="348" cy="424" r="6" fill="#FFFBEB" />
    <!-- Left Mechanical Wing Spar & Crimson Ruby Leading Beacon -->
    <path d="M 392 456 L 204 222 L 228 202 L 418 424 Z" fill="url(#euvGoldArmor)" stroke="{COLOR_OBSIDIAN_DEEP}" stroke-width="6" stroke-linejoin="round" />
    <path d="M 206 216 L 94 102 C 86 94 98 80 108 88 L 222 198 Z" fill="url(#euvRubyGem)" stroke="{COLOR_CRIMSON_DEEP}" stroke-width="4" stroke-linejoin="round" />
    <path d="M 110 96 L 206 192" stroke="#FFC2CE" stroke-width="4" stroke-linecap="round" opacity="0.85" />

    <!-- Grand Right Solar-Gold Holographic Circuit Wing -->
    <path d="M 520 510 L 796 226 L 944 106 C 948 168 914 232 856 276 L 930 214 C 920 278 874 342 806 384 L 892 330 C 864 396 806 446 732 468 L 818 446 C 768 504 686 544 584 548 Z" fill="url(#euvWingSolarR)" stroke="{COLOR_AMBER_GLOW}" stroke-width="5" stroke-linejoin="round" />
    <!-- Right Wing Holographic Circuit Traces -->
    <path d="M 886 184 L 764 304 L 704 304 M 844 294 L 732 384 L 664 384 M 784 394 L 688 456 L 614 456" stroke="#FFFBEB" stroke-width="5" stroke-linecap="round" stroke-linejoin="round" opacity="0.86" />
    <circle cx="700" cy="304" r="7" fill="#FFFBEB" />
    <circle cx="660" cy="384" r="7" fill="#FFFBEB" />
    <circle cx="610" cy="456" r="7" fill="#FFFBEB" />
    <!-- Right Mechanical Wing Spar & Crimson Ruby Leading Beacon -->
    <path d="M 518 518 L 784 226 L 808 248 L 548 544 Z" fill="url(#euvGoldArmor)" stroke="{COLOR_OBSIDIAN_DEEP}" stroke-width="6" stroke-linejoin="round" />
    <path d="M 786 222 L 908 92 C 916 84 930 98 922 106 L 804 238 Z" fill="url(#euvRubyGem)" stroke="{COLOR_CRIMSON_DEEP}" stroke-width="4" stroke-linejoin="round" />
    <path d="M 906 102 L 802 214" stroke="#FFC2CE" stroke-width="4" stroke-linecap="round" opacity="0.85" />

    <!-- Armored S-Curve Tail, Torso & Hind Legs (Obsidian Core + Imperial Gold Plates) -->
    <path d="M 550 568 C 626 510 714 510 784 568 C 858 628 914 612 918 524 C 920 468 892 430 858 394 C 896 416 946 474 944 548 C 940 644 858 696 762 652 C 708 626 654 606 600 632 Z" fill="url(#euvObsidian)" stroke="{COLOR_OBSIDIAN_DEEP}" stroke-width="6" stroke-linejoin="round" />
    <path d="M 624 548 C 674 528 730 536 776 574 C 836 624 898 620 924 556 C 918 614 868 656 798 642 C 742 630 690 594 636 594 Z" fill="url(#euvGoldArmor)" stroke="{COLOR_OBSIDIAN_DEEP}" stroke-width="4" />

    <!-- Hind Armored Legs with Muscular Sculpted Curves & Mirror-Silver Talons -->
    <path d="M 672 598 C 636 638 624 684 646 724 C 668 756 694 748 708 776 L 686 824 L 722 828 L 750 750 C 756 722 726 694 714 676 C 718 648 730 622 742 604 Z" fill="url(#euvGoldArmor)" stroke="{COLOR_OBSIDIAN_DEEP}" stroke-width="6" stroke-linejoin="round" />
    <path d="M 684 824 L 664 872 L 692 848 L 696 882 L 716 846 L 732 874 L 726 826 Z" fill="url(#euvSilverTalon)" stroke="{COLOR_OBSIDIAN_DEEP}" stroke-width="4" stroke-linejoin="round" />
    <path d="M 784 584 C 774 640 784 696 814 736 C 836 764 844 794 846 838 L 882 838 C 890 782 882 732 854 688 C 844 652 842 616 836 584 Z" fill="url(#euvGoldArmor)" stroke="{COLOR_OBSIDIAN_DEEP}" stroke-width="6" stroke-linejoin="round" />
    <path d="M 842 836 L 826 884 L 852 860 L 864 894 L 876 858 L 896 884 L 884 836 Z" fill="url(#euvSilverTalon)" stroke="{COLOR_OBSIDIAN_DEEP}" stroke-width="4" stroke-linejoin="round" />

    <!-- Powerful Obsidian & Gold Chest / Neck / Shoulder Chassis -->
    <path d="M 412 362 C 378 446 330 518 334 612 C 338 692 414 744 494 726 C 562 710 618 646 618 574 C 618 524 576 488 520 486 L 516 368 Z" fill="url(#euvObsidian)" stroke="{COLOR_OBSIDIAN_DEEP}" stroke-width="7" stroke-linejoin="round" />
    <path d="M 416 368 L 496 368 L 510 478 C 460 512 406 544 374 606 C 350 540 374 446 416 368 Z" fill="url(#euvGoldArmor)" stroke="{COLOR_OBSIDIAN_DEEP}" stroke-width="5" stroke-linejoin="round" />
    <path d="M 346 576 C 380 518 452 482 526 486 C 578 490 610 526 602 586 C 594 646 522 706 446 710 C 376 714 334 646 346 576 Z" fill="url(#euvGoldArmor)" stroke="{COLOR_OBSIDIAN_DEEP}" stroke-width="6" />
    <path d="M 416 616 C 454 584 514 586 558 624 C 534 676 470 704 416 686 Z" fill="url(#euvObsidian)" />

    <!-- Raised Left Forearm & Dynamic Splayed Five-Clawed Silver Talons -->
    <path d="M 348 594 L 240 652 L 166 572 L 194 544 L 258 606 L 342 558 Z" fill="url(#euvGoldArmor)" stroke="{COLOR_OBSIDIAN_DEEP}" stroke-width="6" stroke-linejoin="round" />
    <circle cx="176" cy="560" r="34" fill="url(#euvObsidian)" stroke="{COLOR_ROYAL_GOLD}" stroke-width="5" />
    <path d="M 156 542 L 106 492 L 72 510 L 122 528 L 146 558 Z" fill="url(#euvSilverTalon)" stroke="{COLOR_OBSIDIAN_DEEP}" stroke-width="4" stroke-linejoin="round" />
    <path d="M 176 528 L 156 468 L 116 472 L 154 494 L 166 534 Z" fill="url(#euvSilverTalon)" stroke="{COLOR_OBSIDIAN_DEEP}" stroke-width="4" stroke-linejoin="round" />
    <path d="M 154 576 L 104 608 L 138 624 L 166 592 Z" fill="url(#euvSilverTalon)" stroke="{COLOR_OBSIDIAN_DEEP}" stroke-width="4" stroke-linejoin="round" />

    <!-- Grounded Front-Right Armored Foreleg & Bold Silver Talons -->
    <path d="M 526 578 L 604 694 L 560 806 L 516 792 L 552 702 L 486 608 Z" fill="url(#euvGoldArmor)" stroke="{COLOR_OBSIDIAN_DEEP}" stroke-width="6" stroke-linejoin="round" />
    <path d="M 520 792 L 456 856 L 444 926 L 490 868 L 532 824 Z" fill="url(#euvSilverTalon)" stroke="{COLOR_OBSIDIAN_DEEP}" stroke-width="4" stroke-linejoin="round" />
    <path d="M 538 806 L 520 886 L 518 952 L 554 888 L 562 820 Z" fill="url(#euvSilverTalon)" stroke="{COLOR_OBSIDIAN_DEEP}" stroke-width="4" stroke-linejoin="round" />
    <path d="M 562 802 L 578 874 L 564 934 L 598 870 L 584 804 Z" fill="url(#euvSilverTalon)" stroke="{COLOR_OBSIDIAN_DEEP}" stroke-width="4" stroke-linejoin="round" />

    <!-- Shoulder Pauldron & Vivid Crimson Ruby Shoulder Crystal -->
    <path d="M 484 532 C 524 510 578 524 596 566 C 608 598 594 636 562 648 C 526 658 492 630 482 590 C 474 562 474 542 484 532 Z" fill="url(#euvGoldArmor)" stroke="{COLOR_OBSIDIAN_DEEP}" stroke-width="6" />
    <polygon points="534,554 568,574 576,612 552,626 522,604 518,570" fill="url(#euvRubyGem)" stroke="{COLOR_SOLAR_BRIGHT}" stroke-width="3.5" stroke-linejoin="round" />
    <polygon points="534,562 554,576 544,598 528,582" fill="#FF8FA3" opacity="0.65" />

    <!-- Iconic Central Hexagonal Crimson Ruby Chest Crystal -->
    <polygon points="362,566 392,584 398,632 372,664 342,638 338,592" fill="url(#euvRubyGem)" stroke="{COLOR_SOLAR_BRIGHT}" stroke-width="4.5" stroke-linejoin="round" />
    <polygon points="362,576 382,592 372,624 352,604" fill="#FF8FA3" opacity="0.72" />

    <!-- Solar-Gold Flowing Dragon Mane -->
    <path d="M 454 216 C 510 186 580 188 626 226 C 594 246 576 266 594 282 C 556 292 536 312 556 336 C 518 342 496 356 492 384 C 464 340 448 280 454 216 Z" fill="url(#euvWingSolarR)" stroke="{COLOR_AMBER_GLOW}" stroke-width="4" stroke-linejoin="round" />

    <!-- Long Sinuous Multi-Branched Imperial Gold Antlers with Crimson Veins -->
    <path d="M 402 238 C 392 178 398 134 424 88 C 430 132 426 174 426 226 Z" fill="url(#euvGoldArmor)" stroke="{COLOR_OBSIDIAN_DEEP}" stroke-width="5" stroke-linejoin="round" />
    <path d="M 454 236 C 518 176 588 122 686 56 C 642 122 612 162 560 200 C 608 192 646 174 684 134 C 654 196 604 222 534 226 L 474 256 Z" fill="url(#euvGoldArmor)" stroke="{COLOR_OBSIDIAN_DEEP}" stroke-width="5" stroke-linejoin="round" />
    <path d="M 484 224 C 542 174 602 126 658 84 M 544 206 C 594 202 632 184 662 156" stroke="{COLOR_CRIMSON_BRIGHT}" stroke-width="6" stroke-linecap="round" />

    <!-- Noble Heroic Imperial Dragon Head & Forehead Crimson Gem -->
    <path d="M 358 264 C 378 228 430 212 478 226 L 534 226 L 496 276 C 498 314 476 348 440 364 L 382 372 L 352 346 L 340 314 Z" fill="url(#euvGoldArmor)" stroke="{COLOR_OBSIDIAN_DEEP}" stroke-width="6" stroke-linejoin="round" />
    <path d="M 426 224 L 476 224 L 528 228 L 486 266 L 434 254 Z" fill="url(#euvObsidian)" />
    <!-- Crown Crimson Ruby Forehead Jewel -->
    <polygon points="396,226 420,236 416,262 392,268 378,248" fill="url(#euvRubyGem)" stroke="{COLOR_SOLAR_BRIGHT}" stroke-width="3.5" stroke-linejoin="round" />
    <!-- Regal Snout & Noble Jaw -->
    <path d="M 332 306 C 350 288 386 286 416 302 L 424 334 C 392 346 356 344 334 328 Z" fill="url(#euvGoldArmor)" stroke="{COLOR_OBSIDIAN_DEEP}" stroke-width="5" stroke-linejoin="round" />
    <!-- Piercing Blazing Solar-Gold Eye -->
    <polygon points="398,286 436,274 446,292 412,302" fill="{COLOR_OBSIDIAN_DEEP}" />
    <ellipse cx="424" cy="288" rx="14" ry="8" fill="{COLOR_CRIMSON_RUBY}" />
    <ellipse cx="424" cy="288" rx="9" ry="5" fill="{COLOR_SOLAR_BRIGHT}" />
    <!-- Sweeping Long Golden Whiskers -->
    <path d="M 398 326 C 452 368 518 374 562 328 C 594 294 624 268 666 266" stroke="url(#euvGoldHoriz)" stroke-width="8" stroke-linecap="round" fill="none" />
    <path d="M 342 324 C 314 338 308 370 330 392" stroke="url(#euvGoldHoriz)" stroke-width="7" stroke-linecap="round" fill="none" />
  </g>
</svg>
"""


# Analytical fallback Emperor-Armor Dragon polygon primitives (used only when no master PNG is available).
_LEFT_WING_VERTS: tuple[tuple[float, float], ...] = (
    (-128.0, -66.0),
    (-336.0, -306.0),
    (-434.0, -394.0),
    (-414.0, -240.0),
    (-390.0, -144.0),
    (-326.0, -64.0),
    (-246.0, -18.0),
    (-154.0, -18.0),
)

_RIGHT_WING_VERTS: tuple[tuple[float, float], ...] = (
    (8.0, -2.0),
    (284.0, -286.0),
    (432.0, -406.0),
    (418.0, -236.0),
    (380.0, -128.0),
    (306.0, -44.0),
    (174.0, 22.0),
    (72.0, 36.0),
)

_CHEST_ARMOR_VERTS: tuple[tuple[float, float], ...] = (
    (-100.0, -150.0),
    (4.0, -144.0),
    (106.0, 48.0),
    (82.0, 166.0),
    (-18.0, 214.0),
    (-144.0, 174.0),
    (-178.0, 88.0),
    (-156.0, -34.0),
)

_HEAD_CREST_VERTS: tuple[tuple[float, float], ...] = (
    (-154.0, -248.0),
    (-96.0, -296.0),
    (-14.0, -286.0),
    (22.0, -244.0),
    (-36.0, -162.0),
    (-130.0, -140.0),
    (-178.0, -196.0),
)

_CHEST_RUBY_VERTS: tuple[tuple[float, float], ...] = (
    (-150.0, 54.0),
    (-120.0, 72.0),
    (-114.0, 120.0),
    (-140.0, 152.0),
    (-170.0, 126.0),
    (-174.0, 80.0),
)

_SHOULDER_RUBY_VERTS: tuple[tuple[float, float], ...] = (
    (22.0, 42.0),
    (56.0, 62.0),
    (64.0, 100.0),
    (40.0, 114.0),
    (10.0, 92.0),
    (6.0, 58.0),
)

_FOREHEAD_RUBY_VERTS: tuple[tuple[float, float], ...] = (
    (-116.0, -286.0),
    (-92.0, -276.0),
    (-96.0, -250.0),
    (-120.0, -244.0),
    (-134.0, -264.0),
)


def _sd_segment_scalar(px: float, py: float, ax: float, ay: float, bx: float, by: float) -> float:
    pax, pay = px - ax, py - ay
    bax, bay = bx - ax, by - ay
    denom = bax * bax + bay * bay
    h = max(0.0, min(1.0, (pax * bax + pay * bay) / max(denom, 1e-12)))
    return math.hypot(pax - bax * h, pay - bay * h)


def _sd_poly_scalar(x: float, y: float, verts: Sequence[tuple[float, float]]) -> float:
    n = len(verts)
    d2 = 1e18
    sign = 1.0
    for i in range(n):
        x1, y1 = verts[i]
        x2, y2 = verts[(i + 1) % n]
        ex = x2 - x1
        ey = y2 - y1
        wx = x - x1
        wy = y - y1
        denom = ex * ex + ey * ey
        t = max(0.0, min(1.0, (wx * ex + wy * ey) / max(denom, 1e-12)))
        bx = wx - ex * t
        by = wy - ey * t
        dist2 = bx * bx + by * by
        if dist2 < d2:
            d2 = dist2
        c1 = y >= y1
        c2 = y < y2
        c3 = ex * wy > ey * wx
        if (c1 and c2 and c3) or ((not c1) and (not c2) and (not c3)):
            sign = -sign
    return sign * math.sqrt(d2)


def _render_dragon_numpy(size: int) -> bytes:
    """Render the fallback Emperor-Armor Cybernetic Chinese Dragon icon at `size x size` RGBA with NumPy SDF."""
    import numpy as np

    ss = 2 if size >= 512 else 4
    w = size * ss
    coords = (np.arange(w, dtype=np.float64) + 0.5) * (CANVAS_SIZE / w)
    gx, gy = np.meshgrid(coords, coords)
    px = gx - CANVAS_CENTER
    py = gy - CANVAS_CENTER
    aa = CANVAS_SIZE / w

    def sd_poly(x: np.ndarray, y: np.ndarray, verts: Sequence[tuple[float, float]]) -> np.ndarray:
        count = len(verts)
        d2 = np.full_like(x, 1e18, dtype=np.float64)
        sign = np.ones_like(x, dtype=np.float64)
        for i in range(count):
            x1, y1 = verts[i]
            x2, y2 = verts[(i + 1) % count]
            ex, ey = x2 - x1, y2 - y1
            wx, wy = x - x1, y - y1
            denom = max(ex * ex + ey * ey, 1e-12)
            t = np.clip((wx * ex + wy * ey) / denom, 0.0, 1.0)
            bx, by = wx - ex * t, wy - ey * t
            d2 = np.minimum(d2, bx * bx + by * by)
            c1 = y >= y1
            c2 = y < y2
            c3 = ex * wy > ey * wx
            sign = np.where((c1 & c2 & c3) | ((~c1) & (~c2) & (~c3)), -sign, sign)
        return sign * np.sqrt(d2)

    def sd_seg(
        x: np.ndarray, y: np.ndarray, ax: float, ay: float, bx: float, by: float
    ) -> np.ndarray:
        pax, pay = x - ax, y - ay
        bax, bay = bx - ax, by - ay
        denom = max(bax * bax + bay * bay, 1e-12)
        h = np.clip((pax * bax + pay * bay) / denom, 0.0, 1.0)
        return np.hypot(pax - bax * h, pay - bay * h)

    def cov(d: np.ndarray) -> np.ndarray:
        return np.clip(0.5 - d / (aa * 1.05), 0.0, 1.0)

    def over(dst: np.ndarray, src_rgb: np.ndarray, src_a: np.ndarray) -> np.ndarray:
        da = dst[..., 3:4]
        sa = src_a[..., None]
        out_a = sa + da * (1.0 - sa)
        safe_a = np.maximum(out_a, 1e-8)
        out_rgb = (src_rgb * sa + dst[..., :3] * da * (1.0 - sa)) / safe_a
        out = np.empty_like(dst)
        out[..., :3] = np.where(out_a > 1e-8, out_rgb, 0.0)
        out[..., 3:4] = out_a
        return out

    def lerp_rgb(
        c1: tuple[float, float, float], c2: tuple[float, float, float], t: np.ndarray
    ) -> np.ndarray:
        tc = np.clip(t, 0.0, 1.0)[..., None]
        a = np.array(c1, dtype=np.float64)[None, None, :]
        b = np.array(c2, dtype=np.float64)[None, None, :]
        return a * (1.0 - tc) + b * tc

    canvas = np.zeros((w, w, 4), dtype=np.float64)
    c_solar = _hex_to_rgb01(COLOR_SOLAR_BRIGHT)
    c_gold = _hex_to_rgb01(COLOR_IMPERIAL_GOLD)
    c_royal = _hex_to_rgb01(COLOR_ROYAL_GOLD)
    c_amber = _hex_to_rgb01(COLOR_AMBER_GLOW)
    c_obs = _hex_to_rgb01(COLOR_OBSIDIAN_DEEP)
    c_ruby = _hex_to_rgb01(COLOR_CRIMSON_RUBY)
    c_ruby_hi = _hex_to_rgb01(COLOR_CRIMSON_BRIGHT)
    c_silver = _hex_to_rgb01(COLOR_SILVER_MID)

    # 1. Twin wide solar-gold holographic wings.
    d_wings = np.minimum(sd_poly(px, py, _LEFT_WING_VERTS), sd_poly(px, py, _RIGHT_WING_VERTS))
    wing_t = np.clip((py + 400.0) / 460.0, 0.0, 1.0)
    wing_rgb = lerp_rgb(c_solar, c_amber, wing_t)
    canvas = over(canvas, wing_rgb, cov(d_wings) * 0.94)

    # 2. Crimson ruby wingtip leading-edge beacons.
    d_beacons = np.minimum(
        sd_seg(px, py, -418.0, -418.0, -296.0, -304.0) - 18.0,
        sd_seg(px, py, 404.0, -412.0, 282.0, -284.0) - 18.0,
    )
    canvas = over(canvas, lerp_rgb(c_ruby_hi, c_ruby, wing_t), cov(d_beacons))

    # 3. Obsidian chassis + imperial gold armored body, head, and antlers.
    d_body = np.minimum(sd_poly(px, py, _CHEST_ARMOR_VERTS), sd_poly(px, py, _HEAD_CREST_VERTS))
    d_antlers = np.minimum(
        sd_seg(px, py, -110.0, -274.0, -88.0, -424.0) - 14.0,
        sd_seg(px, py, -58.0, -276.0, 174.0, -456.0) - 16.0,
    )
    d_tail = sd_seg(px, py, 80.0, 110.0, 390.0, 90.0) - 34.0
    d_chassis = np.minimum(np.minimum(d_body, d_antlers), d_tail)
    canvas = over(canvas, np.broadcast_to(np.array(c_obs), (w, w, 3)), cov(d_chassis - 8.0))
    gold_t = np.clip((py + 420.0) / 720.0, 0.0, 1.0)
    canvas = over(canvas, lerp_rgb(c_gold, c_royal, gold_t), cov(d_chassis + 4.0))

    # 4. Splayed mirror-silver talon accents.
    d_claws = np.minimum(
        sd_seg(px, py, -330.0, 40.0, -420.0, -10.0) - 14.0,
        sd_seg(px, py, 20.0, 300.0, 10.0, 430.0) - 16.0,
    )
    canvas = over(canvas, np.broadcast_to(np.array(c_silver), (w, w, 3)), cov(d_claws))

    # 5. Vivid crimson ruby forehead, chest, and shoulder gems.
    d_gems = np.minimum(
        np.minimum(sd_poly(px, py, _CHEST_RUBY_VERTS), sd_poly(px, py, _SHOULDER_RUBY_VERTS)),
        sd_poly(px, py, _FOREHEAD_RUBY_VERTS),
    )
    canvas = over(canvas, lerp_rgb(c_ruby_hi, c_ruby, gold_t), cov(d_gems))

    # Premultiplied-alpha downsample for crisp anti-aliased edges.
    pm = np.empty_like(canvas)
    pm[..., :3] = canvas[..., :3] * canvas[..., 3:4]
    pm[..., 3:4] = canvas[..., 3:4]
    pm = pm.reshape(size, ss, size, ss, 4).mean(axis=(1, 3))
    out = np.zeros((size, size, 4), dtype=np.float64)
    a = pm[..., 3:4]
    out[..., :3] = np.where(a > 1e-8, pm[..., :3] / np.maximum(a, 1e-8), 0.0)
    out[..., 3:4] = a
    return np.clip(np.round(out * 255.0), 0, 255).astype(np.uint8).tobytes()


def _render_dragon_pure_python(size: int) -> bytes:
    """Zero-dependency pure-Python fallback rendering the Emperor-Armor Cybernetic Chinese Dragon."""
    buf = bytearray(size * size * 4)
    scale = CANVAS_SIZE / float(size)
    aa = scale * 1.05
    c_solar = _hex_to_rgb01(COLOR_SOLAR_BRIGHT)
    c_gold = _hex_to_rgb01(COLOR_IMPERIAL_GOLD)
    c_obs = _hex_to_rgb01(COLOR_OBSIDIAN_DEEP)
    c_ruby = _hex_to_rgb01(COLOR_CRIMSON_RUBY)

    def cov_s(d: float) -> float:
        return max(0.0, min(1.0, 0.5 - d / aa))

    def over_s(
        dr: float, dg: float, db: float, da: float, sr: float, sg: float, sb: float, sa: float
    ) -> tuple[float, float, float, float]:
        oa = sa + da * (1.0 - sa)
        if oa <= 1e-8:
            return (0.0, 0.0, 0.0, 0.0)
        inv = 1.0 / oa
        return (
            (sr * sa + dr * da * (1.0 - sa)) * inv,
            (sg * sa + dg * da * (1.0 - sa)) * inv,
            (sb * sa + db * da * (1.0 - sa)) * inv,
            oa,
        )

    for y in range(size):
        py = (y + 0.5) * scale - CANVAS_CENTER
        row = y * size * 4
        for x in range(size):
            px = (x + 0.5) * scale - CANVAS_CENTER
            r, g, b, a = 0.0, 0.0, 0.0, 0.0
            mw = cov_s(min(_sd_poly_scalar(px, py, _LEFT_WING_VERTS), _sd_poly_scalar(px, py, _RIGHT_WING_VERTS)))
            if mw > 0.0:
                r, g, b, a = over_s(r, g, b, a, c_solar[0], c_solar[1], c_solar[2], mw * 0.94)
            mb = cov_s(
                min(
                    _sd_segment_scalar(px, py, -418.0, -418.0, -296.0, -304.0) - 18.0,
                    _sd_segment_scalar(px, py, 404.0, -412.0, 282.0, -284.0) - 18.0,
                )
            )
            if mb > 0.0:
                r, g, b, a = over_s(r, g, b, a, c_ruby[0], c_ruby[1], c_ruby[2], mb)
            d_core = min(
                _sd_poly_scalar(px, py, _CHEST_ARMOR_VERTS),
                _sd_poly_scalar(px, py, _HEAD_CREST_VERTS),
                _sd_segment_scalar(px, py, -58.0, -276.0, 174.0, -456.0) - 16.0,
            )
            mo = cov_s(d_core - 8.0)
            if mo > 0.0:
                r, g, b, a = over_s(r, g, b, a, c_obs[0], c_obs[1], c_obs[2], mo)
            mg = cov_s(d_core + 4.0)
            if mg > 0.0:
                r, g, b, a = over_s(r, g, b, a, c_gold[0], c_gold[1], c_gold[2], mg)
            mr = cov_s(
                min(
                    _sd_poly_scalar(px, py, _CHEST_RUBY_VERTS),
                    _sd_poly_scalar(px, py, _SHOULDER_RUBY_VERTS),
                    _sd_poly_scalar(px, py, _FOREHEAD_RUBY_VERTS),
                )
            )
            if mr > 0.0:
                r, g, b, a = over_s(r, g, b, a, c_ruby[0], c_ruby[1], c_ruby[2], mr)
            if a > 0.0:
                idx = row + x * 4
                buf[idx] = int(max(0, min(255, round(r * 255.0))))
                buf[idx + 1] = int(max(0, min(255, round(g * 255.0))))
                buf[idx + 2] = int(max(0, min(255, round(b * 255.0))))
                buf[idx + 3] = int(max(0, min(255, round(a * 255.0))))
    return bytes(buf)


def render_fallback_icon(size: int) -> bytes:
    """Render the fallback Emperor-Armor Cybernetic Chinese Dragon icon at `size x size` RGBA."""
    try:
        import numpy  # noqa: F401

        return _render_dragon_numpy(size)
    except ImportError:
        return _render_dragon_pure_python(size)


def _premult_lanczos(rgba_img: object, size: int) -> object:
    """Resize a PIL RGBA image using premultiplied-alpha Lanczos to eliminate edge haloing."""
    from PIL import Image

    try:
        import numpy as np

        arr = np.asarray(rgba_img, dtype=np.float32) / 255.0
        pm = np.empty_like(arr)
        pm[..., :3] = arr[..., :3] * arr[..., 3:4]
        pm[..., 3:4] = arr[..., 3:4]
        pm_img = Image.fromarray(np.clip(np.round(pm * 255.0), 0, 255).astype(np.uint8))
        pm_res = pm_img.resize((size, size), Image.Resampling.LANCZOS)
        pr = np.asarray(pm_res, dtype=np.float32) / 255.0
        out = np.zeros_like(pr)
        a = pr[..., 3:4]
        out[..., :3] = np.where(a > 1e-4, np.clip(pr[..., :3] / np.maximum(a, 1e-4), 0.0, 1.0), 0.0)
        out[..., 3:4] = np.where(a > 1.0 / 255.0, a, 0.0)
        return Image.fromarray(np.clip(np.round(out * 255.0), 0, 255).astype(np.uint8))
    except ImportError:
        return rgba_img.resize((size, size), Image.Resampling.LANCZOS)


def _pad_and_resize_rgba_png(
    src_path: str, out_path: str, size: int, normalize_padding: bool = False
) -> None:
    """Resize a transparent RGBA PNG with premultiplied Lanczos resampling and optional optical padding."""
    parent = os.path.dirname(os.path.abspath(out_path))
    if parent:
        os.makedirs(parent, exist_ok=True)
    try:
        from PIL import Image

        with Image.open(src_path) as img:
            rgba = img.convert("RGBA")
            if normalize_padding:
                bbox = rgba.getbbox()
                if bbox is not None:
                    cropped = rgba.crop(bbox)
                    cw, ch = cropped.size
                    side = int(round(max(cw, ch) / SOURCE_FILL_RATIO))
                    side = max(side, max(cw, ch))
                    canvas = Image.new("RGBA", (side, side), (0, 0, 0, 0))
                    dx = (side - cw) // 2
                    dy = (side - ch) // 2
                    canvas.paste(cropped, (dx, dy))
                    rgba = canvas
            resized = _premult_lanczos(rgba, size)
            resized.save(out_path, format="PNG", optimize=True)
            return
    except ImportError:
        pass
    if shutil.which("sips"):
        subprocess.run(
            ["sips", "-z", str(size), str(size), "-s", "format", "png", src_path, "--out", out_path],
            check=True,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )
        return
    shutil.copyfile(src_path, out_path)


def _is_valid_png_file(path: Path) -> bool:
    try:
        with path.open("rb") as f:
            return f.read(8) == b"\x89PNG\r\n\x1a\n"
    except OSError:
        return False


def _icon_cache_dir() -> Path:
    repo_target = Path(__file__).resolve().parents[2] / "target" / "euv-icon-cache"
    try:
        repo_target.mkdir(parents=True, exist_ok=True)
        return repo_target
    except OSError:
        fallback = Path(tempfile.gettempdir()) / "euv-icon-cache"
        fallback.mkdir(parents=True, exist_ok=True)
        return fallback


def _download_remote_logo(url: str) -> Optional[Path]:
    """Download and cache the canonical PNG logo from `url`, returning the cached path or None."""
    digest = hashlib.sha256(url.encode("utf-8")).hexdigest()[:16]
    cache_path = _icon_cache_dir() / f"euv-logo-{digest}.png"
    if _is_valid_png_file(cache_path):
        return cache_path

    tmp_path = cache_path.with_suffix(".tmp")
    try:
        req = urllib.request.Request(
            url,
            headers={
                "User-Agent": "euv-icon-fetcher/1.0",
                "Accept-Encoding": "gzip, deflate",
            },
        )
        with urllib.request.urlopen(req, timeout=20) as resp:
            data = resp.read()
            encoding = (resp.headers.get("Content-Encoding") or "").lower()
            if "gzip" in encoding or data[:2] == b"\x1f\x8b":
                data = gzip.decompress(data)
            elif "deflate" in encoding:
                data = zlib.decompress(data)
        if data[:8] == b"\x89PNG\r\n\x1a\n":
            tmp_path.write_bytes(data)
            os.replace(tmp_path, cache_path)
            return cache_path
    except Exception:
        pass
    finally:
        if tmp_path.exists():
            try:
                tmp_path.unlink()
            except OSError:
                pass

    if shutil.which("curl"):
        try:
            subprocess.run(
                ["curl", "-fsSL", "--compressed", "--max-time", "20", url, "-o", str(tmp_path)],
                check=True,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.DEVNULL,
            )
            if _is_valid_png_file(tmp_path):
                os.replace(tmp_path, cache_path)
                return cache_path
        except Exception:
            if tmp_path.exists():
                try:
                    tmp_path.unlink()
                except OSError:
                    pass
    return None


def export_branding_suite(
    output_dir: Path,
    sizes: Iterable[int] = BRANDING_SIZES,
    source_png: Optional[Path] = None,
    source_url: Optional[str] = None,
) -> None:
    """Export Euv Studio SVG + multi-resolution transparent PNG icons into `output_dir`."""
    output_dir.mkdir(parents=True, exist_ok=True)
    svg_path = output_dir / "euv-studio-logo.svg"
    svg_path.write_text(generate_logo_svg(), encoding="utf-8")

    resolved_source: Optional[Path] = None
    normalize = False
    if source_png is not None and source_png.is_file():
        resolved_source = source_png
        normalize = True
    else:
        url = source_url or os.environ.get("EUV_APP_ICON_URL", DEFAULT_LOGO_URL)
        if url:
            resolved_source = _download_remote_logo(url)
            normalize = True

    if resolved_source is not None:
        for size in sizes:
            png_path = output_dir / f"euv-studio-logo-{size}.png"
            _pad_and_resize_rgba_png(str(resolved_source), str(png_path), size, normalize_padding=normalize)
    else:
        for size in sizes:
            png_path = output_dir / f"euv-studio-logo-{size}.png"
            rgba = render_fallback_icon(size)
            write_png(str(png_path), size, size, rgba)


def _default_export_dir() -> Path:
    return Path(__file__).resolve().parents[2] / "target" / "euv-icon-export"


def main(argv: Optional[Sequence[str]] = None) -> int:
    args_list = list(sys.argv[1:] if argv is None else argv)

    # Support --all / --svg / --source / --url flags while preserving 100% backward compatibility
    # with positional invocations: generate-icon.py <output_png> <size_px> [custom_source_png_or_url]
    if any(arg in ("--all", "--svg", "--output-dir", "--source", "--url", "-h", "--help") for arg in args_list):
        parser = argparse.ArgumentParser(
            description="Generate Euv Studio transparent PNG and SVG app icons from the canonical README logo URL."
        )
        parser.add_argument(
            "--all",
            action="store_true",
            help="Export euv-studio-logo.svg and all official PNG sizes into --output-dir.",
        )
        parser.add_argument(
            "--output-dir",
            type=Path,
            default=_default_export_dir(),
            help="Destination directory for --all exports (default: target/euv-icon-export).",
        )
        parser.add_argument(
            "--source",
            type=Path,
            default=None,
            help="Optional local master high-resolution transparent RGBA PNG source.",
        )
        parser.add_argument(
            "--url",
            type=str,
            default=None,
            help=f"Remote PNG logo URL (default: EUV_APP_ICON_URL or {DEFAULT_LOGO_URL}).",
        )
        parser.add_argument(
            "--svg",
            type=Path,
            default=None,
            help="Write the master 1024x1024 vector SVG to the specified path.",
        )
        parsed = parser.parse_args(args_list)
        if parsed.all:
            export_branding_suite(parsed.output_dir, source_png=parsed.source, source_url=parsed.url)
        if parsed.svg is not None:
            parsed.svg.parent.mkdir(parents=True, exist_ok=True)
            parsed.svg.write_text(generate_logo_svg(), encoding="utf-8")
        if not parsed.all and parsed.svg is None:
            parser.print_help(sys.stderr)
            return 2
        return 0

    if len(args_list) < 2:
        sys.stderr.write(
            "Usage: generate-icon.py <output_png> <size_px> [custom_source_png_or_url]\n"
            "       generate-icon.py --all [--output-dir <dir>] [--source <png>] [--url <url>]\n"
        )
        return 2

    out_path = args_list[0]
    size = max(16, min(2048, int(args_list[1])))
    custom_src = args_list[2] if len(args_list) > 2 and args_list[2] else os.environ.get("EUV_APP_ICON", "")
    if custom_src:
        if os.path.isfile(custom_src):
            _pad_and_resize_rgba_png(custom_src, out_path, size, normalize_padding=False)
            return 0
        if custom_src.startswith(("http://", "https://")):
            downloaded = _download_remote_logo(custom_src)
            if downloaded is not None:
                _pad_and_resize_rgba_png(str(downloaded), out_path, size, normalize_padding=True)
                return 0

    # Download and cache the canonical README logo PNG, falling back gracefully when offline.
    remote_url = os.environ.get("EUV_APP_ICON_URL", DEFAULT_LOGO_URL)
    if remote_url:
        downloaded = _download_remote_logo(remote_url)
        if downloaded is not None:
            _pad_and_resize_rgba_png(str(downloaded), out_path, size, normalize_padding=True)
            return 0

    rgba = render_fallback_icon(size)
    write_png(out_path, size, size, rgba)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
