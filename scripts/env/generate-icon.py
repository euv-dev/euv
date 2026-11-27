#!/usr/bin/env python3
"""Generate the official Euv Studio transparent-background vector SVG and multi-size PNG app icons.

Usage:
    generate-icon.py <output_png> <size_px> [custom_source_png]
    generate-icon.py --all [--output-dir <dir>]
    generate-icon.py --svg <output_svg>
"""

from __future__ import annotations

import argparse
import math
import os
import shutil
import struct
import subprocess
import sys
import zlib
from pathlib import Path
from typing import Iterable, Optional, Sequence

# Official Euv Studio branding export dimensions (square RGBA transparent PNGs).
BRANDING_SIZES: tuple[int, ...] = (1024, 512, 256, 192, 180, 128, 64, 32)

# Canonical 1024x1024 coordinate constants (centered at 512, 512).
CANVAS_SIZE: float = 1024.0
CANVAS_CENTER: float = 512.0
SQUIRCLE_RADIUS: float = 430.0  # Spans 82..942 (~8.0% optical breathing margin)
SQUIRCLE_EXPONENT: float = 5.25  # Continuous Apple-style superellipse

# Optical horizontal centering offset for the asymmetric 'E' silhouette.
MARK_OFFSET_X: float = 3.0

# Brand color palette (shared verbatim between SVG and analytical SDF rasterizer).
COLOR_BG_TOP: str = "#192045"
COLOR_BG_BOTTOM: str = "#070A16"
COLOR_AURA_INDIGO: str = "#4F46E5"
COLOR_AURA_CYAN: str = "#38BDF8"
COLOR_AURA_VIOLET: str = "#6366F1"
COLOR_RIM_LIGHT: str = "#E0F2FE"
COLOR_RIM_BASE: str = "#6366F1"
COLOR_MARK_TOP: str = "#FFFFFF"
COLOR_MARK_BOTTOM: str = "#DDE9FC"
COLOR_SPARK_ICE: str = "#67E8F9"
COLOR_SPARK_SKY: str = "#38BDF8"
COLOR_SPARK_INDIGO: str = "#6366F1"


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


def _superellipse_svg_path(
    cx: float = CANVAS_CENTER,
    cy: float = CANVAS_CENTER,
    radius: float = SQUIRCLE_RADIUS,
    exponent: float = SQUIRCLE_EXPONENT,
    samples_per_quadrant: int = 32,
) -> str:
    """Emit a high-precision closed SVG path tracing |x/r|^n + |y/r|^n = 1."""
    pts: list[tuple[float, float]] = []
    total = samples_per_quadrant * 4
    power = 2.0 / exponent
    for i in range(total):
        theta = (2.0 * math.pi * i) / total
        ct = math.cos(theta)
        st = math.sin(theta)
        x = cx + radius * math.copysign(abs(ct) ** power, ct)
        y = cy + radius * math.copysign(abs(st) ** power, st)
        pts.append((x, y))
    parts = [f"M {pts[0][0]:.2f} {pts[0][1]:.2f}"]
    for x, y in pts[1:]:
        parts.append(f"L {x:.2f} {y:.2f}")
    parts.append("Z")
    return " ".join(parts)


def _rounded_polygon_svg_path(
    verts: Sequence[tuple[float, float]],
    radius: float,
    corner_radii: Optional[Sequence[float]] = None,
) -> str:
    """Build an exact SVG path for a closed polygon filleted by per-corner radius."""
    n = len(verts)
    commands: list[str] = []
    for i in range(n):
        p_prev = verts[(i - 1) % n]
        p_curr = verts[i]
        p_next = verts[(i + 1) % n]
        r = corner_radii[i] if corner_radii is not None else radius
        if r <= 0.0:
            cmd = "M" if i == 0 else "L"
            commands.append(f"{cmd} {p_curr[0]:.2f} {p_curr[1]:.2f}")
            continue

        v1x = p_prev[0] - p_curr[0]
        v1y = p_prev[1] - p_curr[1]
        v2x = p_next[0] - p_curr[0]
        v2y = p_next[1] - p_curr[1]

        l1 = math.hypot(v1x, v1y)
        l2 = math.hypot(v2x, v2y)
        u1x, u1y = v1x / l1, v1y / l1
        u2x, u2y = v2x / l2, v2y / l2

        # Clamp trim distance to at most 45% of each adjacent edge.
        dot = max(-0.9999, min(0.9999, u1x * u2x + u1y * u2y))
        half_angle = math.acos(dot) * 0.5
        tan_half = math.tan(half_angle)
        trim = r / max(tan_half, 1e-4)
        trim = min(trim, l1 * 0.45, l2 * 0.45)

        start_x = p_curr[0] + u1x * trim
        start_y = p_curr[1] + u1y * trim
        end_x = p_curr[0] + u2x * trim
        end_y = p_curr[1] + u2y * trim

        if i == 0:
            commands.append(f"M {start_x:.2f} {start_y:.2f}")
        else:
            commands.append(f"L {start_x:.2f} {start_y:.2f}")
        commands.append(f"Q {p_curr[0]:.2f} {p_curr[1]:.2f} {end_x:.2f} {end_y:.2f}")
    commands.append("Z")
    return " ".join(commands)


