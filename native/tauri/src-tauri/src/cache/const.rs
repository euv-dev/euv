include!(concat!(env!("OUT_DIR"), "/config_generated.rs"));
include!(concat!(env!("OUT_DIR"), "/bundled_cache_data.rs"));

/// JavaScript injected into **every** webview on each page load to intercept any
/// attempt to open an external link and route it to the system's default
/// browser instead of navigating in-app.
///
/// Because it is injected from [`crate::Builder::on_page_load`] — an application-level
/// hook that fires for every webview the app owns on every page load — it also
/// covers webviews created dynamically at runtime, after the app has started.
///
/// The script is idempotent (guarded by `__euvLinkGuard`) and intercepts:
/// - clicks on `<a href>` (including `target="_blank"`),
/// - `window.open(...)` calls,
/// - programmatic assignments to `window.location` / `location.href`.
///
/// Internal targets (the app's own `euv://` scheme, the Tauri/wry runtime
/// origins on `*.localhost`, relative URLs, and `about:`/`data:`/`blob:`/
/// `javascript:` URLs) are left untouched so in-app navigation keeps working.
/// Any genuine external `http(s)`/`mailto:`/`tel:`/… target is handed to the
/// `plugin:opener|open_url` command, which opens it in the system browser.
pub(crate) const LINK_INTERCEPTOR_SCRIPT: &str = r#"
(function () {
  if (window.__euvLinkGuard) { return; }
  window.__euvLinkGuard = true;
  if (!window.bridge && window.__TAURI__) { window.bridge = window.__TAURI__; delete window.__TAURI__; }

  function isInternal(href) {
    if (!href) { return true; }
    var s = String(href).trim();
    if (s === '' || s.charAt(0) === '#') { return true; }
    var lower = s.toLowerCase();
    if (lower.indexOf('euv://') === 0) { return true; }
    if (lower.indexOf('about:') === 0
        || lower.indexOf('data:') === 0
        || lower.indexOf('blob:') === 0
        || lower.indexOf('javascript:') === 0) { return true; }
    var u;
    try { u = new URL(s, document.baseURI); } catch (e) { return true; }
    var scheme = (u.protocol || '').replace(':', '').toLowerCase();
    if (scheme === 'euv' || scheme === 'tauri' || scheme === 'asset'
        || scheme === 'about' || scheme === 'data' || scheme === 'blob'
        || scheme === 'javascript') { return true; }
    if (scheme === 'http' || scheme === 'https') {
      var host = (u.hostname || '').toLowerCase();
      if (host === 'localhost' || host === 'tauri.localhost'
          || host === '127.0.0.1' || host.indexOf('.localhost') !== -1
          && host.lastIndexOf('.localhost') === host.length - '.localhost'.length) {
        return true;
      }
      if (u.origin === window.location.origin) { return true; }
      return false;
    }
    return false;
  }

  function openExternal(href) {
    var url;
    try { url = new URL(String(href), document.baseURI).href; }
    catch (e) { url = String(href); }
    try {
      var core = window.bridge && window.bridge.core;
      if (core && typeof core.invoke === 'function') {
        core.invoke('plugin:opener|open_url', { url: url, with: null });
        return true;
      }
      if (window.__TAURI_INTERNALS__ && typeof window.__TAURI_INTERNALS__.invoke === 'function') {
        window.__TAURI_INTERNALS__.invoke('plugin:opener|open_url', { url: url, with: null });
        return true;
      }
    } catch (e) {}
    return false;
  }

  // 1) Intercept link clicks (covers target="_blank" and normal anchors).
  document.addEventListener('click', function (event) {
    if (event.defaultPrevented || event.button !== 0
        || event.metaKey || event.ctrlKey || event.shiftKey || event.altKey) { return; }
    var node = event.target;
    while (node && node.nodeName && node.nodeName.toLowerCase() !== 'a') {
      node = node.parentNode;
    }
    if (!node || node.nodeName.toLowerCase() !== 'a') { return; }
    var href = node.getAttribute('href') || node.href;
    if (isInternal(href)) { return; }
    if (openExternal(href)) {
      event.preventDefault();
      event.stopPropagation();
    }
  }, true);

  // 2) Intercept window.open for external URLs.
  var nativeOpen = window.open;
  window.open = function (url, name, features) {
    if (url && !isInternal(url)) {
      if (openExternal(url)) { return null; }
    }
    return nativeOpen ? nativeOpen.apply(window, arguments) : null;
  };
})();
"#;

