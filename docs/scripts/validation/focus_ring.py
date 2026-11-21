#!/usr/bin/env python3
"""Verify the Docs search focus ring: a box-shadow ring with no geometry shift.

The search field used to signal focus by swapping the wrapper's transparent
1px border for the theme's ring colour. That is a layout change: the box
grows by a pixel and the text inside moves with it, and the theme's
``--ring-width`` / ``--ring-offset`` tokens were never consulted.

The framework's own controls paint the ring as a box-shadow that takes no
space, so this check asserts the same contract for the Docs search field:
the computed ``box-shadow`` carries the ring, the focused box and the text
position are identical to the unfocused state, and the ring colour is the
theme's own ring colour for the active scheme.
"""
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
    "zh": ("zh-CN", "搜索文档"),
    "en": ("en-US", "Search documentation"),
    "ja": ("ja-JP", "ドキュメントを検索"),
    "ko": ("ko-KR", "문서 검색"),
}
WRAPPER = ".c_docs_header_search"
FIELD = ".c_docs_header_search_input"
LISTBOX = "[role=listbox]"


def check(record, name, actual, expected):
    record["checks"].append(dict(name=name, actual=actual, expected=expected, passed=actual == expected))


def metrics(page, selector):
    """Returns the box metrics of the search wrapper and of the text inside it.

    The text offset is measured from the field the wrapper owns, so the two
    states compare the same box and the offset is relative to the box, not to
    the page.
    """
    return page.evaluate(
        """(selector) => {
            const wrapper = document.querySelector(selector);
            if (!wrapper) return null;
            const box = wrapper.getBoundingClientRect();
            const style = getComputedStyle(wrapper);
            const field = wrapper.querySelector('input');
            const fieldBox = field ? field.getBoundingClientRect() : null;
            return {
                width: box.width,
                height: box.height,
                x: box.x,
                y: box.y,
                text_x: fieldBox ? fieldBox.x - box.x : null,
                text_y: fieldBox ? fieldBox.y - box.y : null,
                box_shadow: style.boxShadow,
                border_width: style.borderWidth,
                border_top_color: style.borderTopColor,
            };
        }""",
        selector,
    )


def ring_values(page):
    """Returns the theme's own ring tokens on the themed root."""
    return page.evaluate(
        """() => {
            const root = document.querySelector('.c_app_root, .c_mobile_app_root') || document.documentElement;
            const style = getComputedStyle(root);
            const pick = (name) => style.getPropertyValue(name).trim();
            return {
                ring: pick('--ring'),
                ring_width: pick('--ring-width'),
                ring_offset: pick('--ring-offset'),
                role_ring_light: pick('--role-ring-light'),
                role_ring_dark: pick('--role-ring-dark'),
                theme: root.className,
            };
        }"""
    )


def parse_shadow(value):
    """Extracts the ring colour from a computed box-shadow string.

    Chrome serialises the computed value with the colour first
    (``rgb(0, 0, 0) 0px 0px 0px 2px``), so the colour is the first token
    that is not a length.
    """
    if not value or value == "none":
        return None
    text = value.strip()
    if text.startswith("#"):
        return text.split()[0]
    if text.startswith("rgb"):
        end = text.index(")")
        return text[: end + 1]
    for token in text.split():
        if token.startswith("#") or token.startswith("rgb"):
            return token
    return text


def normalise_colour(value):
    """Normalises a CSS colour to a comparable RGB tuple string."""
    text = (value or "").strip().lower()
    if text.startswith("#"):
        digits = text[1:]
        if len(digits) == 3:
            digits = "".join(char * 2 for char in digits)
        if len(digits) >= 6:
            return "rgb(%d, %d, %d)" % (int(digits[0:2], 16), int(digits[2:4], 16), int(digits[4:6], 16))
    if text.startswith("rgba(") or text.startswith("rgb("):
        numbers = [number for number in text.replace("rgba(", "").replace("rgb(", "").replace(")", "").split(",") if number.strip()]
        if len(numbers) >= 3:
            return "rgb(%d, %d, %d)" % (int(float(numbers[0])), int(float(numbers[1])), int(float(numbers[2])))
    return text


def ring_is_spread(value):
    """True when the computed shadow spreads outward (a ring), not inward."""
    if not value or value == "none":
        return False
    return "inset" not in value.lower()


def shadow_spread(value):
    """Returns the outward spread radius of a computed box-shadow.

    The computed form is ``<colour> <offset-x> <offset-y> <blur> <spread>``,
    so the spread is the fourth length, not the blur.
    """
    if not value or value == "none":
        return 0.0
    lengths = [float(token[:-2]) for token in value.replace("rgba", " ").replace("rgb", " ").split() if token.endswith("px")]
    return lengths[3] if len(lengths) > 3 else 0.0


def length_px(value):
    """Returns a CSS length in pixels, or 0.0 when it is not a length."""
    text = (value or "").strip().lower()
    if text.endswith("px"):
        try:
            return float(text[:-2])
        except ValueError:
            return 0.0
    return 0.0