def generate_logo_svg() -> str:
    """Return the canonical 1024x1024 transparent-background Euv Studio SVG."""
    ox = CANVAS_CENTER + MARK_OFFSET_X
    oy = CANVAS_CENTER

    squircle_outer = _superellipse_svg_path(CANVAS_CENTER, CANVAS_CENTER, SQUIRCLE_RADIUS, SQUIRCLE_EXPONENT)
    squircle_rim = _superellipse_svg_path(CANVAS_CENTER, CANVAS_CENTER, SQUIRCLE_RADIUS - 3.5, SQUIRCLE_EXPONENT)

    # Top sculpted inverted-L native layer (crisp inner elbow, rounded outer corners).
    top_layer_verts = [
        (ox - 214.0, oy - 236.0),
        (ox + 214.0, oy - 236.0),
        (ox + 214.0, oy - 152.0),
        (ox - 130.0, oy - 152.0),
        (ox - 130.0, oy - 94.0),
        (ox - 214.0, oy - 94.0),
    ]
    top_layer_path = _rounded_polygon_svg_path(
        top_layer_verts, 15.0, corner_radii=(15.0, 15.0, 15.0, 3.0, 15.0, 15.0)
    )

    # Bottom sculpted L native layer (exact vertical mirror of top layer).
    bot_layer_verts = [
        (ox - 214.0, oy + 94.0),
        (ox - 130.0, oy + 94.0),
        (ox - 130.0, oy + 152.0),
        (ox + 214.0, oy + 152.0),
        (ox + 214.0, oy + 236.0),
        (ox - 214.0, oy + 236.0),
    ]
    bot_layer_path = _rounded_polygon_svg_path(
        bot_layer_verts, 15.0, corner_radii=(15.0, 15.0, 3.0, 15.0, 15.0, 15.0)
    )

    # Middle reactive layer bar with forward kinetic chevron tip.
    mid_bar_verts = [
        (ox - 214.0, oy - 42.0),
        (ox + 36.0, oy - 42.0),
        (ox + 78.0, oy),
        (ox + 36.0, oy + 42.0),
        (ox - 214.0, oy + 42.0),
    ]
    mid_bar_path = _rounded_polygon_svg_path(mid_bar_verts, 10.0)

    # Luminous electric cyan-to-indigo reactive signal chevron '>' aligned flush at +214.
    chevron_verts = [
        (ox + 106.0, oy - 42.0),
        (ox + 170.0, oy - 42.0),
        (ox + 214.0, oy),
        (ox + 170.0, oy + 42.0),
        (ox + 106.0, oy + 42.0),
        (ox + 150.0, oy),
    ]
    chevron_path = _rounded_polygon_svg_path(chevron_verts, 6.0)

    return f"""<svg xmlns="http://www.w3.org/2000/svg" width="1024" height="1024" viewBox="0 0 1024 1024" fill="none">
  <title>Euv Studio</title>
  <defs>
    <clipPath id="euvSquircleClip">
      <path d="{squircle_outer}" />
    </clipPath>
    <linearGradient id="euvBgGrad" x1="256" y1="82" x2="768" y2="942" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="{COLOR_BG_TOP}" />
      <stop offset="100%" stop-color="{COLOR_BG_BOTTOM}" />
    </linearGradient>
    <radialGradient id="euvAuraIndigo" cx="617" cy="317" r="490" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="{COLOR_AURA_INDIGO}" stop-opacity="0.32" />
      <stop offset="65%" stop-color="{COLOR_AURA_INDIGO}" stop-opacity="0.08" />
      <stop offset="100%" stop-color="{COLOR_AURA_INDIGO}" stop-opacity="0" />
    </radialGradient>
    <radialGradient id="euvAuraCyan" cx="732" cy="522" r="365" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="{COLOR_AURA_CYAN}" stop-opacity="0.14" />
      <stop offset="100%" stop-color="{COLOR_AURA_CYAN}" stop-opacity="0" />
    </radialGradient>
    <radialGradient id="euvAuraViolet" cx="277" cy="777" r="415" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="{COLOR_AURA_VIOLET}" stop-opacity="0.12" />
      <stop offset="100%" stop-color="{COLOR_AURA_VIOLET}" stop-opacity="0" />
    </radialGradient>
    <linearGradient id="euvTopSheen" x1="512" y1="82" x2="512" y2="560" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="#DCEFFF" stop-opacity="0.08" />
      <stop offset="100%" stop-color="#DCEFFF" stop-opacity="0" />
    </linearGradient>
    <linearGradient id="euvGlassRim" x1="360" y1="82" x2="680" y2="942" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="{COLOR_RIM_LIGHT}" stop-opacity="0.54" />
      <stop offset="48%" stop-color="#A5B4FC" stop-opacity="0.24" />
      <stop offset="100%" stop-color="{COLOR_RIM_BASE}" stop-opacity="0.16" />
    </linearGradient>
    <linearGradient id="euvMarkWhite" x1="515" y1="276" x2="515" y2="748" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="{COLOR_MARK_TOP}" />
      <stop offset="100%" stop-color="{COLOR_MARK_BOTTOM}" />
    </linearGradient>
    <linearGradient id="euvSparkGrad" x1="{ox + 106.0:.1f}" y1="{oy - 42.0:.1f}" x2="{ox + 214.0:.1f}" y2="{oy + 42.0:.1f}" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="{COLOR_SPARK_ICE}" />
      <stop offset="45%" stop-color="{COLOR_SPARK_SKY}" />
      <stop offset="100%" stop-color="{COLOR_SPARK_INDIGO}" />
    </linearGradient>
    <radialGradient id="euvSparkHalo" cx="{ox + 162.0:.1f}" cy="{oy:.1f}" r="96" gradientUnits="userSpaceOnUse">
      <stop offset="0%" stop-color="{COLOR_AURA_CYAN}" stop-opacity="0.22" />
      <stop offset="100%" stop-color="{COLOR_AURA_CYAN}" stop-opacity="0" />
    </radialGradient>
    <filter id="euvMarkShadow" x="200" y="200" width="640" height="640" filterUnits="userSpaceOnUse" color-interpolation-filters="sRGB">
      <feDropShadow dx="0" dy="10" stdDeviation="16" flood-color="#020410" flood-opacity="0.46" />
    </filter>
  </defs>

  <g clip-path="url(#euvSquircleClip)">
    <rect x="82" y="82" width="860" height="860" fill="url(#euvBgGrad)" />
    <rect x="82" y="82" width="860" height="860" fill="url(#euvAuraIndigo)" />
    <rect x="82" y="82" width="860" height="860" fill="url(#euvAuraCyan)" />
    <rect x="82" y="82" width="860" height="860" fill="url(#euvAuraViolet)" />
    <rect x="82" y="82" width="860" height="480" fill="url(#euvTopSheen)" />
    <path d="{squircle_rim}" stroke="url(#euvGlassRim)" stroke-width="4.0" fill="none" />

    <circle cx="{ox + 162.0:.1f}" cy="{oy:.1f}" r="96" fill="url(#euvSparkHalo)" />

    <g filter="url(#euvMarkShadow)">
      <path d="{top_layer_path}" fill="url(#euvMarkWhite)" />
      <path d="{mid_bar_path}" fill="url(#euvMarkWhite)" />
      <path d="{bot_layer_path}" fill="url(#euvMarkWhite)" />
      <path d="{chevron_path}" fill="url(#euvSparkGrad)" />
    </g>
  </g>
</svg>
"""


