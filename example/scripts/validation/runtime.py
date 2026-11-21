"""Measure the rebuilt example's route, card, navigation and input contracts."""
import argparse
import hashlib
import json
import re
import threading
from functools import partial
from http.server import SimpleHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

from importlib import import_module

ROOT = Path(__file__).resolve().parents[3]
CHROME = "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"
BAD_RUNTIME = re.compile(r"panicked at|unreachable|closure invoked recursively|after being dropped", re.I)

MEASURE = """() => {
    const root = document.querySelector('.c_app_root,.c_mobile_app_root');
    const rect = e => {
        const r=e.getBoundingClientRect(), s=getComputedStyle(e);
        return {classes:e.className, x:r.x,y:r.y,right:r.right,bottom:r.bottom,width:r.width,height:r.height,
            padding:[s.paddingTop,s.paddingRight,s.paddingBottom,s.paddingLeft].map(parseFloat),
            margin:[s.marginTop,s.marginRight,s.marginBottom,s.marginLeft].map(parseFloat),
            gap:s.gap,display:s.display,border:s.borderWidth,radius:s.borderRadius,shadow:s.boxShadow,
            background:s.backgroundColor,tap:s.webkitTapHighlightColor};
    };
    const token = name => {
        if (!getComputedStyle(root).getPropertyValue('--'+name).trim()) return null;
        const probe=document.createElement('i');
        probe.style.cssText='position:absolute;visibility:hidden;pointer-events:none;height:var(--'+name+');';
        root.append(probe); const value=parseFloat(getComputedStyle(probe).height); probe.remove(); return value;
    };
    const boxes = parent => [...parent.children].flatMap(e => getComputedStyle(e).display==='contents' ? boxes(e) : [e]);
    const cards=[...document.querySelectorAll('.c_page_router .c_card,.c_page_router .c_euv_feature_card,.c_page_router .c_virtual_list_card')];
    const parents=[...new Set(cards.map(e=>{
        let parent=e.parentElement;
        while(parent && getComputedStyle(parent).display==='contents') parent=parent.parentElement;
        return parent;
    }))];
    return {root:rect(root),tokens:Object.fromEntries(['card-padding','card-content-gap','page-block-gap',
        'page-block-gap-mobile','nav-row-height','nav-item-inset','space-xs-sm','space-lg','ring-width','ring-offset'].map(name=>[name,token(name)])),
        cards:cards.map(e=>({...rect(e),kind:e.classList.contains('c_card')?'card':e.classList.contains('c_euv_feature_card')?'feature-card':'virtual-list-card',
            title:e.querySelector('.c_card_title,h2,h3,h4')?.textContent.trim(),nested:!!e.parentElement.closest('.c_card')})),
        contentCards:[...document.querySelectorAll('.c_page_router .c_feature_card,.c_page_router .c_home_stat_card')].map(rect),
        controls:[...document.querySelectorAll('.c_page_router button,.c_page_router input')].map(rect),
        groups:parents.map(parent=>({parent:parent.className,display:getComputedStyle(parent).display,
            gap:getComputedStyle(parent).gap,cards:boxes(parent).filter(e=>e.matches('.c_card,.c_euv_feature_card,.c_virtual_list_card')).map(rect)})),
        cardKinds:[...new Set([...document.querySelectorAll('.c_page_router [class]')].flatMap(e=>[...e.classList]).filter(name=>/^c_.*card$/.test(name)))].sort(),
        home:[...document.querySelectorAll('.c_home,.c_home_content,.c_home_badge_row,.c_home_title,.c_home_subtitle,.c_home_actions,.c_home_stats,.c_home_stat_card,.c_home_section_title,.c_home_section_desc,.c_home_feature_grid,.c_card_stack')].map(rect),
        headings:[...document.querySelectorAll('.c_page_router h1')].map(e=>e.textContent.trim()),
        featureOptions:[...document.querySelectorAll('#theme-feature-selector option')].map(e=>e.value),
        textSelection:getComputedStyle(document.querySelector('.c_euv_user_select_text')||root).userSelect};
}"""


