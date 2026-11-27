#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=../env/common.sh
source "${SCRIPT_DIR}/../env/common.sh"
load_local_env

cd "${REPO_ROOT}/example"
log_info "Building Release WebAssembly bundle..."
wasm-pack build --target web --release --out-dir ../dist/web

cd "${REPO_ROOT}"
python3 - <<'PY'
from pathlib import Path
import shutil

out_dir = Path("dist/web")
wasm_file = "euv_example_bg.wasm"
js_file = out_dir / "euv_example.js"
brand_dir = Path("assets/branding")
favicon_tag = (
    '<link rel="icon" type="image/svg+xml" href="data:image/svg+xml,'
    '%3Csvg xmlns=\'http://www.w3.org/2000/svg\' viewBox=\'0 0 64 64\'%3E'
    '%3Cdefs%3E%3ClinearGradient id=\'g\' x1=\'0\' y1=\'0\' x2=\'1\' y2=\'1\'%3E'
    '%3Cstop offset=\'0%25\' stop-color=\'%23111322\'/%3E'
    '%3Cstop offset=\'55%25\' stop-color=\'%23252a5c\'/%3E'
    '%3Cstop offset=\'100%25\' stop-color=\'%234f46e5\'/%3E%3C/linearGradient%3E%3C/defs%3E'
    '%3Crect x=\'4\' y=\'4\' width=\'56\' height=\'56\' rx=\'16\' fill=\'url(%23g)\'/%3E'
    '%3Crect x=\'18\' y=\'17\' width=\'28\' height=\'6\' rx=\'3\' fill=\'%23ffffff\'/%3E'
    '%3Crect x=\'18\' y=\'29\' width=\'22\' height=\'6\' rx=\'3\' fill=\'%23a5b4fc\'/%3E'
    '%3Crect x=\'18\' y=\'41\' width=\'28\' height=\'6\' rx=\'3\' fill=\'%23ffffff\'/%3E'
    '%3C/svg%3E"/>'
)
brand_svg = brand_dir / "euv-studio-logo.svg"
brand_png = brand_dir / "euv-studio-logo-64.png"
if brand_svg.is_file():
    shutil.copy2(brand_svg, out_dir / "euv-studio-logo.svg")
    favicon_tag = '<link rel="icon" type="image/svg+xml" href="./euv-studio-logo.svg"/>'
if brand_png.is_file():
    shutil.copy2(brand_png, out_dir / "euv-studio-logo-64.png")
    if not brand_svg.is_file():
        favicon_tag = '<link rel="icon" type="image/png" sizes="64x64" href="./euv-studio-logo-64.png"/>'
inline_runtime = js_file.read_text(encoding="utf-8")
inline_runtime = inline_runtime.replace(f"new URL('{wasm_file}', import.meta.url)", f"'./{wasm_file}'").replace(f'new URL("{wasm_file}", import.meta.url)', f"'./{wasm_file}'")
cleaned = []
for line in inline_runtime.splitlines():
    t = line.lstrip()
    if t.startswith("export {") or t.startswith("export default "):
        continue
    if line.startswith("export "):
        line = line[len("export "):]
    cleaned.append(line)
cleaned_loader = "\n".join(cleaned)
html = (
    '<!DOCTYPE html>\n<html lang="en">\n<head>\n<meta charset="utf-8"/>\n'
    '<meta name="viewport" content="width=device-width,initial-scale=1,viewport-fit=cover"/>\n'
    f'{favicon_tag}\n'
    '<title>Euv Studio</title>\n'
    '<style>:root{--euv-safe-top:env(safe-area-inset-top,0px);--euv-safe-right:env(safe-area-inset-right,0px);--euv-safe-bottom:env(safe-area-inset-bottom,0px);--euv-safe-left:env(safe-area-inset-left,0px);}html,body{margin:0;padding:0;width:100vw;height:100vh;height:100dvh;overflow:hidden;background:#09090b;touch-action:none;}</style>\n'
    '</head>\n<body>\n<script type="module">\n'
    f'{cleaned_loader}\n__wbg_init({{ module_or_path: \'./{wasm_file}\' }});\n'
    '</script>\n</body>\n</html>\n'
)
(out_dir / "index.html").write_text(html, encoding="utf-8")
for p in out_dir.iterdir():
    if p.is_file() and (p.suffix == ".js" or p.name.endswith(".d.ts") or p.name in ("package.json", ".gitignore")):
        p.unlink()
PY

JS_COUNT="$(find dist/web -maxdepth 1 -name '*.js' | wc -l | tr -d ' ')"
WASM_SCHEMA="$(strings dist/web/euv_example_bg.wasm | grep -c "euv.ai.schema.v1" || true)"
log_info "Zero-JS Web bundle ready in dist/web (.js count=${JS_COUNT}, release ai-schema=${WASM_SCHEMA})"
[ "${JS_COUNT}" -eq 0 ] && [ "${WASM_SCHEMA}" -eq 0 ]
