# Euv Studio Branding Assets

Official high-resolution, transparent-background icon suite for **Euv Studio**, communicating `euv`'s zero-WebView, zero-VDOM, direct retained scene-graph layers and fine-grained reactive signal precision.

## Files (`assets/branding/`)

| File                       | Format | Dimensions  | Primary Use                       |
| :------------------------- | :----- | :---------- | :-------------------------------- |
| `euv-studio-logo.svg`      | SVG    | `1024×1024` | Master vector source              |
| `euv-studio-logo-1024.png` | RGBA   | `1024×1024` | App Store, macOS, high-res source |
| `euv-studio-logo-512.png`  | RGBA   | `512×512`   | Desktop bundles, docs, marketing  |
| `euv-studio-logo-256.png`  | RGBA   | `256×256`   | Medium previews, desktop icons    |
| `euv-studio-logo-192.png`  | RGBA   | `192×192`   | Android `xxxhdpi` / PWA manifest  |
| `euv-studio-logo-180.png`  | RGBA   | `180×180`   | iOS `@3x` / Apple Touch icon      |
| `euv-studio-logo-128.png`  | RGBA   | `128×128`   | Desktop window / Dock icon        |
| `euv-studio-logo-64.png`   | RGBA   | `64×64`     | Compact toolbar / small badge     |
| `euv-studio-logo-32.png`   | RGBA   | `32×32`     | Browser favicon / tray icon       |

## Visual System

- **Silhouette**: Continuous Apple-style superellipse (`n = 5.25`, `860×860` centered inside `1024×1024` for an `8%` transparent outer margin; `alpha = 0` at all four corners).
- **Surface**: Deep obsidian-indigo diagonal gradient (`#192045` → `#070A16`) with broad studio indigo (`#4F46E5`), subtle right cyan (`#38BDF8`), and bottom-left violet (`#6366F1`) ambient depth, edged with a top-lit anodized glass rim (`#E0F2FE` → `#6366F1`).
- **Monogram**: Precision 3-layer luminous white `E` (`#FFFFFF` → `#DDE9FC`) framed by crisp `52px` structural channels, paired on the middle layer with a forward kinetic chevron spark (`#67E8F9` → `#38BDF8` → `#6366F1`).

## Regeneration

Regenerate the full vector and raster suite deterministically:

```bash
python3 scripts/env/generate-icon.py --all
```

Generate a single custom-sized icon (used by `scripts/build/build-ios-sim.sh` and `scripts/build/build-android-apk.sh`):

```bash
python3 scripts/env/generate-icon.py <output_png> <size_px> [custom_source_png]
```