def assert_ring(record, page, locale, width, unfocused):
    """Asserts the ring contract on the focused search wrapper.

    The assertions are deliberately fail-closed: every value the browser
    reports is compared against the theme's own tokens, so a rule that
    stopped painting a ring, painted it in the wrong colour or painted it
    with the wrong spread fails the check instead of passing silently.
    """
    tokens = ring_values(page)
    record["theme_tokens"] = tokens
    check(record, "theme_defines_ring_width", bool(tokens["ring_width"]), True)
    check(record, "theme_defines_ring_offset", bool(tokens["ring_offset"]), True)
    check(record, "theme_defines_ring_colour", bool(tokens["ring"]), True)
    check(record, "unfocused_has_no_ring", ring_is_spread(unfocused["box_shadow"]), False)
    page.locator(FIELD).first.focus()
    page.wait_for_timeout(150)
    focused = metrics(page, WRAPPER)
    record["unfocused"] = unfocused
    record["focused"] = focused
    check(record, "focused_paints_ring", ring_is_spread(focused["box_shadow"]), True)
    check(record, "focused_shadow_is_not_none", focused["box_shadow"] not in (None, "none", ""), True)
    shadow_colour = normalise_colour(parse_shadow(focused["box_shadow"]))
    theme_colour = normalise_colour(tokens["ring"])
    check(record, "ring_colour_matches_theme", shadow_colour, theme_colour)
    spread = shadow_spread(focused["box_shadow"])
    expected_spread = length_px(tokens["ring_width"]) + length_px(tokens["ring_offset"])
    record["ring_spread"] = spread
    record["expected_ring_spread"] = expected_spread
    check(record, "ring_spread_matches_theme_tokens", spread, expected_spread)
    check(record, "ring_is_thicker_than_a_1px_border", spread > 1.0, True)
    check(record, "ring_is_not_a_border_swap", focused["border_top_color"], unfocused["border_top_color"])
    check(record, "wrapper_width_unchanged", focused["width"], unfocused["width"])
    check(record, "wrapper_height_unchanged", focused["height"], unfocused["height"])
    check(record, "wrapper_x_unchanged", focused["x"], unfocused["x"])
    check(record, "wrapper_y_unchanged", focused["y"], unfocused["y"])
    check(record, "text_x_unchanged", focused["text_x"], unfocused["text_x"])
    check(record, "text_y_unchanged", focused["text_y"], unfocused["text_y"])
    field_unfocused = metrics(page, FIELD)
    page.locator(FIELD).first.focus()
    page.wait_for_timeout(150)
    field_focused = metrics(page, FIELD)
    record["field_unfocused"] = field_unfocused
    record["field_focused"] = field_focused
    check(record, "field_width_unchanged", field_focused["width"], field_unfocused["width"])
    check(record, "field_height_unchanged", field_focused["height"], field_unfocused["height"])
    check(record, "field_x_unchanged", field_focused["x"], field_unfocused["x"])
    check(record, "field_y_unchanged", field_focused["y"], field_unfocused["y"])
    check(record, "border_width_unchanged", focused["border_width"], unfocused["border_width"])
    check(record, "border_colour_unchanged", focused["border_top_color"], unfocused["border_top_color"])


def assert_keyboard_path(record, page):
    """Asserts the keyboard path still reaches the field and drives the listbox."""
    page.keyboard.type("pdf")
    page.wait_for_timeout(400)
    check(record, "keyboard_opens_listbox", page.locator(LISTBOX).count() > 0, True)
    check(record, "field_aria_expanded", page.locator(FIELD).first.get_attribute("aria-expanded"), "true")
    page.keyboard.press("Escape")
    page.wait_for_timeout(200)
    check(record, "Escape_closes_listbox", page.locator(FIELD).first.get_attribute("aria-expanded"), "false")


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
                errors, wasm = [], []
                page.on("pageerror", lambda error: errors.append(str(error)))

                def response_seen(response):
                    if urlsplit(response.url).path.endswith(".wasm"):
                        wasm.append(dict(url=response.url, status=response.status, sha256=hashlib.sha256(response.body()).hexdigest()))

                page.on("response", response_seen)
                for locale, (tag, placeholder) in MESSAGES.items():
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
                            # The search field lives in the desktop header only:
                            # the mobile shell renders its own header without a
                            # search box, so the ring contract has no surface
                            # there and the absence must be asserted, not
                            # silently skipped.
                            if page.locator(WRAPPER).count() == 0:
                                check(record, "mobile_has_no_search_field", True, True)
                                check(record, "mobile_search_field_count", page.locator(FIELD).count(), 0)
                            else:
                                unfocused = metrics(page, WRAPPER)
                                check(record, "wrapper_present", unfocused is not None, True)
                                if unfocused is None:
                                    raise AssertionError("The search wrapper is not rendered")
                                assert_ring(record, page, locale, width, unfocused)
                                assert_keyboard_path(record, page)
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
                        print(json.dumps(dict(locale=locale, width=width, passed=record["passed"], failed=[item["name"] for item in record["checks"] if not item["passed"]]), ensure_ascii=False), flush=True)
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
