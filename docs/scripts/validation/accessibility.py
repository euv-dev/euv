#!/usr/bin/env python3
"""Exercise localized Docs search and modal navigation with trusted keyboard events."""
import argparse
import hashlib
import json
from pathlib import Path
import threading
from functools import partial
from importlib import import_module

from runtime import CatalogServer, Handler

MESSAGES = {
    "zh": ["打开导航菜单", "关闭导航菜单", "文档导航", "切换主题", "搜索文档", "搜索文档…", "输入关键词搜索文档", "没有匹配的页面"],
    "en": ["Open navigation menu", "Close navigation menu", "Documentation navigation", "Toggle theme", "Search documentation", "Search documentation...", "Type to search documentation", "No matching pages"],
    "ja": ["ナビゲーションメニューを開く", "ナビゲーションメニューを閉じる", "ドキュメントナビゲーション", "テーマを切り替える", "ドキュメントを検索", "ドキュメントを検索…", "キーワードを入力してドキュメントを検索", "一致するページがありません"],
    "ko": ["탐색 메뉴 열기", "탐색 메뉴 닫기", "문서 탐색", "테마 전환", "문서 검색", "문서 검색…", "검색어를 입력하여 문서 검색", "일치하는 페이지가 없습니다"],
}
OPENER = '[class*=c_mobile_menu_button]'
CLOSE = '.c_mobile_drawer_close_button'
DRAWER = '.c_mobile_nav_drawer'
SEARCH = '[role=combobox]'


def check(record, name, condition, actual=None):
    record["checks"].append(dict(name=name, passed=bool(condition), actual=actual))


def ax_nodes(cdp):
    return [dict(role=node.get("role", {}).get("value"), name=node.get("name", {}).get("value"), properties=node.get("properties", [])) for node in cdp.send("Accessibility.getFullAXTree")["nodes"] if not node.get("ignored")]


def tab_to(page, selector, reverse=False):
    for _ in range(180):
        if page.locator(selector).evaluate_all('(nodes)=>nodes.includes(document.activeElement)'):
            return True
        page.keyboard.press("Shift+Tab" if reverse else "Tab")
    return False


def focused(page, selector):
    return page.locator(selector).evaluate_all('(nodes)=>nodes.includes(document.activeElement)')


def settle(page):
    page.evaluate('()=>new Promise(resolve=>requestAnimationFrame(()=>requestAnimationFrame(resolve)))')
    page.wait_for_timeout(160)


def check_search(page, cdp, record, messages):
    field = page.locator(SEARCH)
    check(record, "search_keyboard_reachable", tab_to(page, SEARCH))
    settle(page)
    check(record, "search_accessible_label", field.get_attribute("aria-label") == messages[4], field.get_attribute("aria-label"))
    check(record, "search_placeholder", field.get_attribute("placeholder") == messages[5], field.get_attribute("placeholder"))
    nodes = ax_nodes(cdp)
    check(record, "search_AX_name", any(n["role"] == "combobox" and n["name"] == messages[4] for n in nodes))
    status = page.locator('[role=status]')
    check(record, "empty_query_announcement", status.inner_text() == messages[6], status.inner_text())
    check(record, "polite_atomic_status", status.get_attribute("aria-live") == "polite" and status.get_attribute("aria-atomic") == "true")
    check(record, "status_exposed_in_AX", any(n["role"] == "status" and any(p["name"] == "live" and p["value"]["value"] == "polite" for p in n["properties"]) for n in nodes))
    page.keyboard.type("zzzz-no-matching-docs-928364")
    settle(page)
    check(record, "no_match_announcement", status.inner_text() == messages[7], status.inner_text())
    check(record, "no_match_AX_text", any(n["name"] == messages[7] for n in ax_nodes(cdp)))
    page.keyboard.press("Escape")
    settle(page)
    check(record, "Escape_closes_search", field.get_attribute("aria-expanded") == "false")
    check(record, "closed_status_not_in_AX", not any(n["role"] == "status" for n in ax_nodes(cdp)))
    page.keyboard.press("ControlOrMeta+A")
    page.keyboard.press("Backspace")
    page.keyboard.press("Tab")


