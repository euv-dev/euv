#!/usr/bin/env python3
"""Write the website-owned, deterministic native update inventory."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess

NAME = "euv-release.json"
RESERVED = {NAME, "__euv_local_bundle__.json", "_manifest.json", ".empty", "active"}


def safe_path(value):
    return bool(value) and all(part not in ("", ".", "..") for part in value.split("/")) and not any(ord(c) < 32 or ord(c) == 127 or c in "\\:%?#" for c in value) and not any(part in RESERVED for part in value.split("/"))


def resolved_version(example):
    output = subprocess.check_output(["cargo", "metadata", "--no-deps", "--format-version", "1", "--manifest-path", str(example / "Cargo.toml")], text=True)
    packages = json.loads(output)["packages"]
    return next(package["version"] for package in packages if Path(package["manifest_path"]).resolve() == (example / "Cargo.toml").resolve())


def generate(dist, version):
    if not re.fullmatch(r"(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)", version):
        raise ValueError("Release version must be a numeric semantic version")
    if dist.is_symlink() or not dist.is_dir():
        raise ValueError("Distribution must be a regular directory")
    files = []
    for path in sorted(dist.rglob("*")):
        relative = path.relative_to(dist).as_posix()
        if path.is_symlink():
            raise ValueError(f"Symlink is forbidden: {relative}")
        if path.name in RESERVED:
            continue
        if not safe_path(relative):
            raise ValueError(f"Unsafe release path: {relative}")
        if path.is_dir():
            continue
        if not path.is_file():
            raise ValueError(f"Not a regular file: {relative}")
        data = path.read_bytes()
        files.append({"path": relative, "sha256": hashlib.sha256(data).hexdigest(), "size": len(data)})
    if not any(item["path"] == "index.html" and item["size"] for item in files):
        raise ValueError("Missing nonempty index.html")
    payload = (json.dumps({"format": 1, "version": version, "files": files}, indent=2, ensure_ascii=True) + "\n").encode()
    destination = dist / NAME
    destination.write_bytes(payload)
    return len(files)


def main():
    example_dir = Path(__file__).resolve().parents[2] / "example"
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--dist", type=Path, default=example_dir / "www")
    parser.add_argument("--version")
    args = parser.parse_args()
    version = args.version or resolved_version(example_dir)
    count = generate(args.dist.absolute(), version)
    print(f"Wrote {args.dist / NAME}: {version}, {count} files")


if __name__ == "__main__":
    main()
