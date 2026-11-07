include!(concat!(env!("OUT_DIR"), "/config_generated.rs"));
include!(concat!(env!("OUT_DIR"), "/bundled_cache_data.rs"));

/// The filename used for the active version pointer symlink.
pub(crate) const ACTIVE_LINK: &str = "active";

/// The prefix for version directory names.
pub(crate) const VERSION_PREFIX: &str = "v_";

/// Interval in milliseconds between fetch retry attempts.
pub(crate) const RETRY_INTERVAL_MILLIS: u64 = 1000;

/// The custom URI scheme name used for serving cached resources.
pub(crate) const SCHEME_NAME: &str = "euv";

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