def _render_icon_numpy(size: int) -> bytes:
    """Render the official Euv Studio transparent RGBA icon using vectorized NumPy SDF."""
    import numpy as np

    # Use 4x4 supersampling for small/medium sizes and 2x2 for 512/1024 (2048px internal grid).
    ss = 2 if size >= 512 else 4
    w = size * ss
    coords = (np.arange(w, dtype=np.float64) + 0.5) * (CANVAS_SIZE / w)
    gx, gy = np.meshgrid(coords, coords)
    px = gx - CANVAS_CENTER
    py = gy - CANVAS_CENTER
    aa = CANVAS_SIZE / w

    def sd_superellipse(x: np.ndarray, y: np.ndarray, r: float, n: float) -> np.ndarray:
        ax = np.abs(x) / r
        ay = np.abs(y) / r
        eps = 1e-12
        p = ax**n + ay**n
        f = (p + eps) ** (1.0 / n) - 1.0
        g_x = (ax + eps) ** (n - 1.0)
        g_y = (ay + eps) ** (n - 1.0)
        denom = np.maximum(1e-6, (p + eps) ** (1.0 - 1.0 / n))
        grad_len = np.hypot(g_x, g_y) / denom
        return (f / np.maximum(0.5, grad_len)) * r

    def sd_round_box(
        x: np.ndarray, y: np.ndarray, cx: float, cy: float, hx: float, hy: float, rad: float
    ) -> np.ndarray:
        qx = np.abs(x - cx) - (hx - rad)
        qy = np.abs(y - cy) - (hy - rad)
        out = np.hypot(np.maximum(qx, 0.0), np.maximum(qy, 0.0))
        inn = np.minimum(np.maximum(qx, qy), 0.0)
        return out + inn - rad

    def sd_rounded_polygon(
        x: np.ndarray, y: np.ndarray, verts: Sequence[tuple[float, float]], rad: float
    ) -> np.ndarray:
        count = len(verts)
        d2 = np.full_like(x, 1e18, dtype=np.float64)
        sign = np.ones_like(x, dtype=np.float64)
        for i in range(count):
            x1, y1 = verts[i]
            x2, y2 = verts[(i + 1) % count]
            ex = x2 - x1
            ey = y2 - y1
            wx = x - x1
            wy = y - y1
            denom = ex * ex + ey * ey
            t = np.clip((wx * ex + wy * ey) / denom, 0.0, 1.0)
            bx = wx - ex * t
            by = wy - ey * t
            d2 = np.minimum(d2, bx * bx + by * by)
            c1 = y >= y1
            c2 = y < y2
            c3 = ex * wy > ey * wx
            cond = (c1 & c2 & c3) | ((~c1) & (~c2) & (~c3))
            sign = np.where(cond, -sign, sign)
        return sign * np.sqrt(d2) - rad

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

    def lerp_rgb(c1: tuple[float, float, float], c2: tuple[float, float, float], t: np.ndarray) -> np.ndarray:
        tc = np.clip(t, 0.0, 1.0)[..., None]
        a = np.array(c1, dtype=np.float64)[None, None, :]
        b = np.array(c2, dtype=np.float64)[None, None, :]
        return a * (1.0 - tc) + b * tc

    canvas = np.zeros((w, w, 4), dtype=np.float64)

    # 1. Precision continuous superellipse body (strictly 0 alpha outside squircle).
    d_body = sd_superellipse(px, py, SQUIRCLE_RADIUS, SQUIRCLE_EXPONENT)
    mask_body = cov(d_body)

    # Deep anodized obsidian-indigo architectural gradient surface.
    t_v = np.clip((py + SQUIRCLE_RADIUS) / (2.0 * SQUIRCLE_RADIUS), 0.0, 1.0)
    t_d = np.clip((px * 0.42 + py * 0.91 + 455.0) / 910.0, 0.0, 1.0)
    bg_rgb = lerp_rgb(_hex_to_rgb01(COLOR_BG_TOP), _hex_to_rgb01(COLOR_BG_BOTTOM), t_d**0.96)

    # Broad studio indigo aura (upper-right).
    d_aura = np.hypot((px - 105.0) / 530.0, (py + 195.0) / 480.0)
    aura = np.exp(-(d_aura**2) * 1.65) * 0.31
    bg_rgb = bg_rgb + np.array(_hex_to_rgb01(COLOR_AURA_INDIGO))[None, None, :] * aura[..., None]

    # Refined cyan-cobalt atmospheric sheen on right half.
    d_cyan = np.hypot((px - 220.0) / 410.0, (py - 10.0) / 360.0)
    cyan_haze = np.exp(-(d_cyan**2) * 2.0) * 0.13
    bg_rgb = bg_rgb + np.array(_hex_to_rgb01(COLOR_AURA_CYAN))[None, None, :] * cyan_haze[..., None]

    # Subtle bottom-left violet counter-depth.
    d_vio = np.hypot((px + 235.0) / 450.0, (py - 265.0) / 420.0)
    vio_haze = np.exp(-(d_vio**2) * 1.85) * 0.11
    bg_rgb = bg_rgb + np.array(_hex_to_rgb01(COLOR_AURA_VIOLET))[None, None, :] * vio_haze[..., None]

    # Delicate top studio glass highlight.
    top_light = np.clip(1.0 - t_v * 1.72, 0.0, 1.0) ** 2.5 * 0.07
    bg_rgb = bg_rgb + np.array([0.86, 0.94, 1.0])[None, None, :] * top_light[..., None]
    bg_rgb = np.clip(bg_rgb, 0.0, 1.0)
    canvas = over(canvas, bg_rgb, mask_body)

    # 2. Anodized glass top-edge specular rim highlight inset 1.5px inside the crisp dark outer edge.
    rim_band = cov(d_body + 1.5) * (1.0 - cov(d_body + 5.5)) * mask_body
    rim_t = np.clip((-py * 0.88 - px * 0.34 + 380.0) / 760.0, 0.0, 1.0)
    rim_rgb = lerp_rgb(_hex_to_rgb01(COLOR_RIM_BASE), _hex_to_rgb01(COLOR_RIM_LIGHT), rim_t)
    rim_alpha = rim_band * (0.17 + 0.39 * (rim_t**1.4))
    canvas = over(canvas, rim_rgb, rim_alpha)

    inner_bevel = cov(d_body + 5.5) * (1.0 - cov(d_body + 9.0))
    canvas = over(
        canvas,
        np.broadcast_to(np.array([0.02, 0.025, 0.07]), (w, w, 3)),
        inner_bevel * 0.24 * mask_body,
    )

    # 3. Sculpted 3-layer native 'E' monogram + kinetic reactive chevron spark.
    mx = px - MARK_OFFSET_X
    my = py

    d_top_bar = sd_round_box(mx, my, 0.0, -194.0, 214.0, 42.0, 15.0)
    d_top_leg = sd_round_box(mx, my, -172.0, -165.0, 42.0, 71.0, 15.0)
    d_top = np.minimum(d_top_bar, d_top_leg)

    d_bot_bar = sd_round_box(mx, my, 0.0, 194.0, 214.0, 42.0, 15.0)
    d_bot_leg = sd_round_box(mx, my, -172.0, 165.0, 42.0, 71.0, 15.0)
    d_bot = np.minimum(d_bot_bar, d_bot_leg)

    r_m = 7.0
    mid_verts = (
        (-214.0 + r_m, -42.0 + r_m),
        (36.0, -42.0 + r_m),
        (78.0 - r_m, 0.0),
        (36.0, 42.0 - r_m),
        (-214.0 + r_m, 42.0 - r_m),
    )
    d_mid = sd_rounded_polygon(mx, my, mid_verts, r_m)
    d_white = np.minimum(np.minimum(d_top, d_bot), d_mid)

    # Razor-sharp geometric kinetic chevron '>' built from two crisp diagonal arms.
    r_c = 5.0
    arm_top = (
        (106.0, -42.0 + r_c),
        (170.0, -42.0 + r_c),
        (214.0 - r_c, 0.0),
        (150.0, 0.0),
    )
    arm_bot = (
        (150.0, 0.0),
        (214.0 - r_c, 0.0),
        (170.0, 42.0 - r_c),
        (106.0, 42.0 - r_c),
    )
    d_spark = np.minimum(
        sd_rounded_polygon(mx, my, arm_top, r_c),
        sd_rounded_polygon(mx, my, arm_bot, r_c),
    )

    # Restrained interior elevation shadow strictly clipped inside squircle.
    d_mark_all = np.minimum(d_white, d_spark)
    shadow_alpha = (
        np.exp(-(np.maximum(d_mark_all + 6.0, 0.0) ** 2) / (2.0 * 26.0**2)) * 0.42 * mask_body
    )
    shadow_bias = np.clip((my + 260.0) / 480.0, 0.28, 1.0)
    canvas = over(
        canvas,
        np.broadcast_to(np.array([0.01, 0.015, 0.05]), (w, w, 3)),
        shadow_alpha * shadow_bias,
    )

    # Restrained cyan luminescence behind the kinetic spark.
    spark_glow = np.exp(-(np.maximum(d_spark, 0.0) ** 2) / (2.0 * 54.0**2)) * 0.21 * mask_body
    canvas = over(
        canvas,
        np.broadcast_to(np.array(_hex_to_rgb01(COLOR_AURA_CYAN)), (w, w, 3)),
        spark_glow,
    )

    # Composite crisp titanium-white 3-layer 'E'.
    t_white = np.clip((my + 240.0) / 480.0, 0.0, 1.0)
    white_rgb = lerp_rgb(_hex_to_rgb01(COLOR_MARK_TOP), _hex_to_rgb01(COLOR_MARK_BOTTOM), t_white)
    canvas = over(canvas, white_rgb, cov(d_white) * mask_body)

    # Composite luminous ice-cyan -> sky -> electric-indigo kinetic chevron spark.
    t_spark = np.clip((mx - 104.0) / 112.0 * 0.55 + (my + 42.0) / 84.0 * 0.45, 0.0, 1.0)
    c_ice = _hex_to_rgb01(COLOR_SPARK_ICE)
    c_sky = _hex_to_rgb01(COLOR_SPARK_SKY)
    c_ind = _hex_to_rgb01(COLOR_SPARK_INDIGO)
    spark_rgb = np.where(
        t_spark[..., None] < 0.45,
        lerp_rgb(c_ice, c_sky, t_spark / 0.45),
        lerp_rgb(c_sky, c_ind, (t_spark - 0.45) / 0.55),
    )
    canvas = over(canvas, spark_rgb, cov(d_spark) * mask_body)

    # Premultiplied-alpha downsample for razor-sharp edge anti-aliasing.
    pm = np.empty_like(canvas)
    pm[..., :3] = canvas[..., :3] * canvas[..., 3:4]
    pm[..., 3:4] = canvas[..., 3:4]
    pm = pm.reshape(size, ss, size, ss, 4).mean(axis=(1, 3))
    out = np.zeros((size, size, 4), dtype=np.float64)
    a = pm[..., 3:4]
    out[..., :3] = np.where(a > 1e-8, pm[..., :3] / np.maximum(a, 1e-8), 0.0)
    out[..., 3:4] = a
    u8 = np.clip(np.round(out * 255.0), 0, 255).astype(np.uint8)
    return u8.tobytes()


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
        t = max(0.0, min(1.0, (wx * ex + wy * ey) / denom))
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


