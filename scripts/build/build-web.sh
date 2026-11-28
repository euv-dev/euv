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
python3 "${REPO_ROOT}/scripts/env/generate-icon.py" "${REPO_ROOT}/dist/web/euv-studio-logo-64.png" 64 "${EUV_APP_ICON:-}"
python3 - <<'PY'
from pathlib import Path

out_dir = Path("dist/web")
wasm_file = "euv_example_bg.wasm"
js_file = out_dir / "euv_example.js"
favicon_tag = (
    '<link rel="icon" type="image/svg+xml" href="data:image/svg+xml,'
    '%3Csvg xmlns=\'http://www.w3.org/2000/svg\' viewBox=\'0 0 64 64\'%3E'
    '%3Cdefs%3E%3ClinearGradient id=\'g\' x1=\'8\' y1=\'8\' x2=\'56\' y2=\'56\' gradientUnits=\'userSpaceOnUse\'%3E'
    '%3Cstop offset=\'0%25\' stop-color=\'%23fff2a6\'/%3E'
    '%3Cstop offset=\'48%25\' stop-color=\'%23ffd24d\'/%3E'
    '%3Cstop offset=\'100%25\' stop-color=\'%23d97706\'/%3E%3C/linearGradient%3E%3C/defs%3E'
    '%3Cpath d=\'M25 28 L5 8 L10 26 L22 33 Z M34 32 L59 7 L54 27 L39 36 Z\' fill=\'url(%23g)\'/%3E'
    '%3Cpath d=\'M7 7 L14 14 M57 7 L49 15\' stroke=\'%23e6192e\' stroke-width=\'3.2\' stroke-linecap=\'round\'/%3E'
    '%3Cpath d=\'M22 22 L32 14 L44 6 L34 22 L40 38 L30 48 L20 42 L20 28 Z\' fill=\'%2314171f\' stroke=\'url(%23g)\' stroke-width=\'2.5\' stroke-linejoin=\'round\'/%3E'
    '%3Cpolygon points=\'25,33 29,36 27,43 22,40\' fill=\'%23e6192e\'/%3E'
    '%3Ccircle cx=\'26\' cy=\'16\' r=\'2.2\' fill=\'%23e6192e\'/%3E'
    '%3C/svg%3E"/>'
)
web_png = out_dir / "euv-studio-logo-64.png"
if web_png.is_file():
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