def inventory():
    constants = (ROOT / "example/src/component/router/view/const.rs").read_text()
    routes = dict(re.findall(r'const\s+(\w+):\s*&str\s*=\s*"(/[^\"]*)"', constants))
    router = (ROOT / "example/src/component/router/view/fn.rs").read_text()
    names = set(re.findall(r'\b(?:ROUTE_\w+|GESTURE_ROUTE)\b', router))
    nav = re.findall(r'\("([^\"]*)",\s*"([^\"]*)",\s*"(/[^\"]*)"\)',
                     (ROOT / "example/src/component/nav/view/const.rs").read_text())
    selected = sorted({routes[name] for name in names})
    route_views = {}
    for arm in re.finditer(r'((?:ROUTE_\w+|GESTURE_ROUTE)(?:\s*\|\s*(?:ROUTE_\w+|GESTURE_ROUTE))*)\s*=>\s*\{\s*(page_\w+)\s*\{\s*\}',router):
        for name in re.findall(r'ROUTE_\w+|GESTURE_ROUTE',arm[1]):
            route_views[routes[name]] = arm[2]
    assert set(route_views) == set(selected)
    assert selected and nav and {row[2] for row in nav} <= set(selected)
    pages = sorted(path.parent.parent.name for path in (ROOT / "example/src/page").glob("*/view/fn.rs"))
    surface_cards = ["c_card","c_euv_feature_card","c_virtual_list_card"]
    content_cards = ["c_feature_card","c_home_stat_card"]
    source_cards = sorted({name for path in (ROOT/"example/src").rglob("*.rs")
                           for name in re.findall(r'\b(c_[a-z0-9_]*card)\s*\(',path.read_text())})
    assert set(source_cards) <= set(surface_cards+content_cards), source_cards
    return {"routes": selected, "route_views":route_views, "nav": [{"icon": i, "label": n, "route": r} for i, n, r in nav], "page_sources": pages,"surface_cards":surface_cards,"content_cards":content_cards,"source_card_classes":source_cards}


class Handler(SimpleHTTPRequestHandler):
    def log_message(self, format, *args):
        pass


def navigate(page, url, width, mode):
    page.set_viewport_size({"width": width, "height": 900})
    page.emulate_media(color_scheme=mode)
    page.goto(url, wait_until="domcontentloaded")
    page.locator(".c_page_container").first.wait_for(timeout=15000)
    page.wait_for_function("""([width,mode])=>{
        const roots=document.querySelectorAll('.c_app_root,.c_mobile_app_root');
        return roots.length===1 && innerWidth===width && roots[0].classList.contains(width<768?'c_mobile_app_root':'c_app_root')
            && roots[0].classList.contains('c_theme_'+mode);
    }""",arg=[width,mode],timeout=10000)
    page.wait_for_timeout(80)