def check_drawer(page, cdp, record, messages):
    opener, close, drawer = page.locator(OPENER), page.locator(CLOSE), page.locator(DRAWER)
    check(record, "closed_drawer_not_in_AX", not any(n["role"] == "dialog" or n["name"] == messages[1] for n in ax_nodes(cdp)))
    check(record, "opener_keyboard_reachable", tab_to(page, OPENER))
    check(record, "opener_name_and_title", opener.get_attribute("aria-label") == messages[0] and opener.get_attribute("title") == messages[0])
    check(record, "opener_AX_name", any(n["role"] == "button" and n["name"] == messages[0] for n in ax_nodes(cdp)))
    page.keyboard.press("Enter")
    settle(page)
    check(record, "keyboard_opens_drawer", "c_mobile_nav_drawer_closed" not in drawer.get_attribute("class"))
    check(record, "opener_ARIA_contract", opener.get_attribute("aria-expanded") == "true" and opener.get_attribute("aria-controls") == drawer.get_attribute("id"))
    check(record, "dialog_role_modal", drawer.get_attribute("role") == "dialog" and drawer.get_attribute("aria-modal") == "true")
    check(record, "dialog_localized_name", drawer.get_attribute("aria-label") == messages[2])
    check(record, "close_name_and_title", close.get_attribute("aria-label") == messages[1] and close.get_attribute("title") == messages[1])
    nodes = ax_nodes(cdp)
    record["drawer_accessibility"] = [n for n in nodes if n["role"] in ("dialog", "button")]
    check(record, "dialog_AX_name", any(n["role"] == "dialog" and n["name"] == messages[2] for n in nodes))
    check(record, "close_AX_name", any(n["role"] == "button" and n["name"] == messages[1] for n in nodes))
    check(record, "initial_focus_close", focused(page, CLOSE))
    controls = drawer.evaluate('(root)=>[...root.querySelectorAll("a[href],button,input,select,textarea,[tabindex]")].filter(e=>e.tabIndex>=0&&e.getBoundingClientRect().height>0&&!e.closest("[inert],[hidden],[disabled],[aria-hidden=true]")).length')
    trace = []
    for key in ("Tab", "Shift+Tab"):
        for _ in range(max(controls + 2, 8)):
            page.keyboard.press(key)
            trace.append(page.evaluate('({inside:!!document.activeElement?.closest(".c_mobile_nav_drawer"),id:document.activeElement?.id,tag:document.activeElement?.tagName})'))
    check(record, "forward_reverse_Tab_contained", all(entry["inside"] for entry in trace), trace)
    check(record, "background_inert", page.locator('.c_mobile_header').get_attribute("inert") is not None and page.locator('.c_mobile_main').get_attribute("inert") is not None)
    page.keyboard.press("Escape")
    settle(page)
    check(record, "Escape_closes_drawer", "c_mobile_nav_drawer_closed" in drawer.get_attribute("class"))
    check(record, "Escape_restores_opener", focused(page, OPENER))
    check(record, "closed_drawer_removed_from_AX", not any(n["role"] == "dialog" or n["name"] == messages[1] for n in ax_nodes(cdp)))
    tab_to(page, OPENER)
    page.keyboard.press("Space")
    settle(page)
    check(record, "Space_opens_and_focuses", focused(page, CLOSE))
    tab_to(page, CLOSE)
    page.keyboard.press("Enter")
    settle(page)
    check(record, "close_Enter_restores_opener", focused(page, OPENER) and "c_mobile_nav_drawer_closed" in drawer.get_attribute("class"))
    page.keyboard.press("Enter")
    settle(page)
    check(record, "locale_trigger_keyboard_reachable", tab_to(page, '.c_nav_locale_row button'))
    page.keyboard.press("Enter")
    settle(page)
    check(record, "nested_locale_menu_open", page.locator('[role=menu][data-state=open]').count() == 1)
    page.keyboard.press("ArrowDown")
    page.keyboard.press("Escape")
    settle(page)
    check(record, "nested_Escape_preserves_drawer", "c_mobile_nav_drawer_closed" not in drawer.get_attribute("class"))
    check(record, "nested_Escape_restores_locale_trigger", focused(page, '.c_nav_locale_row button'))
    page.keyboard.press("Escape")
    settle(page)
    check(record, "nested_followup_Escape_restores_opener", focused(page, OPENER))
    page.keyboard.press("Enter")
    settle(page)
    page.go_back(wait_until="domcontentloaded")
    settle(page)
    check(record, "history_close_restores_opener", focused(page, OPENER) and "c_mobile_nav_drawer_closed" in page.locator(DRAWER).get_attribute("class"))
    page.locator(OPENER).click()
    settle(page)
    check(record, "pointer_open_focuses_close", focused(page, CLOSE))
    page.locator(CLOSE).click()
    settle(page)
    check(record, "pointer_close_restores_opener", focused(page, OPENER))


