"""Validate the example's live route/theme inventory and removed feature wiring."""
import argparse
import importlib.util
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[3]


def check(root):
    metadata = json.loads(subprocess.check_output(
        ["cargo", "metadata", "--no-deps", "--format-version", "1"], cwd=root, text=True))
    packages = {package["name"]: package for package in metadata["packages"]}
    example = packages["euv-example"]
    features = {name for name in example["features"] if name.startswith("theme-")}
    removed = "theme-" + "custom"
    assert all(removed not in package["features"] for package in packages.values()), removed
    tracked = subprocess.check_output(["git", "ls-files", "-z"], cwd=root).decode().split("\0")
    references = []
    for name in tracked:
        path = root / name
        if path.is_file() and path.suffix in {".rs", ".toml", ".yml", ".yaml", ".md", ".py", ".sh", ".json"}:
            if removed in path.read_text(errors="replace"):
                references.append(name)
    assert not references, references
    for name in ("example/tests/custom_layout", "docs/tests/custom_layout", "example/src/style/class/motion", "example/tests/motion"):
        assert not (root / name).exists(), name
    table = (root / "example/src/component/layout/view/const.rs").read_text()
    options = {"default", *re.findall(r'\("(theme-[^\"]+)"\s*,', table)}
    declaration = re.search(r'THEME_FEATURE_OPTIONS:\s*\[\(&str, &str\);\s*(\d+)\]', table)
    assert declaration, "Missing typed theme option table"
    declared = int(declaration[1])
    assert options == features | {"default"}, (sorted(options), sorted(features))
    assert declared == len(options), (declared, len(options))
    spec = importlib.util.spec_from_file_location("example_runtime", root / "example/scripts/validation/runtime.py")
    assert spec and spec.loader, "Runtime inventory module is missing"
    runtime = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(runtime)
    inv = runtime.inventory()
    views = set()
    for path in (root / "example/src/page").glob("*/view/fn.rs"):
        views.update(re.findall(r'fn\s+(page_\w+)\(', path.read_text()))
    router = (root / "example/src/component/router/view/fn.rs").read_text()
    routed_views = set(re.findall(r'\b(page_\w+)\s*\{\}', router))
    assert views == routed_views, {"unrouted": sorted(views-routed_views), "missing": sorted(routed_views-views)}
    assert len({item["route"] for item in inv["nav"]}) == len(inv["nav"])
    # Attribute extension is a general API, not the removed theme feature.
    assert "/custom-attrs" in inv["routes"] and "page_custom_attrs" in routed_views
    result = {"passed": True, "routes": inv["routes"], "route_count": len(inv["routes"]),
              "navigation": inv["nav"], "nav_count": len(inv["nav"]),
              "views": sorted(views), "theme_variants": sorted(options), "theme_count": len(options),
              "surface_card_classes":inv["surface_cards"],"content_card_classes":inv["content_cards"],"source_card_classes":inv["source_card_classes"],
              "removed_feature_references": references, "tracked_paths_checked": len(tracked)-1,
              "generic_attribute_extension_preserved": True}
    return result


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=ROOT)
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    result = check(args.root.resolve())
    if args.output:
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(json.dumps(result, ensure_ascii=False, indent=2))
    print(json.dumps(result, ensure_ascii=False))
