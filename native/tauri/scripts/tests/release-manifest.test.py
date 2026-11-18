#!/usr/bin/env python3
"""Regression tests for the production website inventory generator."""
import importlib.util
import json
import hashlib
import os
from pathlib import Path
import tempfile
import unittest

SOURCE = Path(__file__).resolve().parents[4] / "example/scripts/write_release_manifest.py"
spec = importlib.util.spec_from_file_location("release_manifest", SOURCE)
assert spec is not None and spec.loader is not None
manifest = importlib.util.module_from_spec(spec)
spec.loader.exec_module(manifest)


class ReleaseManifestTests(unittest.TestCase):
    def test_complete_deterministic_inventory(self):
        with tempfile.TemporaryDirectory(dir=os.environ["TMPDIR"]) as temporary:
            root = Path(temporary)
            for name, data in [("index.html", b"root"), ("themes/default/index.html", b"default"), ("themes/custom/pkg/app.wasm", b"wasm"), ("style.css", b"css"), ("images/logo.svg", b"svg"), ("_manifest.json", b"reserved")]:
                target = root / name
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_bytes(data)
            self.assertEqual(manifest.generate(root, "0.35.91"), 5)
            first = (root / manifest.NAME).read_bytes()
            manifest.generate(root, "0.35.91")
            self.assertEqual(first, (root / manifest.NAME).read_bytes())
            release = json.loads(first)
            self.assertEqual(release["version"], "0.35.91")
            for item in release["files"]:
                data = (root / item["path"]).read_bytes()
                self.assertEqual(item["size"], len(data))
                self.assertEqual(item["sha256"], hashlib.sha256(data).hexdigest())

    def test_rejects_symlinks_and_unsafe_names(self):
        for name in ["/absolute", "../escape", "a//b", "a%2fb", "https://evil/a", "a\\b", "x\nx"]:
            self.assertFalse(manifest.safe_path(name))
        with tempfile.TemporaryDirectory(dir=os.environ["TMPDIR"]) as temporary:
            root = Path(temporary)
            (root / "index.html").write_bytes(b"root")
            (root / "link").symlink_to(root / "index.html")
            with self.assertRaises(ValueError):
                manifest.generate(root, "0.35.91")


if __name__ == "__main__":
    unittest.main()
