#!/usr/bin/env python3
"""Verify localized theme/language titles and exact AX names on ordinary Docs URLs."""
import argparse
from datetime import datetime, timezone
from functools import partial
import hashlib
from importlib import import_module
import json
from pathlib import Path
import threading
from urllib.parse import urlsplit

from runtime import CatalogServer, Handler

MESSAGES = {
    "zh": ("zh-CN", "语言", "简体中文", "切换主题"),
    "en": ("en-US", "Language", "English", "Toggle theme"),
    "ja": ("ja-JP", "言語", "日本語", "テーマを切り替える"),
    "ko": ("ko-KR", "언어", "한국어", "테마 전환"),
}
LOCALE_TRIGGER = ".c_nav_locale_row button"
CHINESE_LANGUAGE_TITLE = "语言"


def check(record, name, actual, expected):
    record["checks"].append(dict(name=name, actual=actual, expected=expected, passed=actual == expected))


def accessible_button(cdp, selector):
    document = cdp.send("DOM.getDocument")
    node = cdp.send("DOM.querySelector", dict(nodeId=document["root"]["nodeId"], selector=selector))
    assert node["nodeId"], selector
    nodes = cdp.send("Accessibility.getPartialAXTree", dict(nodeId=node["nodeId"], fetchRelatives=False))["nodes"]
    exposed = [node for node in nodes if not node.get("ignored")]
    assert len(exposed) == 1, exposed
    return dict(role=exposed[0].get("role", {}).get("value"), name=exposed[0].get("name", {}).get("value"))


def inspect_control(page, cdp, record, prefix, selector, title, name):
    control = page.locator(selector)
    check(record, prefix + "_count", control.count(), 1)
    check(record, prefix + "_title", control.get_attribute("title"), title)
    check(record, prefix + "_AX", accessible_button(cdp, selector), dict(role="button", name=name))


def inspect_language_control(page, cdp, record, locale, title, label):
    """Assert the language control's title is the locale's own string.

    The control used to carry one hard-coded Chinese title in every
    locale, so the check is deliberately negative as well: outside the
    Chinese locale the title must not be the Chinese literal, and the
    accessible name must stay the visible locale caption.
    """
    control = page.locator(LOCALE_TRIGGER)
    check(record, "language_count", control.count(), 1)
    actual = control.get_attribute("title")
    check(record, "language_title", actual, title)
    check(record, "language_AX", accessible_button(cdp, LOCALE_TRIGGER), dict(role="button", name=label))
    check(record, "language_caption", control.inner_text().strip(), label)
    if locale == "zh":
        check(record, "language_title_is_chinese", actual == CHINESE_LANGUAGE_TITLE, True)
    else:
        check(record, "language_title_is_not_chinese", actual != CHINESE_LANGUAGE_TITLE, True)


def run(args):
    args.output.parent.mkdir(parents=True, exist_ok=True)
    server = thread = None
    directory = args.directory or args.artifact_dir
    base = args.base_url
    if args.directory:
        server = CatalogServer(("127.0.0.1", 0), partial(Handler, directory=str(args.directory.resolve())), args.catalog_url)
        thread = threading.Thread(target=server.serve_forever, daemon=True)
        thread.start()
        base = f"http://127.0.0.1:{server.server_port}/"
    parts = urlsplit(base)
    if parts.query or parts.fragment or parts.username or parts.password:
        raise ValueError("Use an ordinary base URL without query, fragment or credentials")
    records: list[dict] = []
    result: dict = dict(started_at=datetime.now(timezone.utc).isoformat(), base_url=base, records=records, browser_closed=False, server_closed=False)
    try:
        with import_module("playwright.sync_api").sync_playwright() as playwright:
            browser = playwright.chromium.launch(executable_path=args.chrome, headless=True)
            try:
                page = browser.new_page()
                cdp = page.context.new_cdp_session(page)
                errors, wasm = [], []
                page.on("pageerror", lambda error: errors.append(str(error)))

                def response_seen(response):
                    if urlsplit(response.url).path.endswith(".wasm"):
                        wasm.append(dict(url=response.url, status=response.status, sha256=hashlib.sha256(response.body()).hexdigest()))

                page.on("response", response_seen)
                for locale, (tag, title, label, theme) in MESSAGES.items():
                    folder = "" if locale == "zh" else locale + "/"
                    expected_digest = None
                    if directory:
                        candidates = list((directory / folder / "pkg").glob("*.wasm"))
                        assert len(candidates) == 1, candidates
                        expected_digest = hashlib.sha256(candidates[0].read_bytes()).hexdigest()
                    for width in (1280, 390):
                        page.goto("about:blank")
                        page.set_viewport_size(dict(width=width, height=900))
                        errors.clear()
                        wasm.clear()
                        url = base.rstrip("/") + "/" + folder + "#/ltpp-html-pdf/"
                        record: dict = dict(locale=locale, width=width, height=900, url=url, checks=[])
                        records.append(record)
                        try:
                            response = page.goto(url, wait_until="domcontentloaded")
                            page.locator("h1").first.wait_for(timeout=30000)
                            check(record, "document_HTTP", response.status, 200)
                            check(record, "document_language", page.locator("html").get_attribute("lang"), tag)
                            theme_selector = ".c_docs_header_icon_button" if width == 1280 else ".c_mobile_theme_button"
                            inspect_control(page, cdp, record, "theme", theme_selector, theme, theme)
                            if width == 390:
                                page.locator("#docs-mobile-drawer-trigger").click()
                                page.locator(".c_mobile_nav_drawer:not(.c_mobile_nav_drawer_closed)").wait_for()
                                page.wait_for_timeout(300)
                            inspect_language_control(page, cdp, record, locale, title, label)
                            record["loaded_wasm"] = list(wasm)
                            check(record, "one_loaded_WASM", len(wasm), 1)
                            check(record, "WASM_HTTP", [item["status"] for item in wasm], [200])
                            if expected_digest:
                                check(record, "loaded_artifact_identity", [item["sha256"] for item in wasm], [expected_digest])
                            check(record, "no_runtime_errors", list(errors), [])
                        except Exception as error:
                            check(record, "execution", str(error), None)
                        record["passed"] = all(item["passed"] for item in record["checks"])
                        args.output.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n")
                        print(json.dumps(dict(locale=locale, width=width, passed=record["passed"], failed=[item for item in record["checks"] if not item["passed"]]), ensure_ascii=False), flush=True)
            finally:
                browser.close()
                result["browser_closed"] = True
    finally:
        if server and thread:
            server.shutdown()
            server.server_close()
            thread.join()
        result["server_closed"] = True
        result.update(total=len(records), failed=sum(not record.get("passed", False) for record in records), checks=sum(len(record["checks"]) for record in records), finished_at=datetime.now(timezone.utc).isoformat())
        args.output.write_text(json.dumps(result, ensure_ascii=False, indent=2) + "\n")
    return int(result["total"] != 8 or result["failed"] != 0)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    source = parser.add_mutually_exclusive_group(required=True)
    source.add_argument("--directory", type=Path)
    source.add_argument("--base-url")
    parser.add_argument("output", type=Path)
    parser.add_argument("--artifact-dir", type=Path)
    parser.add_argument("--catalog-url", default="https://ltpp.vip/api/page-artifacts")
    parser.add_argument("--chrome", default="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome")
    raise SystemExit(run(parser.parse_args()))