def _sd_round_box_scalar(
    x: float, y: float, cx: float, cy: float, hx: float, hy: float, rad: float
) -> float:
    qx = abs(x - cx) - (hx - rad)
    qy = abs(y - cy) - (hy - rad)
    return math.hypot(max(qx, 0.0), max(qy, 0.0)) + min(max(qx, qy), 0.0) - rad


def _render_icon_pure_python(size: int) -> bytes:
    """Zero-dependency pure-Python fallback renderer matching the exact SDF specification."""
    buf = bytearray(size * size * 4)
    scale = CANVAS_SIZE / float(size)
    aa = scale * 1.05
    c_top = _hex_to_rgb01(COLOR_BG_TOP)
    c_bot = _hex_to_rgb01(COLOR_BG_BOTTOM)
    c_aura = _hex_to_rgb01(COLOR_AURA_INDIGO)
    c_cyan = _hex_to_rgb01(COLOR_AURA_CYAN)
    c_vio = _hex_to_rgb01(COLOR_AURA_VIOLET)
    c_rim_b = _hex_to_rgb01(COLOR_RIM_BASE)
    c_rim_l = _hex_to_rgb01(COLOR_RIM_LIGHT)
    c_wt = _hex_to_rgb01(COLOR_MARK_TOP)
    c_wb = _hex_to_rgb01(COLOR_MARK_BOTTOM)
    c_si = _hex_to_rgb01(COLOR_SPARK_ICE)
    c_ss = _hex_to_rgb01(COLOR_SPARK_SKY)
    c_sd = _hex_to_rgb01(COLOR_SPARK_INDIGO)

    r_m = 7.0
    mid_verts = (
        (-214.0 + r_m, -42.0 + r_m),
        (36.0, -42.0 + r_m),
        (78.0 - r_m, 0.0),
        (36.0, 42.0 - r_m),
        (-214.0 + r_m, 42.0 - r_m),
    )
    r_c = 5.0
    arm_top = (
        (106.0, -42.0 + r_c),
        (170.0, -42.0 + r_c),
        (214.0 - r_c, 0.0),
        (150.0, 0.0),
    )
    arm_bot = (
        (150.0, 0.0),
        (214.0 - r_c, 0.0),
        (170.0, 42.0 - r_c),
        (106.0, 42.0 - r_c),
    )

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

    n = SQUIRCLE_EXPONENT
    inv_n = 1.0 / n
    r_outer = SQUIRCLE_RADIUS

    for y in range(size):
        py = (y + 0.5) * scale - CANVAS_CENTER
        row = y * size * 4
        ay = abs(py) / r_outer
        ay_n = ay**n
        for x in range(size):
            px = (x + 0.5) * scale - CANVAS_CENTER
            ax = abs(px) / r_outer
            p = ax**n + ay_n
            f = (p + 1e-12) ** inv_n - 1.0
            gx = (ax + 1e-12) ** (n - 1.0)
            gy = (ay + 1e-12) ** (n - 1.0)
            denom = max(1e-6, (p + 1e-12) ** (1.0 - inv_n))
            d_body = (f / max(0.5, math.hypot(gx, gy) / denom)) * r_outer
            mask_body = cov_s(d_body)
            if mask_body <= 0.0:
                continue

            t_v = max(0.0, min(1.0, (py + r_outer) / (2.0 * r_outer)))
            t_d = max(0.0, min(1.0, (px * 0.42 + py * 0.91 + 455.0) / 910.0)) ** 0.96
            br = c_top[0] * (1.0 - t_d) + c_bot[0] * t_d
            bg = c_top[1] * (1.0 - t_d) + c_bot[1] * t_d
            bb = c_top[2] * (1.0 - t_d) + c_bot[2] * t_d

            d_aura = math.hypot((px - 105.0) / 530.0, (py + 195.0) / 480.0)
            aura = math.exp(-(d_aura * d_aura) * 1.65) * 0.31
            d_c = math.hypot((px - 220.0) / 410.0, (py - 10.0) / 360.0)
            ch = math.exp(-(d_c * d_c) * 2.0) * 0.13
            d_v = math.hypot((px + 235.0) / 450.0, (py - 265.0) / 420.0)
            vh = math.exp(-(d_v * d_v) * 1.85) * 0.11
            tl = (max(0.0, min(1.0, 1.0 - t_v * 1.72)) ** 2.5) * 0.07

            br = min(1.0, br + c_aura[0] * aura + c_cyan[0] * ch + c_vio[0] * vh + 0.86 * tl)
            bg = min(1.0, bg + c_aura[1] * aura + c_cyan[1] * ch + c_vio[1] * vh + 0.94 * tl)
            bb = min(1.0, bb + c_aura[2] * aura + c_cyan[2] * ch + c_vio[2] * vh + 1.00 * tl)
            r, g, b, a = br, bg, bb, mask_body

            rim_band = cov_s(d_body + 1.5) * (1.0 - cov_s(d_body + 5.5)) * mask_body
            if rim_band > 0.0:
                rt = max(0.0, min(1.0, (-py * 0.88 - px * 0.34 + 380.0) / 760.0))
                rr = c_rim_b[0] * (1.0 - rt) + c_rim_l[0] * rt
                rg = c_rim_b[1] * (1.0 - rt) + c_rim_l[1] * rt
                rb = c_rim_b[2] * (1.0 - rt) + c_rim_l[2] * rt
                r, g, b, a = over_s(r, g, b, a, rr, rg, rb, rim_band * (0.17 + 0.39 * (rt**1.4)))

            mx = px - MARK_OFFSET_X
            my = py
            d_top = min(
                _sd_round_box_scalar(mx, my, 0.0, -194.0, 214.0, 42.0, 15.0),
                _sd_round_box_scalar(mx, my, -172.0, -165.0, 42.0, 71.0, 15.0),
            )
            d_bot = min(
                _sd_round_box_scalar(mx, my, 0.0, 194.0, 214.0, 42.0, 15.0),
                _sd_round_box_scalar(mx, my, -172.0, 165.0, 42.0, 71.0, 15.0),
            )
            d_mid = _sd_poly_scalar(mx, my, mid_verts) - r_m
            d_white = min(d_top, d_bot, d_mid)
            d_spark = min(
                _sd_poly_scalar(mx, my, arm_top) - r_c,
                _sd_poly_scalar(mx, my, arm_bot) - r_c,
            )

            d_all = min(d_white, d_spark)
            if d_all < 90.0:
                sa = math.exp(-(max(d_all + 6.0, 0.0) ** 2) / (2.0 * 26.0 * 26.0)) * 0.42 * mask_body
                sb = max(0.28, min(1.0, (my + 260.0) / 480.0))
                r, g, b, a = over_s(r, g, b, a, 0.01, 0.015, 0.05, sa * sb)

            if d_spark < 160.0:
                sg_a = math.exp(-(max(d_spark, 0.0) ** 2) / (2.0 * 54.0 * 54.0)) * 0.21 * mask_body
                r, g, b, a = over_s(r, g, b, a, c_cyan[0], c_cyan[1], c_cyan[2], sg_a)

            cw = cov_s(d_white) * mask_body
            if cw > 0.0:
                tw = max(0.0, min(1.0, (my + 240.0) / 480.0))
                wr = c_wt[0] * (1.0 - tw) + c_wb[0] * tw
                wg = c_wt[1] * (1.0 - tw) + c_wb[1] * tw
                wb = c_wt[2] * (1.0 - tw) + c_wb[2] * tw
                r, g, b, a = over_s(r, g, b, a, wr, wg, wb, cw)

            cs = cov_s(d_spark) * mask_body
            if cs > 0.0:
                ts = max(0.0, min(1.0, (mx - 104.0) / 112.0 * 0.55 + (my + 42.0) / 84.0 * 0.45))
                if ts < 0.45:
                    k = ts / 0.45
                    sr = c_si[0] * (1.0 - k) + c_ss[0] * k
                    sg = c_si[1] * (1.0 - k) + c_ss[1] * k
                    sb = c_si[2] * (1.0 - k) + c_ss[2] * k
                else:
                    k = (ts - 0.45) / 0.55
                    sr = c_ss[0] * (1.0 - k) + c_sd[0] * k
                    sg = c_ss[1] * (1.0 - k) + c_sd[1] * k
                    sb = c_ss[2] * (1.0 - k) + c_sd[2] * k
                r, g, b, a = over_s(r, g, b, a, sr, sg, sb, cs)

            idx = row + x * 4
            buf[idx] = int(max(0, min(255, round(r * 255.0))))
            buf[idx + 1] = int(max(0, min(255, round(g * 255.0))))
            buf[idx + 2] = int(max(0, min(255, round(b * 255.0))))
            buf[idx + 3] = int(max(0, min(255, round(a * 255.0))))
    return bytes(buf)


