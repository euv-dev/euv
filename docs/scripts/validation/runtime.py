"""Exercise the public docs WASM entrypoint without changing browser ownership or artifacts."""
import argparse
import hashlib
import json
import re
import threading
import urllib.request
from functools import partial
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import urlsplit

from importlib import import_module
from typing import cast

ROUTES = ["#/ltpp-html-pdf/", "#/http-request/", "#/ltpp-web-ide/", "#/", "#/ltpp-html-pdf/", "#/http-request/"]
LIFECYCLE_ROUTES = ["#/http-request/", "#/ltpp-web-ide/", "#/", "#/http-request/", "#/", "#/ltpp-web-ide/"]
RUNTIME_ERRORS = re.compile(r"panicked at|unreachable|closure invoked recursively|after being dropped", re.I)


class CatalogServer(ThreadingHTTPServer):
    def __init__(self, address, handler, catalog_url):
        self.catalog_url = catalog_url
        super().__init__(address, handler)


class Handler(SimpleHTTPRequestHandler):
    def do_GET(self):
        if urlsplit(self.path).path == "/api/page-artifacts":
            try:
                request = urllib.request.Request(cast(CatalogServer, self.server).catalog_url, headers={"User-Agent": "docs-runtime-regression/1.0"})
                with urllib.request.urlopen(request, timeout=30) as response:
                    data = response.read()
                self.send_response(200)
                self.send_header("Content-Type", "application/json")
                self.end_headers()
                self.wfile.write(data)
            except Exception:
                self.send_error(502, "Public catalog unavailable")
            return
        super().do_GET()

    def log_message(self, format, *args):
        pass


def run(args):
    args.output.parent.mkdir(parents=True, exist_ok=True)
    server = CatalogServer(("127.0.0.1", 0), partial(Handler, directory=str(args.directory.resolve())), args.catalog_url)
    threading.Thread(target=server.serve_forever, daemon=True).start()
    records = []
    closed = False
    try:
        with import_module("playwright.sync_api").sync_playwright() as playwright:
            browser = playwright.chromium.launch(executable_path=args.chrome, headless=True)
            try:
                page = browser.new_page()
                errors, console, clicks, wasm = [], [], [], []
                page.on("pageerror", lambda error: errors.append(str(error)))
                page.on("console", lambda message: console.append(message.text) if message.type == "error" else None)
                page.expose_function("recordRuntimeClick", lambda event: clicks.append(event))
                page.add_init_script("document.addEventListener('click',e=>window.recordRuntimeClick({href:e.target.closest('a')?.getAttribute('href'),trusted:e.isTrusted}),true)")

                def response_seen(response):
                    if urlsplit(response.url).path.endswith(".wasm"):
                        try:
                            wasm.append({"url": response.url, "sha256": hashlib.sha256(response.body()).hexdigest()})
                        except Exception as error:
                            wasm.append({"error": type(error).__name__})

                page.on("response", response_seen)
                for locale in args.locales.split(","):
                    folder = "" if locale == "zh" else locale + "/"
                    files = list((args.directory / folder / "pkg").glob("*.wasm"))
                    if len(files) != 1:
                        raise ValueError(f"Expected one WASM for {locale}, found {len(files)}")
                    expected_hash = hashlib.sha256(files[0].read_bytes()).hexdigest()
                    for width in (1280, 390):
                        for mode in ("cold-pdf", "persistent-pdf", "persistent-lifecycle"):
                            record = {"locale": locale, "width": width, "mode": mode, "passed": False, "transitions": []}
                            page.goto("about:blank")
                            errors.clear(); console.clear(); clicks.clear(); wasm.clear()
                            try:
                                page.set_viewport_size({"width": width, "height": 900})
                                base = f"http://127.0.0.1:{server.server_port}/{folder}"
                                page.goto(base + ("#/ltpp-html-pdf/" if mode == "cold-pdf" else ""), wait_until="domcontentloaded")
                                page.locator("h1").first.wait_for(timeout=12000)
                                if mode == "cold-pdf":
                                    page.wait_for_timeout(500)
                                    assert any("pdf" in text.lower() for text in page.locator("h1").all_inner_texts()), "PDF heading not rendered"
                                else:
                                    page.wait_for_function("document.querySelectorAll('.docs-catalog-card').length >= 2", timeout=45000)
                                    record["catalog_cards"] = page.locator(".docs-catalog-card").count()
                                    marker = page.evaluate("window.__runtimeDocument=crypto.randomUUID()")
                                    home_heading = page.locator("h1").all_inner_texts()
                                    sequence = ROUTES if mode == "persistent-pdf" else LIFECYCLE_ROUTES
                                    for target in sequence:
                                        if width == 390:
                                            page.locator(".c_mobile_menu_button").first.click(timeout=10000)
                                        selector = f'.c_mobile_nav_drawer a[href="{target}"]' if width == 390 else f'a[href="{target}"]:visible'
                                        page.locator(selector).first.click(timeout=10000)
                                        page.wait_for_timeout(700)
                                        headings = page.locator("h1").all_inner_texts()
                                        assert page.evaluate("window.__runtimeDocument") == marker, "Document reloaded"
                                        assert clicks and clicks[-1]["trusted"] and clicks[-1]["href"] == target, "Not a trusted route click"
                                        assert headings, "No rendered heading"
                                        assert target == "#/" or headings != home_heading, "Homepage content retained after navigation"
                                        if "html-pdf" in target:
                                            assert any("pdf" in text.lower() for text in headings), "PDF content missing"
                                        assert not errors and not any(RUNTIME_ERRORS.search(line) for line in console), "Runtime failure during navigation"
                                        record["transitions"].append({"target": target, "headings": headings, "same_document": True, "trusted": True})
                                    for direction in ("back", "back", "forward", "forward"):
                                        page.evaluate(f"history.{direction}()")
                                        page.wait_for_timeout(700)
                                        assert page.evaluate("window.__runtimeDocument") == marker, "History reloaded document"
                                        assert not errors and not any(RUNTIME_ERRORS.search(line) for line in console), "Runtime failure during history navigation"
                                assert not errors and not any(RUNTIME_ERRORS.search(line) for line in console), "Runtime failure"
                                assert wasm and all(entry.get("sha256") == expected_hash for entry in wasm), "Loaded WASM differs from artifact"
                                record["passed"] = True
                            except Exception as error:
                                record["error"] = str(error)
                                page.screenshot(path=str(args.output.parent / f"{locale}-{width}-{mode}.png"))
                            record.update(page_errors=list(errors), console_errors=list(console), loaded_wasm=list(wasm), expected_wasm_sha256=expected_hash)
                            records.append(record)
                            args.output.write_text(json.dumps({"records": records}, ensure_ascii=False, indent=2))
                            print(json.dumps({key: record.get(key) for key in ("locale", "width", "mode", "passed", "error")}, ensure_ascii=False), flush=True)
            finally:
                browser.close()
                closed = True
    finally:
        server.shutdown()
        server.server_close()
    result = {"scope": "Docs runtime crashes only; unrelated resource/localization defects are not certified.", "total": len(records), "failed": sum(not item["passed"] for item in records), "browser_closed": closed, "records": records}
    args.output.write_text(json.dumps(result, ensure_ascii=False, indent=2))
    return 1 if result["failed"] else 0


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("directory", type=Path)
    parser.add_argument("output", type=Path)
    parser.add_argument("--locales", default="zh,en,ja,ko")
    parser.add_argument("--catalog-url", default="https://once-drop.ltpp.vip/api/page-artifacts")
    parser.add_argument("--chrome", default="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome")
    raise SystemExit(run(parser.parse_args()))
