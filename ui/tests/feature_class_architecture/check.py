"""Read-only production architecture checks for canonical feature classes."""
from pathlib import Path
import argparse
import json
import re


def feature_runtime_checks(source):
    """Find cfg! macros even when any/all/not nest across source lines."""
    findings = []
    for match in re.finditer(r"\bcfg\s*!\s*\(", source):
        depth = 1
        quoted = False
        escaped = False
        end = match.end()
        while end < len(source) and depth:
            char = source[end]
            if quoted:
                if escaped:
                    escaped = False
                elif char == "\\":
                    escaped = True
                elif char == '"':
                    quoted = False
            elif char == '"':
                quoted = True
            elif char == "(":
                depth += 1
            elif char == ")":
                depth -= 1
            end += 1
        if re.search(r'feature\s*=\s*"theme-[^\"]+"', source[match.end():end]):
            findings.append(source.count("\n", 0, match.start()) + 1)
    return findings


def check(root):
    mapping = json.loads(Path(__file__).with_name("mapping.json").read_text())
    legacy = re.compile(r"\b(?:" + "|".join(map(re.escape, mapping)) + r")\b")
    errors = []
    for directory in [root / "src/component", root / "src/style/class"]:
        for path in sorted(directory.rglob("*.rs")):
            source = path.read_text()
            errors.extend(f"{path.relative_to(root)}:{line}: Cargo theme choice outside class! definitions"
                          for line in feature_runtime_checks(source))
            errors.extend(f"{path.relative_to(root)}:{source.count(chr(10), 0, match.start()) + 1}: legacy factory {match.group()}"
                          for match in legacy.finditer(source))
    return errors


def self_test():
    for source in [
        'if cfg!(feature = "theme-shadcn") {}',
        'cfg ! (any(target_arch = "wasm32",\n all(not(feature = "theme-geist"), feature = "theme-shadcn")))',
        'cfg!(not(any(feature = "theme-geist", feature = "theme-park")))',
    ]:
        assert feature_runtime_checks(source) == [1]
    for source in ['cfg!(target_arch = "wasm32")', '#[cfg(feature = "theme-shadcn")]',
                   'if active && dark_mode {}', 'c_scheme_shadcn()']:
        assert not feature_runtime_checks(source)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("root", type=Path, nargs="?", default=Path(__file__).resolve().parents[2])
    args = parser.parse_args()
    self_test()
    errors = check(args.root)
    print("\n".join(errors) if errors else "canonical feature-class architecture: passed")
    raise SystemExit(bool(errors))