def render_default_icon(size: int) -> bytes:
    """Render the official transparent-background Euv Studio icon at `size x size` RGBA."""
    try:
        import numpy  # noqa: F401

        return _render_icon_numpy(size)
    except ImportError:
        return _render_icon_pure_python(size)


def _resize_custom_icon(src_path: str, out_path: str, size: int) -> None:
    """Resize a user-supplied custom source PNG preserving RGBA transparency."""
    parent = os.path.dirname(os.path.abspath(out_path))
    if parent:
        os.makedirs(parent, exist_ok=True)
    try:
        from PIL import Image

        with Image.open(src_path) as img:
            rgba = img.convert("RGBA").resize((size, size), Image.Resampling.LANCZOS)
            rgba.save(out_path, format="PNG", optimize=True)
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


def export_branding_suite(output_dir: Path, sizes: Iterable[int] = BRANDING_SIZES) -> None:
    """Regenerate the complete Euv Studio SVG + multi-resolution PNG branding suite."""
    output_dir.mkdir(parents=True, exist_ok=True)
    svg_path = output_dir / "euv-studio-logo.svg"
    svg_path.write_text(generate_logo_svg(), encoding="utf-8")
    for size in sizes:
        png_path = output_dir / f"euv-studio-logo-{size}.png"
        rgba = render_default_icon(size)
        write_png(str(png_path), size, size, rgba)