def navigation(page, width, expected, current, screenshots=None):
    if width < 768:
        page.locator(".c_mobile_menu_button").first.click()
        page.wait_for_function("""()=>{const e=document.querySelector('.c_mobile_nav_drawer');if(!e)return false;
            const r=e.getBoundingClientRect();return r.left>=-.1 && r.right<=innerWidth+.1 && getComputedStyle(e).pointerEvents!=='none';}""",timeout=10000)
    rows = page.locator('.c_nav_items_scroll a[href^="#"]')
    assert rows.count() == len(expected), (rows.count(), len(expected))
    tokens = page.evaluate(MEASURE)["tokens"]
    records = rows.evaluate_all("""elements=>elements.map(e=>{
        e.scrollIntoView({behavior:'instant',block:'nearest',inline:'nearest'});
        const r=e.getBoundingClientRect(),p=e.closest('.c_nav_items_scroll'),b=p.getBoundingClientRect(),s=getComputedStyle(e);
        const hit=document.elementFromPoint(r.x+r.width/2,r.y+r.height/2);
        return {label:e.textContent.trim(),href:e.getAttribute('href'),height:r.height,left:r.left-b.left,right:b.right-r.right,
            top:r.top,bottom:r.bottom,nativeLeft:p.clientLeft,nativeRight:p.offsetWidth-p.clientWidth-p.clientLeft,
            scrollTop:p.scrollTop,scrollHeight:p.scrollHeight,clientHeight:p.clientHeight,
            hit:!!hit&&e.contains(hit),shadow:s.boxShadow,radius:s.borderRadius,background:s.backgroundColor};
    })""")
    for index, item in enumerate(expected):
        row = rows.nth(index)
        record = records[index]
        assert record["href"] == "#" + item["route"] and record["hit"], record
        if current:
            minimum = tokens["nav-row-height"]
            assert minimum and record["height"] >= minimum - .1, record
            maximum = tokens["space-lg"]
            inset = tokens["nav-item-inset"]
            # The column reserves its scrollbar gutter on the right only, so the
            # row's own inset is the column's padding on both sides and the left
            # edge keeps nothing but a hint of breathing room.
            assert record["nativeLeft"] < .1, record
            assert 0 < record["left"] <= inset + .1 and record["left"] <= maximum + .1, record
            assert abs(record["left"]-record["nativeLeft"]-inset)<.1, record
            assert abs(record["right"]-record["nativeRight"]-inset)<.1, record
            assert record["right"] >= record["left"], record
            assert inset>=tokens["ring-width"]+tokens["ring-offset"], tokens
        if index in {0, len(expected)//2, len(expected)-1}:
            row.scroll_into_view_if_needed()
            before = row.bounding_box()
            row.hover()
            after = row.bounding_box()
            assert before == after, "Navigation hover changed geometry"
            record["hoverShadow"] = row.evaluate("e=>getComputedStyle(e).boxShadow")
            record["hoverBackground"] = row.evaluate("e=>getComputedStyle(e).backgroundColor")
            if screenshots and index in (0,len(expected)-1):
                page.screenshot(path=str(screenshots.with_name(screenshots.name+f"-{index}-hover.png")))
    # Exercise actual keyboard modality, not programmatic focus styling.
    rows.first.focus()
    focused = None
    for _ in range(len(expected) + 15):
        page.keyboard.press("Tab")
        focused = page.evaluate("""() => {
            const e=document.activeElement;
            if(!e?.matches('.c_nav_items_scroll a'))return null;
            const s=getComputedStyle(e);return {label:e.textContent,visible:e.matches(':focus-visible'),outline:s.outlineStyle,
                outlineWidth:s.outlineWidth,shadow:s.boxShadow};
        }""")
        if focused:
            break
    assert focused and focused["visible"], focused
    if current:
        assert focused["outline"] != "none" or focused["shadow"] != "none", focused
    long_row = rows.nth(max(range(len(expected)), key=lambda i: len(expected[i]["label"])))
    long_row.scroll_into_view_if_needed()
    long_label = long_row.evaluate("e=>({label:e.textContent,width:e.getBoundingClientRect().width,height:e.getBoundingClientRect().height,scroll:e.scrollWidth,client:e.clientWidth})")
    if current:
        assert long_label["scroll"] <= long_label["client"] + 1, long_label
    compatibility = long_row.evaluate("""e=>{
        const parent=e.closest('.c_nav_items_scroll'),probe=document.createElement('a'),label=document.createElement('span');
        probe.className=e.className;label.className='c_nav_item_label';label.textContent='Internationalization and long feature navigation labels';probe.append(label);parent.append(probe);
        try {probe.scrollIntoView();const r=probe.getBoundingClientRect(),p=parent.getBoundingClientRect(),s=getComputedStyle(label);
            return {height:r.height,left:r.left-p.left,right:p.right-r.right,scroll:probe.scrollWidth,client:probe.clientWidth,ellipsis:s.textOverflow,nowrap:s.whiteSpace,
                nativeLeft:p.clientLeft,nativeRight:p.offsetWidth-p.clientWidth-p.clientLeft};
        }finally{probe.remove();}
    }""")
    if current:
        inset = tokens["nav-item-inset"]
        assert compatibility["height"]>=tokens["nav-row-height"] and compatibility["nativeLeft"] < .1, compatibility
        assert abs(compatibility["left"]-compatibility["nativeLeft"]-inset)<.1, compatibility
        assert abs(compatibility["right"]-compatibility["nativeRight"]-inset)<.1, compatibility
        assert compatibility["scroll"]<=compatibility["client"]+1 and compatibility["ellipsis"]=="ellipsis", compatibility
    if width < 768:
        page.locator(".c_mobile_drawer_close_button").first.click()
        page.wait_for_function("""()=>{const e=document.querySelector('.c_mobile_nav_drawer');return e &&
            (e.getBoundingClientRect().right<=.1 || getComputedStyle(e).visibility==='hidden');}""",timeout=10000)
    scrollbar = page.locator('.c_nav_items_scroll').evaluate("""e=>({width:getComputedStyle(e).scrollbarWidth,
        color:getComputedStyle(e).scrollbarColor,webkitWidth:getComputedStyle(e,'::-webkit-scrollbar').width,
        thumb:getComputedStyle(e,'::-webkit-scrollbar-thumb').backgroundColor,
        webkitSupported:CSS.supports('(selector(::-webkit-scrollbar))')})""")
    if current:
        assert scrollbar["width"]=="thin" and scrollbar["color"]!="auto", scrollbar
    return {"rows": records, "keyboard": focused, "longestLabel": long_label,"longLabelFixture":compatibility,"scrollbar":scrollbar}


def boundaries(page, width):
    records = page.evaluate("""() => {
        const host=document.querySelector('.c_app_root,.c_mobile_app_root');
        const records=[];
        for(const depth of [0,1,2,4]) {
            const fixture=document.createElement('div'); fixture.className='c_page_container';
            fixture.style.cssText='position:fixed;left:0;top:0;visibility:hidden;pointer-events:none;width:'+host.clientWidth+'px';
            let outer=fixture;
            for(let level=0;level<depth;level++) {
                const boundary=document.createElement(level%2?'slot':'div');
                boundary.setAttribute('data-euv-dynamic-id','stack-probe');boundary.style.display='contents';outer.append(boundary);outer=boundary;
            }
            const stack=document.createElement('div'); stack.className='c_card_stack'; outer.append(stack);
            for(let index=0;index<2;index++) {
                let parent=stack;
                for(let level=0;level<depth;level++) {
                    const boundary=document.createElement(level%2?'slot':'div');
                    boundary.setAttribute('data-euv-dynamic-id','spacing-probe'); boundary.style.display='contents';
                    parent.append(boundary); parent=boundary;
                }
                const card=document.createElement('div');card.className='c_card';card.textContent='card';parent.append(card);
            }
            host.append(fixture);
            try {
                const cards=[...fixture.querySelectorAll('.c_card')], a=cards[0].getBoundingClientRect(),b=cards[1].getBoundingClientRect();
                records.push({depth,gap:b.top-a.bottom,stackMargin:getComputedStyle(stack).marginTop,margins:cards.map(e=>getComputedStyle(e).marginTop),
                    token:getComputedStyle(host).getPropertyValue(innerWidth<768?'--page-block-gap-mobile':'--page-block-gap').trim()});
            } finally {fixture.remove();}
        }
        return records;
    }""")
    for row in records:
        assert abs(row["gap"] - float(row["token"].removesuffix("px"))) < .1, row
        assert row["margins"] == ["0px","0px"], row
        assert abs(float(row["stackMargin"].removesuffix("px"))-float(row["token"].removesuffix("px")))<.1, row
    return records


def interaction(page, cdp, width, output, current):
    events = []
    page.evaluate("window.__inputEvidence=[]; for(const name of ['pointerdown','touchstart','mousedown','click']) document.addEventListener(name,e=>window.__inputEvidence.push({type:e.type,trusted:e.isTrusted,target:e.target.className}),{capture:true,passive:true})")
    categories = [("blank", ".c_app_main,.c_mobile_main"), ("card", ".c_card")]
    for kind, selector in [("link", ".c_home_btn_secondary"),
                           ("button", ".c_page_router .c_button_controls button"),
                           ("input", ".c_page_router input:not([type=hidden])")]:
        if page.locator(selector).count():
            categories.append((kind, selector))
    records = []
    for name, selector in categories:
        target = page.locator(selector).first
        target.scroll_into_view_if_needed()
        box = target.bounding_box()
        x, y = box["x"] + box["width"]/2, max(1, box["y"]) + min(box["height"], 100)/2
        if name == "blank":
            x, y = box["x"] + 3, max(box["y"], 0) + 4
        hit = page.evaluate("([x,y])=>{const e=document.elementFromPoint(x,y);return {tag:e.tagName,classes:e.className,tap:getComputedStyle(e).webkitTapHighlightColor}}", [x,y])
        page.mouse.move(x,y)
        page.mouse.down()
        page.wait_for_timeout(150)
        pressed = page.evaluate("([x,y])=>{const e=document.elementFromPoint(x,y),s=getComputedStyle(e);return {tap:s.webkitTapHighlightColor,background:s.backgroundColor,transform:s.transform,focus:e.matches(':focus'),focusVisible:e.matches(':focus-visible'),shadow:s.boxShadow,active:e.matches(':active'),ancestors:[...document.querySelectorAll(':active')].map(n=>n.className)}}", [x,y])
        page.screenshot(path=str(output / f"{width}-{name}-pointer.png"))
        # Cancel the click when a link would otherwise navigate away.
        page.mouse.move(0, 0)
        page.mouse.up()
        cdp.send("Emulation.setTouchEmulationEnabled", {"enabled": True, "maxTouchPoints": 1})
        cdp.send("Input.dispatchTouchEvent", {"type":"touchStart","touchPoints":[{"x":x,"y":y}]})
        page.wait_for_timeout(150)
        page.screenshot(path=str(output / f"{width}-{name}-touch.png"))
        touch = page.evaluate("([x,y])=>{const e=document.elementFromPoint(x,y);return {tap:getComputedStyle(e).webkitTapHighlightColor,active:e.matches(':active'),ancestors:[...document.querySelectorAll(':active')].map(n=>n.className)}}", [x,y])
        cdp.send("Input.dispatchTouchEvent", {"type":"touchEnd","touchPoints":[]})
        page.wait_for_timeout(30)
        page.screenshot(path=str(output / f"{width}-{name}-tap.png"))
        page.wait_for_timeout(320)
        record = {"category": name, "hit":hit, "pointer":pressed,"touch":touch,
                  "completed_tap":page.evaluate("({url:location.href,clicks:window.__inputEvidence.filter(e=>e.type==='click'&&e.trusted).length})")}
        records.append(record)
        if current:
            assert all(value["tap"] == "rgba(0, 0, 0, 0)" for value in [hit,pressed,touch]), record
    events = page.evaluate("window.__inputEvidence")
    assert any(e["trusted"] and e["type"] == "pointerdown" for e in events), events
    assert any(e["trusted"] and e["type"] == "touchstart" for e in events), events
    editable = page.locator('.c_page_router input[type="text"],.c_page_router input:not([type])')
    selection = None
    if editable.count():
        field = editable.first
        field.fill("selectable text")
        field.press("ControlOrMeta+A")
        selection = field.evaluate("e=>({start:e.selectionStart,end:e.selectionEnd,length:e.value.length})")
        assert selection["start"] == 0 and selection["end"] == selection["length"], selection
    return {"surfaces":records,"events":events,"textSelection":selection}


def run(args):
    args.output.parent.mkdir(parents=True, exist_ok=True)
    shots = args.output.parent / (args.output.stem + "-screenshots")
    shots.mkdir(exist_ok=True)
    inv = inventory()
    variants = args.directory / "themes" if (args.directory / "themes").is_dir() else args.directory
    schemes = sorted(path.name for path in variants.iterdir() if path.is_dir() and (path / "index.html").is_file())
    if args.schemes:
        schemes = args.schemes.split(",")
    assert schemes
    hashes = {str(path.relative_to(variants)):hashlib.sha256(path.read_bytes()).hexdigest() for path in variants.glob("*/pkg/*.wasm") if path.parent.parent.name in schemes}
    report = {"inventory":inv,"schemes":schemes,"routes":[],"fallback":[],"patterns":[],"artifact_hashes":hashes,"browser_closed":False,"server_closed":False}
    if args.resume and args.output.is_file():
        previous = json.loads(args.output.read_text())
        assert previous["inventory"] == inv and previous["schemes"] == schemes
        assert previous.get("artifact_hashes", hashes) == hashes, "Artifacts changed; cannot resume previous evidence"
        report["routes"] = [r for r in previous["routes"] if r.get("passed") or
            (args.baseline and r.get("headings") and not any(BAD_RUNTIME.search(e) for e in r.get("errors",[])))]
        report["patterns"] = [r for r in previous["patterns"] if r.get("passed",args.baseline) and "navigation" in r and
            (r["scheme"] != "default" or r["width"] not in (1280,390) or r["mode"] != "light" or "interaction" in r)]
    server = ThreadingHTTPServer(("127.0.0.1",0),partial(Handler,directory=str(variants.resolve())))
    threading.Thread(target=server.serve_forever,daemon=True).start()
    def save():
        args.output.write_text(json.dumps(report, ensure_ascii=False, indent=2))
    try:
        with import_module("playwright.sync_api").sync_playwright() as pw:
            browser = pw.chromium.launch(executable_path=args.chrome, headless=True)
            try:
                page = browser.new_page(has_touch=True)
                cdp = page.context.new_cdp_session(page)
                errors = []
                page.on("pageerror",lambda error:errors.append(str(error)))
                page.on("console",lambda message:errors.append("console: "+message.text) if message.type=="error" and BAD_RUNTIME.search(message.text) else None)
                page.add_init_script("window.__euvRuntimeDocument=crypto.randomUUID()")
                loaded_documents = {}
                wasm = []
                def response_seen(response):
                    if response.url.split("?",1)[0].endswith(".wasm"):
                        try:
                            wasm.append({"url":response.url,"sha256":hashlib.sha256(response.body()).hexdigest()})
                        except Exception as error:
                            wasm.append({"url":response.url,"error":str(error)})
                page.on("response",response_seen)
                prefix = f"http://127.0.0.1:{server.server_port}"
                def go(scheme, route, width, mode):
                    files = list((variants/scheme/"pkg").glob("*.wasm"))
                    assert len(files)==1, files
                    expected = hashlib.sha256(files[0].read_bytes()).hexdigest()
                    wasm.clear()
                    navigate(page,f"{prefix}/{scheme}/#{route}",width,mode)
                    document = page.evaluate("window.__euvRuntimeDocument")
                    if wasm:
                        assert all(row.get("sha256")==expected for row in wasm), (scheme,route,expected,wasm)
                        loaded_documents[document] = list(wasm)
                        kind = "network"
                    else:
                        assert document in loaded_documents, "No verified WASM for the active document"
                        assert all(row.get("sha256")==expected for row in loaded_documents[document]), (scheme,route,expected,loaded_documents[document])
                        kind = "same-document"
                    return [{**row,"document":document,"load_kind":kind} for row in loaded_documents[document]]
                if args.tap_only:
                    report["tap_wasm"] = go(schemes[0],"/",390,"light")
                    report["tap"] = interaction(page,cdp,390,shots,not args.baseline)
                else:
                    baseline_scheme = "default" if "default" in schemes else schemes[0]
                    for width in (() if args.patterns_only else (1280,390)):
                        for mode in ("light","dark"):
                            for route in inv["routes"]:
                                if any((r["route"],r["width"],r["mode"]) == (route,width,mode) for r in report["routes"]):
                                    continue
                                errors.clear()
                                loaded = go(baseline_scheme,route,width,mode)
                                measured = page.evaluate(MEASURE)
                                record = {"route":route,"view":inv["route_views"][route],"width":width,"mode":mode,**measured,"errors":list(errors),"loaded_wasm":loaded}
                                assert not errors, errors
                                assert measured["headings"], f"No page heading: {route}"
                                assert set(measured["cardKinds"]) <= set(inv["surface_cards"]+inv["content_cards"]), measured["cardKinds"]
                                if route=="/virtual-list":
                                    assert any(card["kind"]=="virtual-list-card" for card in measured["cards"]), "Virtual-list card was not audited"
                                if not args.baseline:
                                    for card in measured["cards"]:
                                        assert card["padding"] == [measured["tokens"]["card-padding"]]*4, card
                                record["passed"] = True
                                report["routes"].append(record)
                                save()
                    for width in (() if args.patterns_only else (1280,390)):
                        for mode in ("light","dark"):
                            loaded = go(baseline_scheme,"/__euv_missing_route__",width,mode)
                            measured = page.evaluate(MEASURE)
                            assert measured["headings"] and any("404" in heading for heading in measured["headings"]), measured["headings"]
                            if not args.baseline:
                                assert all(card["padding"]==[measured["tokens"]["card-padding"]]*4 for card in measured["cards"])
                            report["fallback"].append({"view":"page_not_found","width":width,"mode":mode,**measured,"loaded_wasm":loaded})
                            save()
                    for scheme in schemes:
                        for width in (1280,390,767,768):
                            for mode in ("light","dark"):
                                if any((r["scheme"],r["width"],r["mode"]) == (scheme,width,mode) for r in report["patterns"]):
                                    continue
                                errors.clear()
                                loaded = go(scheme,"/",width,mode)
                                record = {"scheme":scheme,"width":width,"mode":mode,**page.evaluate(MEASURE),"loaded_wasm":loaded}
                                report["patterns"].append(record)
                                nav_shots = shots / f"{scheme}-{width}-{mode}-nav" if scheme in ("default","shadcn","theme-shadcn") else None
                                nav = navigation(page,width,inv["nav"],not args.baseline,nav_shots)
                                record["navigation"] = nav
                                if not args.baseline and scheme in ("default","shadcn","theme-shadcn"):
                                    record["transparent_boundaries"] = boundaries(page,width)
                                if not args.baseline:
                                    token = record["tokens"]["page-block-gap-mobile" if width<768 else "page-block-gap"]
                                    for group in record["groups"]:
                                        cards = group["cards"]
                                        if group["display"] == "flex" and len(cards)>1:
                                            for first,second in zip(cards,cards[1:]):
                                                assert abs(second["y"]-first["bottom"]-token)<.1, group
                                    for card in record["cards"]:
                                        assert card["padding"] == [record["tokens"]["card-padding"]]*4, card
                                    for group in record["home"]:
                                        if "c_card_stack" in group["classes"].split():
                                            assert abs(group["margin"][0]-token)<.1, group
                                    assert ("theme-"+"custom") not in record["featureOptions"], record["featureOptions"]
                                if scheme==baseline_scheme and width in (1280,390):
                                    interaction_shots = shots / f"{scheme}-{width}-{mode}"
                                    interaction_shots.mkdir(exist_ok=True)
                                    record["interaction"] = {"home":interaction(page,cdp,width,interaction_shots,not args.baseline)}
                                    for route in ("/counter", "/form"):
                                        loaded = go(scheme,route,width,mode)
                                        route_shots = interaction_shots / route.strip("/")
                                        route_shots.mkdir(exist_ok=True)
                                        record["interaction"][route] = {**interaction(page,cdp,width,route_shots,not args.baseline),"loaded_wasm":loaded}
                                record["shared_page_samples"] = {}
                                for route in ("/virtual-list","/counter","/form"):
                                    loaded = go(scheme,route,width,mode)
                                    sample = page.evaluate(MEASURE)
                                    if not args.baseline:
                                        assert sample["cards"], (scheme,route,"No audited card surface")
                                        assert all(card["padding"]==[sample["tokens"]["card-padding"]]*4 for card in sample["cards"]), sample["cards"]
                                        assert all(control["tap"]=="rgba(0, 0, 0, 0)" for control in sample["controls"]), sample["controls"]
                                    record["shared_page_samples"][route] = {**sample,"loaded_wasm":loaded}
                                assert not errors, errors
                                record["errors"] = list(errors)
                                record["passed"] = True
                                save()
                if not args.tap_only:
                    if not args.patterns_only:
                        assert len({(r["route"],r["width"],r["mode"]) for r in report["routes"]}) == len(inv["routes"])*4
                        assert len(report["fallback"]) == 4
                    assert len({(r["scheme"],r["width"],r["mode"]) for r in report["patterns"]}) == len(schemes)*8
                report["route_count"] = len(set(r["route"] for r in report["routes"]))
                report["route_cases"] = len(report["routes"])
                report["pattern_cases"] = len(report["patterns"])
                report["page_view_count"] = len({r["view"] for r in report["routes"]} | {r["view"] for r in report["fallback"]})
                report["artifact_hashes"] = {str(path.relative_to(variants)):hashlib.sha256(path.read_bytes()).hexdigest() for path in variants.glob("*/pkg/*.wasm")}
                report["passed"] = True
            finally:
                browser.close()
                report["browser_closed"] = True
    except Exception as error:
        report["passed"] = False
        report["error"] = str(error)
        raise
    finally:
        server.shutdown()
        server.server_close()
        report["server_closed"] = True
        save()
    print(json.dumps({key:report.get(key) for key in ("passed","route_count","route_cases","pattern_cases","browser_closed","server_closed")}),flush=True)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("directory",type=Path)
    parser.add_argument("output",type=Path)
    parser.add_argument("--chrome",default=CHROME)
    parser.add_argument("--schemes")
    parser.add_argument("--baseline",action="store_true")
    parser.add_argument("--tap-only",action="store_true")
    parser.add_argument("--resume",action="store_true")
    parser.add_argument("--patterns-only",action="store_true")
    run(parser.parse_args())
