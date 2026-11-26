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

out_dir = Path("dist/web")
wasm_file = "euv_example_bg.wasm"
js_file = out_dir / "euv_example.js"
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
    '<link rel="icon" href="data:image/svg+xml,%3Csvg xmlns=\'http://www.w3.org/2000/svg\' viewBox=\'0 0 32 32\'%3E%3Crect width=\'32\' height=\'32\' rx=\'8\' fill=\'%2318181b\'/%3E%3Ctext x=\'16\' y=\'22\' text-anchor=\'middle\' font-family=\'sans-serif\' font-weight=\'800\' font-size=\'18\' fill=\'%23fafafa\'%3EE%3C/text%3E%3C/svg%3E"/>\n'
    '<title>euv 0.38 — Cross-Platform Native Showcase</title>\n'
    '<style>html,body{margin:0;padding:0;width:100vw;height:100vh;overflow:hidden;background:#09090b;touch-action:none;}</style>\n'
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