def run(args):
    args.output.parent.mkdir(parents=True, exist_ok=True)
    server = CatalogServer(("127.0.0.1", 0), partial(Handler, directory=str(args.directory.resolve())), args.catalog_url)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    records: list[dict] = []
    try:
        with import_module('playwright.sync_api').sync_playwright() as playwright:
            browser = playwright.chromium.launch(executable_path=args.chrome, headless=True)
            try:
                page = browser.new_page()
                cdp = page.context.new_cdp_session(page)
                errors, events, wasm = [], [], []
                page.on('pageerror', lambda error: errors.append(dict(message=str(error), stack=error.stack, after=record['checks'][-1]['name'] if record['checks'] else 'load')))
                page.expose_function('recordAccessibilityKey', lambda event: events.append(event))
                page.add_init_script('document.addEventListener("keydown",e=>window.recordAccessibilityKey({key:e.key,trusted:e.isTrusted}),true)')
                def response_seen(response):
                    if response.url.split('?')[0].endswith('.wasm'):
                        wasm.append(dict(url=response.url, sha256=hashlib.sha256(response.body()).hexdigest()))
                page.on('response', response_seen)
                for locale in args.locales.split(','):
                    messages = MESSAGES[locale]
                    folder = '' if locale == 'zh' else locale+'/'
                    expected = list((args.directory/folder/'pkg').glob('*.wasm'))
                    assert len(expected) == 1
                    digest = hashlib.sha256(expected[0].read_bytes()).hexdigest()
                    for width in map(int, args.widths.split(',')):
                        record: dict = dict(locale=locale, initial_width=width, checks=[])
                        records.append(record)
                        errors.clear(); events.clear(); wasm.clear()
                        try:
                            page.goto('about:blank')
                            page.set_viewport_size(dict(width=width, height=900))
                            page.goto(f'http://127.0.0.1:{server.server_port}/{folder}#/ltpp-html-pdf/', wait_until='domcontentloaded')
                            page.locator('h1').first.wait_for(timeout=20000)
                            settle(page)
                            theme = page.locator('.c_docs_header_icon_button' if width == 1280 else '.c_mobile_theme_button')
                            check(record, 'theme_localized_name_and_title', theme.get_attribute('title') == messages[3] and theme.get_attribute('aria-label') == messages[3])
                            check(record, 'theme_AX_name', any(n['role'] == 'button' and n['name'] == messages[3] for n in ax_nodes(cdp)))
                            if width == 1280:
                                check_search(page, cdp, record, messages)
                                check(record, 'desktop_has_no_mobile_controls', page.locator(OPENER).count() == 0 and page.locator(DRAWER).count() == 0)
                                page.set_viewport_size(dict(width=390, height=900))
                                settle(page)
                            check_drawer(page, cdp, record, messages)
                            page.set_viewport_size(dict(width=1280, height=900))
                            settle(page)
                            check(record, 'resize_removes_modal_and_inert', page.locator(DRAWER).count() == 0 and page.locator('[inert]').count() == 0)
                            if width == 390:
                                check_search(page, cdp, record, messages)
                            check(record, 'real_trusted_keyboard_events', bool(events) and all(e['trusted'] for e in events), len(events))
                            check(record, 'loaded_exact_release_WASM', bool(wasm) and all(entry['sha256'] == digest for entry in wasm), list(wasm))
                            check(record, 'no_runtime_errors', not errors, list(errors))
                        except Exception as error:
                            check(record, 'execution', False, str(error))
                        record['passed'] = all(c['passed'] for c in record['checks'])
                        if not record['passed']:
                            page.screenshot(path=str(args.output.parent/f'{locale}-{width}-a11y.png'))
                        args.output.write_text(json.dumps(dict(records=records),ensure_ascii=False,indent=2))
                        print(json.dumps(dict(locale=locale,width=width,passed=record['passed'],failed=[c['name'] for c in record['checks'] if not c['passed']]),ensure_ascii=False),flush=True)
            finally:
                browser.close()
    finally:
        server.shutdown(); server.server_close(); thread.join()
    result = dict(total=len(records),failed=sum(not r['passed'] for r in records),checks=sum(len(r['checks']) for r in records),browser_closed=True,server_closed=True,records=records)
    args.output.write_text(json.dumps(result,ensure_ascii=False,indent=2))
    return int(result['failed'] != 0)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('directory',type=Path)
    parser.add_argument('output',type=Path)
    parser.add_argument('--catalog-url',default='https://once-drop.ltpp.vip/api/page-artifacts')
    parser.add_argument('--chrome',default='/Applications/Google Chrome.app/Contents/MacOS/Google Chrome')
    parser.add_argument('--locales',default='zh,en,ja,ko')
    parser.add_argument('--widths',default='1280,390')
    raise SystemExit(run(parser.parse_args()))