def _default_branding_dir() -> Path:
    return Path(__file__).resolve().parents[2] / "assets" / "branding"


def main(argv: Optional[Sequence[str]] = None) -> int:
    args_list = list(sys.argv[1:] if argv is None else argv)

    # Support new --all / --svg flags while preserving 100% backward compatibility
    # with positional invocations: generate-icon.py <output_png> <size_px> [custom_source_png]
    if any(arg in ("--all", "--svg", "--output-dir", "-h", "--help") for arg in args_list):
        parser = argparse.ArgumentParser(
            description="Generate Euv Studio transparent-background vector SVG and PNG app icons."
        )
        parser.add_argument(
            "--all",
            action="store_true",
            help="Regenerate euv-studio-logo.svg and all official PNG sizes in assets/branding/.",
        )
        parser.add_argument(
            "--output-dir",
            type=Path,
            default=_default_branding_dir(),
            help="Destination directory for --all exports (default: assets/branding).",
        )
        parser.add_argument(
            "--svg",
            type=Path,
            default=None,
            help="Write the master 1024x1024 vector SVG to the specified path.",
        )
        parsed = parser.parse_args(args_list)
        if parsed.all:
            export_branding_suite(parsed.output_dir)
        if parsed.svg is not None:
            parsed.svg.parent.mkdir(parents=True, exist_ok=True)
            parsed.svg.write_text(generate_logo_svg(), encoding="utf-8")
        if not parsed.all and parsed.svg is None:
            parser.print_help(sys.stderr)
            return 2
        return 0

    if len(args_list) < 2:
        sys.stderr.write(
            "Usage: generate-icon.py <output_png> <size_px> [custom_source_png]\n"
            "       generate-icon.py --all [--output-dir <dir>]\n"
        )
        return 2

    out_path = args_list[0]
    size = max(16, min(2048, int(args_list[1])))
    custom_src = args_list[2] if len(args_list) > 2 and args_list[2] else os.environ.get("EUV_APP_ICON", "")
    if custom_src and os.path.isfile(custom_src):
        _resize_custom_icon(custom_src, out_path, size)
        return 0

    # Fast zero-dependency fallback when numpy is absent and checked-in 1024 master PNG exists.
    master_png = _default_branding_dir() / "euv-studio-logo-1024.png"
    try:
        import numpy  # noqa: F401

        rgba = _render_icon_numpy(size)
        write_png(out_path, size, size, rgba)
    except ImportError:
        if master_png.is_file() and size <= 1024:
            _resize_custom_icon(str(master_png), out_path, size)
        else:
            rgba = _render_icon_pure_python(size)
            write_png(out_path, size, size, rgba)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