/// JavaScript injected into **every** webview on each page load to reconcile the
/// loaded euv page with the app's immersive (edge-to-edge) window.
///
/// The app intentionally keeps immersive mode on: the WebView is laid out under
/// the system status bar, so the page must offset its top chrome by the real
/// `env(safe-area-inset-top)` value. The script:
///
/// 1. Sets `window.__EUV_IMMERSIVE__ = true` so euv-ui (>= 0.18.12) applies the
///    measured inset to `--euv-mobile-safe-top` itself at app init.
/// 2. Ensures the document viewport meta contains `viewport-fit=cover`, which is
///    required for `env(safe-area-inset-*)` to report real insets in the
///    WebView (the public euv templates ship without it because letterboxed
///    browsers must not consume the top inset).
/// 3. Measures `env(safe-area-inset-top)` through a probe element and writes it
///    to the `--euv-mobile-safe-top` CSS custom property on `<html>`, which
///    `c_mobile_header` / `c_mobile_nav_drawer` consume for their top padding.
/// 4. Measures the soft-keyboard inset through `visualViewport`: when the IME
///    opens, `visualViewport.height` shrinks and the difference against a
///    baseline window inner height is the keyboard height. The value is
///    written to `--euv-keyboard-height` on `<html>` and exposed as
///    `window.__EUV_KEYBOARD_HEIGHT__`. Both are kept in sync with the native
///    Android IME inset dispatch (`euv:keyboard-height` CustomEvent) when the
///    native side is available — native values always win because they
///    include navigation-bar / gesture-area compensation the visualViewport
///    derivation cannot.
/// 5. Injects a small compat stylesheet so cached copies of the page built with
///    euv-ui <= 0.18.11 (whose header does not consume the variable) still get
///    the correct padding, plus a keyboard-bottom helper that pads any element
///    marked `data-euv-keyboard-aware` so the focused input never sits behind
///    the soft keyboard even on pages that do not consume
///    `--euv-keyboard-height` themselves.
///
/// It re-runs on `DOMContentLoaded`, `load`, `resize`, `orientationchange`,
/// `focusin`, `focusout`, and `visualViewport.resize` so viewport edits made
/// by the page itself, device rotation, and IME state changes are handled.
///
/// Viewport work is coalesced into one frame and reuses one hidden probe.
pub(crate) const IMMERSIVE_SAFE_AREA_SCRIPT: &str = r#"
(function () {
  if (window.__euvImmersiveGuard) { return; }
  window.__euvImmersiveGuard = true;
  window.__EUV_IMMERSIVE__ = true;

  function ensureCover() {
    var metas = document.querySelectorAll('meta[name="viewport"]');
    if (!metas.length) {
      if (!document.head) { return; }
      var m = document.createElement('meta');
      m.name = 'viewport';
      m.content = 'width=device-width, initial-scale=1.0';
      document.head.appendChild(m);
      metas = [m];
    }
    Array.prototype.forEach.call(metas, function (m) {
      if (!/viewport-fit\s*=\s*cover/.test(m.content)) {
        m.content = m.content.replace(/\s*$/, '') + ', viewport-fit=cover';
      }
    });
  }

  var insetProbe = null;
  var insetPending = false;
  function applyInset() {
    insetPending = false;
    if (!document.documentElement) return;
    ensureCover();
    if (!insetProbe || !insetProbe.isConnected) {
      insetProbe = document.createElement('div');
      insetProbe.setAttribute('aria-hidden', 'true');
      var s = insetProbe.style;
      s.position = 'absolute';
      s.visibility = 'hidden';
      s.pointerEvents = 'none';
      s.paddingTop = 'env(safe-area-inset-top, 0px)';
      (document.body || document.documentElement).appendChild(insetProbe);
    }
    var top = (typeof window.__EUV_NATIVE_SAFE_TOP__ === 'number')
      ? (window.__EUV_NATIVE_SAFE_TOP__ + 'px')
      : getComputedStyle(insetProbe).paddingTop;
    var rootStyle = document.documentElement.style;
    if (rootStyle.getPropertyValue('--euv-mobile-safe-top') !== top) {
      rootStyle.setProperty('--euv-mobile-safe-top', top);
    }
  }
  function scheduleInset() {
    if (insetPending) return;
    insetPending = true;
    requestAnimationFrame(applyInset);
  }

  function injectCompat() {
    if (document.getElementById('euv-immersive-compat')) { return; }
    var st = document.createElement('style');
    st.id = 'euv-immersive-compat';
    st.textContent =
      '.c_mobile_header{padding-top:var(--euv-mobile-safe-top,0px)!important;' +
      'height:calc(var(--mobile-header-height,52px) + var(--euv-mobile-safe-top,0px))!important}' +
      '.c_mobile_nav_drawer{padding-top:var(--euv-mobile-safe-top,0px)!important}' +
      // Bottom-aware elements (modals, sticky toolbars, fixed bottom bars) opt
      // in to the keyboard inset by carrying `data-euv-keyboard-aware`. They
      // receive a transition-aware bottom padding that lifts them above the
      // soft keyboard without any framework change.
      '[data-euv-keyboard-aware]{padding-bottom:var(--euv-keyboard-height,0px)!important;' +
      'transition:padding-bottom 0.18s ease-out}';
    (document.head || document.documentElement).appendChild(st);
  }

  // Soft-keyboard inset bridge. Tracks visualViewport height against a
  // baseline window inner height so the page can react to IME show / hide.
  var baselineHeight = window.innerHeight;
  function applyKeyboard(h) {
    var px = (h > 0) ? (h + 'px') : '0px';
    document.documentElement.style.setProperty('--euv-keyboard-height', px);
    window.__EUV_KEYBOARD_HEIGHT__ = h;
  }
  applyKeyboard(0);

  // Native Android IME dispatch (MainActivity OnApplyWindowInsetsListener) wins
  // when it has reported anything; visualViewport is the fallback for iOS /
  // desktop / older Android builds where the native bridge is absent.
  window.addEventListener('euv:keyboard-height', function (e) {
    var h = (e && e.detail && typeof e.detail.height === 'number')
      ? e.detail.height : 0;
    if (h > 0) {
      baselineHeight = window.innerHeight;
    }
    applyKeyboard(h);
  });

  if (window.visualViewport) {
    var vv = window.visualViewport;
    function fromViewport() {
      if (typeof window.__EUV_KEYBOARD_HEIGHT__ === 'number'
          && window.__EUV_KEYBOARD_HEIGHT__ > 0) {
        // Native has authoritative value; do not override.
        baselineHeight = window.innerHeight;
        return;
      }
      var kb = Math.max(0, baselineHeight - vv.height - vv.offsetTop);
      applyKeyboard(Math.round(kb));
    }
    vv.addEventListener('resize', fromViewport);
    window.addEventListener('focusin', function () {
      baselineHeight = window.innerHeight;
      setTimeout(fromViewport, 0);
    });
    window.addEventListener('focusout', function () {
      if (typeof window.__EUV_KEYBOARD_HEIGHT__ !== 'number'
          || window.__EUV_KEYBOARD_HEIGHT__ === 0) {
        applyKeyboard(0);
      }
    });
  }

  // Honor any push from native that arrived before this script ran.
  if (typeof window.__euvInitialKeyboardHeight === 'number') {
    applyKeyboard(window.__euvInitialKeyboardHeight);
  }

  function boot() {
    try {
      scheduleInset();
      injectCompat();
    } catch (_) {}
  }

  document.addEventListener('DOMContentLoaded', boot);
  window.addEventListener('load', boot);
  window.addEventListener('resize', scheduleInset);
  window.addEventListener('orientationchange', scheduleInset);
  boot();
})();
"#;

