import json
import os
import sys
import struct
import io
import argparse


def load_config():
    config_path = os.path.join(os.path.dirname(__file__), "..", "app.config.json")
    config_path = os.path.abspath(config_path)
    with open(config_path, "r") as f:
        return json.load(f)


def main():
    parser = argparse.ArgumentParser(description="Generate app icons from source image")
    parser.add_argument(
        "--source", "-s", help="Source icon image path (overrides app.config.json)"
    )
    args = parser.parse_args()

    config = load_config()
    root_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))

    # Determine source image
    source = args.source or os.path.join(root_dir, config["icons"]["source"])
    if not os.path.exists(source):
        print(f"[ERROR] Source icon not found: {source}")
        sys.exit(1)

    try:
        from PIL import Image
    except ImportError:
        print("[ERROR] Pillow not installed. Run: pip install Pillow")
        sys.exit(1)

    im = Image.open(source).convert("RGBA")
    print(f"[INFO] Source: {source} ({im.size[0]}x{im.size[1]})")

    # ===== Tauri Desktop Icons =====
    tauri_icons_dir = os.path.join(root_dir, "src-tauri", "icons")
    os.makedirs(tauri_icons_dir, exist_ok=True)

    png_sizes = {
        "32x32.png": (32, 32),
        "128x128.png": (128, 128),
        "128x128@2x.png": (256, 256),
        "icon.png": (512, 512),
        "Square30x30Logo.png": (30, 30),
        "Square44x44Logo.png": (44, 44),
        "Square71x71Logo.png": (71, 71),
        "Square89x89Logo.png": (89, 89),
        "Square107x107Logo.png": (107, 107),
        "Square142x142Logo.png": (142, 142),
        "Square150x150Logo.png": (150, 150),
        "Square284x284Logo.png": (284, 284),
        "Square310x310Logo.png": (310, 310),
        "StoreLogo.png": (50, 50),
    }

    print("\n[STEP] Generating Tauri desktop icons...")
    for name, (w, h) in png_sizes.items():
        resized = im.resize((w, h), Image.LANCZOS)
        resized.save(os.path.join(tauri_icons_dir, name), "PNG")
        print(f"  {name} ({w}x{h})")

    # Windows .ico
    ico_sizes = [16, 24, 32, 48, 64, 128, 256]
    ico_images = [im.resize((s, s), Image.LANCZOS) for s in ico_sizes]
    ico_path = os.path.join(tauri_icons_dir, "icon.ico")
    ico_images[0].save(
        ico_path,
        format="ICO",
        sizes=[(s, s) for s in ico_sizes],
        append_images=ico_images[1:],
    )
    print(f"  icon.ico (multi-size)")

    # macOS .icns
    def icns_png_type(size_px, scale=1):
        table = {
            (16, 1): "icp4",
            (16, 2): "ic11",
            (32, 1): "icp5",
            (32, 2): "ic12",
            (64, 1): "icp6",
            (64, 2): "ic13",
            (128, 1): "ic07",
            (128, 2): "ic14",
            (256, 1): "ic08",
            (256, 2): "ic1a",
            (512, 1): "ic09",
            (512, 2): "ic1b",
            (1024, 1): "ic10",
        }
        return table.get((size_px, scale))

    def img_to_png_bytes(img):
        buf = io.BytesIO()
        img.save(buf, format="PNG")
        return buf.getvalue()

    icns_entries = []
    icns_specs = [
        (16, 1),
        (16, 2),
        (32, 1),
        (32, 2),
        (64, 1),
        (64, 2),
        (128, 1),
        (128, 2),
        (256, 1),
        (256, 2),
        (512, 1),
        (512, 2),
        (1024, 1),
    ]

    for px, scale in icns_specs:
        actual = px * scale
        code = icns_png_type(px, scale)
        if code is None:
            continue
        resized = im.resize((actual, actual), Image.LANCZOS)
        png_data = img_to_png_bytes(resized)
        icns_entries.append((code, png_data))

    with open(os.path.join(tauri_icons_dir, "icon.icns"), "wb") as f:
        data_blocks = b""
        for code, png_data in icns_entries:
            block_type = code.encode("ascii")
            block_len = 8 + len(png_data)
            data_blocks += struct.pack(">4sI", block_type, block_len) + png_data
        total_len = 8 + len(data_blocks)
        f.write(struct.pack(">4sI", b"icns", total_len))
        f.write(data_blocks)
    print(f"  icon.icns ({total_len} bytes)")

    # ===== Android Icons =====
    android_res_dir = os.path.join(
        root_dir, "src-tauri", "gen", "android", "app", "src", "main", "res"
    )

    android_sizes = {
        "mipmap-mdpi": {
            "ic_launcher.png": 48,
            "ic_launcher_round.png": 48,
            "ic_launcher_foreground.png": 108,
        },
        "mipmap-hdpi": {
            "ic_launcher.png": 72,
            "ic_launcher_round.png": 72,
            "ic_launcher_foreground.png": 162,
        },
        "mipmap-xhdpi": {
            "ic_launcher.png": 96,
            "ic_launcher_round.png": 96,
            "ic_launcher_foreground.png": 216,
        },
        "mipmap-xxhdpi": {
            "ic_launcher.png": 144,
            "ic_launcher_round.png": 144,
            "ic_launcher_foreground.png": 324,
        },
        "mipmap-xxxhdpi": {
            "ic_launcher.png": 192,
            "ic_launcher_round.png": 192,
            "ic_launcher_foreground.png": 432,
        },
    }

    print("\n[STEP] Generating Android icons...")
    for folder, icons in android_sizes.items():
        folder_path = os.path.join(android_res_dir, folder)
        os.makedirs(folder_path, exist_ok=True)
        for name, size in icons.items():
            resized = im.resize((size, size), Image.LANCZOS)
            resized.save(os.path.join(folder_path, name), "PNG")
            print(f"  {folder}/{name} ({size}x{size})")

    # Play Store icon
    play_icon = im.resize((512, 512), Image.LANCZOS)
    play_icon.save(
        os.path.join(android_res_dir, "mipmap-xxxhdpi", "ic_launcher_web.png"), "PNG"
    )
    print(f"  mipmap-xxxhdpi/ic_launcher_web.png (512x512)")

    # ===== iOS Icons (if iOS target is configured) =====
    if "ios" in config.get("build", {}).get("targets", []):
        ios_assets_dir = os.path.join(
            root_dir,
            "src-tauri",
            "gen",
            "apple",
            "Assets.xcassets",
            "AppIcon.appiconset",
        )
        if os.path.exists(os.path.dirname(ios_assets_dir)):
            os.makedirs(ios_assets_dir, exist_ok=True)
            print("\n[STEP] Generating iOS icons...")
            ios_sizes = [
                (20, [1, 2, 3]),
                (29, [1, 2, 3]),
                (40, [1, 2, 3]),
                (60, [2, 3]),
                (76, [1, 2]),
                (83.5, [2]),
                (1024, [1]),
            ]
            contents_images = []
            for base_size, scales in ios_sizes:
                for scale in scales:
                    actual = int(base_size * scale)
                    filename = f"app_icon_{actual}x{actual}.png"
                    resized = im.resize((actual, actual), Image.LANCZOS)
                    resized.save(os.path.join(ios_assets_dir, filename), "PNG")
                    contents_images.append(
                        {
                            "filename": filename,
                            "idiom": "universal",
                            "scale": f"{scale}x",
                            "size": f"{base_size}x{base_size}",
                        }
                    )
                    print(f"  {filename} ({actual}x{actual})")

            # Write Contents.json
            contents = {
                "images": contents_images,
                "info": {"author": "gen-icons.py", "version": 1},
            }
            with open(os.path.join(ios_assets_dir, "Contents.json"), "w") as f:
                json.dump(contents, f, indent=2)
            print(f"  Contents.json written")
        else:
            print(
                "\n[SKIP] iOS assets directory not found (run 'tauri ios init' first)"
            )

    print("\n[DONE] All icons generated successfully!")


if __name__ == "__main__":
    main()