/// Installs a lazy browser GPU capability probe without creating startup contexts.
pub(crate) const GPU_PROBE_SCRIPT: &str = include_str!("../../../resources/runtime/gpu-probe.js");

/// The filename used for the active version pointer symlink.
pub(crate) const ACTIVE_LINK: &str = "active";

/// The prefix for version directory names.
pub(crate) const VERSION_PREFIX: &str = "v_";

/// Interval in milliseconds between fetch retry attempts.
pub(crate) const RETRY_INTERVAL_MILLIS: u64 = 1000;

/// The custom URI scheme name used for serving cached resources.
pub(crate) const SCHEME_NAME: &str = "euv";

/// Native window label configured for Apple clients.
#[cfg(any(target_os = "ios", target_os = "macos"))]
pub(crate) const MAIN_WINDOW_LABEL: &str = "main";

/// Response body returned before the local cache is available.
pub(crate) const CACHE_NOT_READY_BODY: &[u8] = b"Cache not ready";

/// Response body returned when a cached resource does not exist.
pub(crate) const RESOURCE_NOT_FOUND_BODY: &[u8] = b"Not found";

/// Maximum number of old version directories to keep.
pub(crate) const MAX_KEPT_VERSIONS: usize = 2;

/// The entry point required before a resource snapshot is published.
pub(crate) const INDEX_FILE_NAME: &str = "index.html";

/// The readiness tag for a pinned session snapshot.
pub(crate) const SOURCE_ACTIVE: &str = "active";

/// The readiness tag used before initialization finishes.
pub(crate) const SOURCE_NONE: &str = "none";

/// Deadline for a complete remote response, including its body.
pub(crate) const REQUEST_TIMEOUT_SECS: u64 = 30;

/// WebAssembly resource suffix.
pub(crate) const WASM_EXTENSION: &str = ".wasm";

/// HTTP header carrying the response media type.
pub(crate) const CONTENT_TYPE_HEADER: &str = "Content-Type";

/// String tag emitted by native on a successful `update_cache`.
///
/// Must stay in lockstep with `CacheUpdateStatus::as_tag()` in the
/// euv-app bridge layer — both ends compare against the same literal so
/// a future renumbering / re-case here breaks the webview retry loop.
pub(crate) const UPDATE_RESULT_SUCCESS: &str = "success";

/// String tag emitted by native on a failed `update_cache`.
///
/// See `UPDATE_RESULT_SUCCESS` for the cross-side coupling note.
pub(crate) const UPDATE_RESULT_FAILED: &str = "failed";

/// JavaScript snippet injected into index.html to listen for reload events from the Tauri backend.
pub(crate) const RELOAD_LISTENER_SCRIPT: &str = r#"<script>
(function (){
  if(!window.bridge&&window.__TAURI__){window.bridge=window.__TAURI__;delete window.__TAURI__;}
  if(window.bridge&&window.bridge.event){
    window.bridge.event.listen('euv://reload',function(){
      window.location.reload();
    });
  } else {
    document.addEventListener('DOMContentLoaded',function(){
      var t=setInterval(function (){
        if(window.bridge&&window.bridge.event){
          clearInterval(t);
          window.bridge.event.listen('euv://reload',function(){
            window.location.reload();
          });
        }
      },100);
      setTimeout(function (){clearInterval(t);},10000);
    });
  }
})();
</script>"#;

/// URL scheme prefix for `http://` absolute URLs — skipped during resource discovery.
pub(crate) const HTTP_PREFIX: &str = "http://";

/// URL scheme prefix for `https://` absolute URLs — skipped during resource discovery.
pub(crate) const HTTPS_PREFIX: &str = "https://";

/// Inline-data URL prefix (`data:image/png;base64,...`) — skipped during resource discovery.
pub(crate) const DATA_PREFIX: &str = "data:";

/// Protocol-relative URL prefix (`//cdn.example.com/x.js`) — skipped during resource discovery.
pub(crate) const SCHEME_RELATIVE: &str = "//";

/// Authority separator embedded in absolute URLs (`https://host/path`) — any string containing
/// this is treated as absolute and skipped during resource discovery.
pub(crate) const SCHEME_SEPARATOR: &str = "://";

/// CSS selector that matches external `<script src="...">` tags.
pub(crate) const SCRIPT_SRC_SELECTOR: &str = "script[src]";

/// CSS selector that matches `<link href="...">` tags (stylesheets, favicons, preloads).
pub(crate) const LINK_HREF_SELECTOR: &str = "link[href]";

/// CSS selector that matches `<img src="...">` tags.
pub(crate) const IMG_SRC_SELECTOR: &str = "img[src]";

/// CSS selector that matches inline `<script>` (no `src` attribute) tags whose body should
/// be fed to the JS parser for AST-level resource extraction.
pub(crate) const INLINE_SCRIPT_SELECTOR: &str = "script:not([src])";

/// File-extension suffixes that identify a string literal as a local resource path to cache.
pub(crate) const RESOURCE_EXTENSIONS: &[&str] = &[
    ".wasm", ".js", ".mjs", ".css", ".json", ".png", ".jpg", ".jpeg", ".gif", ".svg", ".ico",
    ".woff", ".woff2", ".ttf", ".otf", ".webp",
];

/// Debug panel HTML snippet injected into index.html in debug builds.
///
/// Displays cache source information and a log viewer for real-time debugging.
#[cfg(debug_assertions)]
pub(crate) const DEBUG_PANEL_SCRIPT: &str = r#"<script>
(function (){
  var expanded=false;
  var bar=document.createElement('div');
  bar.id='__euv_debug_bar';
  bar.style.cssText='position:fixed;bottom:0;left:0;right:0;height:36px;background:#1a1a2e;color:#0f0;font:13px/36px monospace;padding:0 12px;z-index:2147483647;cursor:pointer;user-select:none;display:flex;align-items:center;box-shadow:0 -2px 8px rgba(0,0,0,0.5);';
  bar.textContent='\u25B2 [DEBUG] source: {{SOURCE}} | tap to expand';

  var panel=document.createElement('div');
  panel.id='__euv_debug_panel';
  panel.style.cssText='position:fixed;bottom:0;left:0;right:0;height:50vh;background:#0d0d1a;color:#0f0;font:12px monospace;z-index:2147483646;display:none;flex-direction:column;box-shadow:0 -4px 16px rgba(0,0,0,0.7);';

  var header=document.createElement('div');
  header.style.cssText='padding:8px 12px;background:#1a1a2e;border-bottom:1px solid #333;flex-shrink:0;display:flex;justify-content:space-between;align-items:center;';
  header.innerHTML='<span style="color:#0f0;font-weight:bold;">EUV Debug Console</span><span id="__euv_close" style="color:#f55;cursor:pointer;font-size:18px;">\u2716</span>';

  var info=document.createElement('div');
  info.style.cssText='padding:6px 12px;background:#111;border-bottom:1px solid #222;flex-shrink:0;color:#aaa;font-size:11px;word-break:break-all;';
  info.textContent='source: {{SOURCE}} | path: {{PATH}}';

  var logArea=document.createElement('div');
  logArea.id='__euv_debug_logs';
  logArea.style.cssText='flex:1;overflow-y:auto;padding:8px 12px;';

  panel.appendChild(header);
  panel.appendChild(info);
  panel.appendChild(logArea);

  function toggle(){
    expanded=!expanded;
    if(expanded){
      panel.style.display='flex';
      bar.style.bottom='50vh';
      bar.textContent='\u25BC [DEBUG] source: {{SOURCE}} | tap to collapse';
    } else {
      panel.style.display='none';
      bar.style.bottom='0';
      bar.textContent='\u25B2 [DEBUG] source: {{SOURCE}} | tap to expand';
    }
  }
  bar.addEventListener('click',toggle);

  function addLog(msg){
    var line=document.createElement('div');
    line.style.cssText='padding:2px 0;border-bottom:1px solid #1a1a2e;color:#0f0;word-break:break-all;';
    var now=new Date();
    var ts=now.getHours().toString().padStart(2,'0')+':'+now.getMinutes().toString().padStart(2,'0')+':'+now.getSeconds().toString().padStart(2,'0')+'.'+now.getMilliseconds().toString().padStart(3,'0');
    line.textContent='['+ts+'] '+msg;
    logArea.appendChild(line);
    logArea.scrollTop=logArea.scrollHeight;
  }

  function initListener(){
    if(!window.bridge&&window.__TAURI__){window.bridge=window.__TAURI__;delete window.__TAURI__;}
    if(window.bridge&&window.bridge.event){
      window.bridge.event.listen('euv://debug-log',function(e){
        addLog(e.payload||'');
      });
      addLog('[panel] listener registered');
      addLog('[panel] source: {{SOURCE}}');
      addLog('[panel] path: {{PATH}}');
    } else {
      var t=setInterval(function (){
        if(window.bridge&&window.bridge.event){
          clearInterval(t);
          window.bridge.event.listen('euv://debug-log',function(e){
            addLog(e.payload||'');
          });
          addLog('[panel] listener registered');
          addLog('[panel] source: {{SOURCE}}');
          addLog('[panel] path: {{PATH}}');
        }
      },100);
      setTimeout(function (){clearInterval(t);},10000);
    }
  }

  document.addEventListener('DOMContentLoaded',function(){
    document.body.appendChild(panel);
    document.body.appendChild(bar);
    document.getElementById('__euv_close').addEventListener('click',function(e){
      e.stopPropagation();
      toggle();
    });
    initListener();
  });
})();
</script>"#;
