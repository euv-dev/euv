pub(crate) const INDEX_JSON_NAME: &str = "search-index.json";
pub(crate) const SITEMAP_XML_NAME: &str = "sitemap.xml";
pub(crate) const INDEX_HTML_NAME: &str = "index.html";
pub(crate) const HTML_EXT: &str = ".html";
pub(crate) const MARKDOWN_EXT_CHECK: &str = "md";
pub(crate) const MARKDOWN_EXT_SUFFIX: &str = ".md";
pub(crate) const MARKDOWN_EXT_UPPER: &str = ".MD";
pub(crate) const STEM_README_LOWER: &str = "readme";
pub(crate) const STEM_INDEX_LOWER: &str = "index";
pub(crate) const FENCE_BACKTICK: &str = "```";
pub(crate) const FENCE_TILDE: &str = "~~~";
pub(crate) const HTML_A_HREF_OPEN: &str = "<a href=\"";
pub(crate) const HTML_TITLE_ATTR_OPEN: &str = " title=\"";
pub(crate) const HTML_EXTERNAL_LINK_ATTRS: &str = " target=\"_blank\" rel=\"noopener noreferrer\"";
pub(crate) const HTML_DETAILS_CLOSE: &str = "</details>";
pub(crate) const HTML_DIV_CLOSE: &str = "</div>";
pub(crate) const ROOT_ROUTE_STR: &str = "/";
pub(crate) const EMPTY_JSON_ARRAY: &str = "[]";
pub(crate) const EMPTY_JSON_STRING: &str = "\"\"";
pub(crate) const TOC_LVL3_CLASS: &str = "lvl3";
pub(crate) const TOC_LVL4_CLASS: &str = "lvl4";
pub(crate) const DEFAULT_BRAND_MONOGRAM: char = 'E';
pub(crate) const DEFAULT_CODE_LABEL: &str = "text";
pub(crate) const ERR_NO_MARKDOWN_FOUND: &str =
    "No Markdown (.md) files discovered under source directory";
pub(crate) const CALLOUT_FENCE: &str = ":::";
pub(crate) const CALLOUT_KIND_TIP: &str = "tip";
pub(crate) const CALLOUT_KIND_WARNING: &str = "warning";
pub(crate) const CALLOUT_KIND_DANGER: &str = "danger";
pub(crate) const CALLOUT_KIND_INFO: &str = "info";
pub(crate) const CALLOUT_KIND_NOTE: &str = "note";
pub(crate) const CALLOUT_KIND_IMPORTANT: &str = "important";
pub(crate) const CALLOUT_KIND_CAUTION: &str = "caution";
pub(crate) const CALLOUT_KIND_DETAILS: &str = "details";
pub(crate) const CALLOUT_TITLE_TIP: &str = "Tip";
pub(crate) const CALLOUT_TITLE_WARNING: &str = "Warning";
pub(crate) const CALLOUT_TITLE_DANGER: &str = "Danger";
pub(crate) const CALLOUT_TITLE_INFO: &str = "Info";
pub(crate) const CALLOUT_TITLE_NOTE: &str = "Note";
pub(crate) const CALLOUT_TITLE_IMPORTANT: &str = "Important";
pub(crate) const CALLOUT_TITLE_CAUTION: &str = "Caution";
pub(crate) const CALLOUT_TITLE_DETAILS: &str = "Details";
pub(crate) const SCHEME_HTTP: &str = "http://";
pub(crate) const SCHEME_HTTPS: &str = "https://";
pub(crate) const SCHEME_MAILTO: &str = "mailto:";
pub(crate) const SCHEME_DATA: &str = "data:";
pub(crate) const SCHEME_JAVASCRIPT: &str = "javascript:";
pub(crate) const SCHEME_VBSCRIPT: &str = "vbscript:";
pub(crate) const UNSAFE_HTML_TAGS: [&str; 4] = ["<script", "<iframe", "<object", "<embed"];
pub(crate) const ASSET_COPY_DIRS: [&str; 6] = [
    "img",
    "images",
    "assets",
    "public",
    "static",
    "markdown-images",
];
pub(crate) const CO_LOCATED_ASSET_EXTS: [&str; 12] = [
    "png", "jpg", "jpeg", "gif", "svg", "webp", "ico", "avif", "mp4", "webm", "pdf", "woff2",
];
pub(crate) const SITEMAP_HEADER: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n";
pub(crate) const SITEMAP_FOOTER: &str = "</urlset>\n";
pub(crate) const SITEMAP_URL_OPEN: &str = "  <url><loc>";
pub(crate) const SITEMAP_URL_CLOSE: &str = "</loc></url>\n";
pub(crate) const HTML_DOCTYPE: &str = "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><link rel=\"icon\" href=\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 32 32'%3E%3Crect width='32' height='32' rx='8' fill='%2318181b'/%3E%3Ctext x='16' y='22' text-anchor='middle' font-family='sans-serif' font-weight='800' font-size='18' fill='%23fafafa'%3EE%3C/text%3E%3C/svg%3E\"><script>(function(){try{var k='euv-docs-theme',s=localStorage.getItem(k);if(s==='dark'||(!s&&window.matchMedia&&window.matchMedia('(prefers-color-scheme: dark)').matches)){document.documentElement.setAttribute('data-theme','dark');}}catch(e){}})();</script><title>";
pub(crate) const HTML_TITLE_SEP: &str = " - ";
pub(crate) const HTML_STYLES_AND_HEAD_END: &str = r#"</title><style>
:root{--bg:#ffffff;--fg:#09090b;--muted:#f4f4f5;--muted-fg:#71717a;--card:#ffffff;--border:#e4e4e7;--accent:#18181b;--accent-fg:#fafafa;--code-bg:#EBEEF5;--code-head:#E1E4EB;--code-border:#CED1D7;--code-fg:#5D5D5F;--code-ln:#B3B3B4;--inline-code-bg:#E5E8EF;--inline-code-fg:#53A053;--tok-kw:#7C4DFF;--tok-ctrl:#C838C6;--tok-op:#00BEC4;--tok-fn:#437AED;--tok-ty:#00BEC4;--tok-prim:#EF8ED8;--tok-str:#53A053;--tok-num:#FF6D12;--tok-cmt:#A9A9AA;--tok-mac:#437AED;--tok-prop:#CD6069;--tok-const:#C838C6;--tok-punc:#A0A1A7;--tok-add:#44C145;--tok-del:#F14C4C;--link:#2563eb;--radius:10px;--shadow:0 1px 2px rgba(0,0,0,.05);--tip:#16a34a;--warn:#d97706;--danger:#dc2626;--info:#2563eb;--important:#7c3aed}
[data-theme="dark"]{--bg:#09090b;--fg:#fafafa;--muted:#18181b;--muted-fg:#a1a1aa;--card:#111114;--border:#27272a;--accent:#fafafa;--accent-fg:#09090b;--code-bg:#282C34;--code-head:#21252B;--code-border:#343A46;--code-fg:#B0B7C3;--code-ln:#535773;--inline-code-bg:#181A1F;--inline-code-fg:#98C379;--tok-kw:#A78CFA;--tok-ctrl:#CF68E1;--tok-op:#56B7C3;--tok-fn:#6495EE;--tok-ty:#56B7C3;--tok-prim:#FF6AB3;--tok-str:#98C379;--tok-num:#FF9070;--tok-cmt:#676E95;--tok-mac:#6495EE;--tok-prop:#E06C75;--tok-const:#CF68E1;--tok-punc:#79859D;--tok-add:#4EC150;--tok-del:#F14C4C;--link:#60a5fa;--tip:#4ade80;--warn:#fbbf24;--danger:#f87171;--info:#60a5fa;--important:#a78bfa}
*{box-sizing:border-box}html,body{margin:0;padding:0;background:var(--bg);color:var(--fg);font-family:Inter,ui-sans-serif,system-ui,-apple-system,Segoe UI,Roboto,sans-serif;line-height:1.68;-webkit-font-smoothing:antialiased;overflow-x:clip}
a{color:var(--link);text-decoration:none}a:hover{text-decoration:underline}
.topbar{position:sticky;top:0;z-index:40;height:56px;display:flex;align-items:center;justify-content:space-between;gap:12px;padding:0 20px;background:color-mix(in srgb,var(--bg) 90%,transparent);backdrop-filter:blur(10px);border-bottom:1px solid var(--border)}
.brand{display:flex;align-items:center;gap:10px;font-weight:700;font-size:15px;color:var(--fg);text-decoration:none;min-width:0}
.brand:hover{text-decoration:none}
.brand-mark{width:26px;height:26px;flex:0 0 26px;border-radius:7px;background:var(--accent);color:var(--accent-fg);display:inline-flex;align-items:center;justify-content:center;font-weight:800;font-size:13px}
.brand-name{overflow:hidden;text-overflow:ellipsis;white-space:nowrap}
.top-actions{display:flex;align-items:center;gap:8px;flex-shrink:0}
.search-wrap{position:relative;width:240px;max-width:44vw}
.search-input{width:100%;height:34px;padding:0 12px;border-radius:8px;border:1px solid var(--border);background:var(--muted);color:var(--fg);font-size:13px;outline:none}
.search-input:focus{border-color:var(--accent);background:var(--bg)}
.search-results{position:absolute;top:40px;right:0;width:360px;max-width:calc(100vw - 24px);max-height:380px;overflow:auto;background:var(--card);border:1px solid var(--border);border-radius:var(--radius);box-shadow:0 12px 32px rgba(0,0,0,.16);display:none;z-index:60;padding:6px}
.search-results.open{display:block}
.search-item{display:block;padding:8px 10px;border-radius:7px;color:var(--fg);text-decoration:none}
.search-item:hover,.search-item.selected{background:var(--muted);text-decoration:none}
.search-item-title{font-weight:600;font-size:13px}.search-item-meta{font-size:11.5px;color:var(--muted-fg)}.search-item-snip{font-size:12px;color:var(--muted-fg);margin-top:2px;display:-webkit-box;-webkit-line-clamp:2;-webkit-box-orient:vertical;overflow:hidden}
.btn{height:34px;padding:0 11px;border-radius:8px;border:1px solid var(--border);background:var(--card);color:var(--fg);font-size:13px;font-weight:500;cursor:pointer;display:inline-flex;align-items:center;gap:6px;white-space:nowrap}
.btn:hover{background:var(--muted)}.menu-btn{display:none}
.backdrop{display:none;position:fixed;inset:56px 0 0 0;background:rgba(0,0,0,.42);z-index:45;backdrop-filter:blur(2px)}
.backdrop.open{display:block}
.shell{max-width:1380px;margin:0 auto;display:grid;grid-template-columns:256px minmax(0,1fr) 220px;min-height:calc(100vh - 56px)}
.shell.no-toc{grid-template-columns:256px minmax(0,1fr)}
.sidebar{position:sticky;top:56px;height:calc(100vh - 56px);overflow:auto;padding:20px 14px;border-right:1px solid var(--border);background:var(--bg)}
.sec-title{font-size:11px;font-weight:700;text-transform:uppercase;letter-spacing:.06em;color:var(--muted-fg);padding:12px 10px 6px}
.nav-link{display:block;padding:7px 10px;border-radius:7px;font-size:13.5px;color:var(--muted-fg);margin-bottom:2px;white-space:nowrap;overflow:hidden;text-overflow:ellipsis}
.nav-link:hover{background:var(--muted);color:var(--fg);text-decoration:none}
.nav-link.active{background:var(--muted);color:var(--fg);font-weight:600;border-left:2px solid var(--accent)}
.content{min-width:0;padding:32px 44px 56px;max-width:860px;width:100%;margin:0 auto;overflow-wrap:anywhere}
.badge{display:inline-block;padding:2px 10px;border-radius:999px;background:var(--muted);border:1px solid var(--border);color:var(--muted-fg);font-size:12px;font-weight:600;margin-bottom:10px}
.content h1{font-size:32px;line-height:1.2;margin:0 0 10px;letter-spacing:-.02em}
.lead{font-size:15.5px;color:var(--muted-fg);margin:0 0 24px}
.content h2{font-size:22px;margin:30px 0 10px;padding-bottom:6px;border-bottom:1px solid var(--border);scroll-margin-top:72px}
.content h3{font-size:17px;margin:22px 0 8px;scroll-margin-top:72px}
.content h4,.content h5,.content h6{font-size:15px;margin:18px 0 6px;scroll-margin-top:72px}
.anchor{opacity:0;margin-left:8px;font-weight:500;font-size:.85em;color:var(--muted-fg);transition:opacity .12s}
.content h2:hover .anchor,.content h3:hover .anchor,.content h4:hover .anchor,.anchor:focus{opacity:1;text-decoration:none}
.content p{margin:12px 0;font-size:15px}
.content ul,.content ol{margin:10px 0 14px;padding-left:22px}.content li{margin:5px 0;font-size:15px}
.content li>ul,.content li>ol{margin:4px 0}
.task-box{margin-right:6px;vertical-align:middle;accent-color:var(--tok-kw)}
.content code{display:inline-block;vertical-align:baseline;text-align:center;line-height:1.35;padding:1px 6px;border-radius:5px;background:var(--inline-code-bg);border:1px solid var(--code-border);color:var(--inline-code-fg);font-size:.86em;font-family:'MesloLGS Nerd Font Mono','JetBrains Mono','SF Mono',Menlo,Monaco,Consolas,ui-monospace,monospace;word-break:break-word;box-decoration-break:clone}
.content kbd{display:inline-flex;align-items:center;justify-content:center;vertical-align:baseline;text-align:center;line-height:1.35;padding:1px 6px;border-radius:5px;border:1px solid var(--border);background:var(--muted);font-size:.82em;font-family:'MesloLGS Nerd Font Mono','JetBrains Mono','SF Mono',Menlo,Monaco,Consolas,ui-monospace,monospace;box-shadow:inset 0 -1px 0 var(--border)}
.content mark{padding:1px 5px;border-radius:4px;background:color-mix(in srgb,var(--warn) 25%,transparent);color:var(--fg)}
.content sup,.content sub{font-size:.76em;line-height:0}
.content dl{margin:16px 0;padding:12px 16px;border:1px solid var(--border);border-radius:var(--radius);background:var(--card)}
.content dt{font-weight:700;font-size:14.5px;margin-top:10px;color:var(--fg)}.content dt:first-child{margin-top:0}
.content dd{margin:4px 0 8px 16px;font-size:14px;color:var(--muted-fg)}
.math-inline{padding:2px 7px;border-radius:5px;background:var(--muted);color:var(--tok-fn);font-style:italic}
.math-display{margin:16px 0;padding:14px 18px;border:1px solid var(--border);border-radius:var(--radius);background:var(--muted);text-align:center;overflow-x:auto}
.math-display code{border:0;background:transparent;font-size:14.5px;color:var(--tok-fn)}
.content details{margin:16px 0;padding:12px 16px;border:1px solid var(--border);border-radius:var(--radius);background:var(--muted)}
.content details>summary{cursor:pointer;font-weight:600;font-size:14.5px}
.content img{max-width:100%;border-radius:8px;vertical-align:middle}
.content img:not([height]){height:auto}
.content blockquote{margin:16px 0;padding:10px 16px;border-left:3px solid var(--accent);background:var(--muted);border-radius:0 8px 8px 0;color:var(--muted-fg)}
.content blockquote>p:first-child{margin-top:0}.content blockquote>p:last-child{margin-bottom:0}
.callout{margin:16px 0;padding:12px 16px;border:1px solid var(--border);border-left:4px solid var(--info);border-radius:8px;background:var(--muted)}
.callout-title{font-weight:700;font-size:12.5px;margin-bottom:4px;text-transform:uppercase;letter-spacing:.05em}
.callout-tip{border-left-color:var(--tip)}.callout-tip .callout-title{color:var(--tip)}
.callout-warning{border-left-color:var(--warn)}.callout-warning .callout-title{color:var(--warn)}
.callout-danger,.callout-caution{border-left-color:var(--danger)}.callout-danger .callout-title,.callout-caution .callout-title{color:var(--danger)}
.callout-note,.callout-info{border-left-color:var(--info)}.callout-note .callout-title,.callout-info .callout-title{color:var(--info)}
.callout-important{border-left-color:var(--important)}.callout-important .callout-title{color:var(--important)}
.callout-details summary{cursor:pointer;font-weight:600;font-size:14px;list-style:none}.callout-details summary::-webkit-details-marker{display:none}
.content hr{border:0;border-top:1px solid var(--border);margin:24px 0}
.code-wrap{position:relative;margin:16px 0;border:1px solid var(--code-border);border-radius:var(--radius);overflow:hidden;background:var(--code-bg);max-width:100%}
.code-head{display:flex;justify-content:space-between;align-items:center;padding:7px 14px;background:var(--code-head);border-bottom:1px solid var(--code-border);font-size:11.5px;font-weight:600;color:var(--code-fg)}
.copy-btn{border:1px solid var(--code-border);background:var(--card);color:var(--fg);border-radius:6px;padding:2px 9px;font-size:11px;font-weight:600;cursor:pointer}
.copy-btn:hover{background:var(--muted)}
.code-wrap pre{margin:0;padding:12px 0;overflow-x:auto;font-size:13.5px;line-height:1.58;tab-size:2;font-family:'MesloLGS Nerd Font Mono','JetBrains Mono','SF Mono',Menlo,Monaco,Consolas,ui-monospace,monospace;color:var(--code-fg);max-width:100%}
.code-wrap pre code{display:block;padding:0;border:0;background:transparent;color:inherit;font-size:inherit;text-align:left;line-height:inherit;word-break:normal}
.code-line{display:flex;padding:0 16px 0 0;min-height:1.58em}
.code-ln{flex:0 0 42px;padding-right:12px;text-align:right;color:var(--code-ln);user-select:none;opacity:.85}
.code-txt{flex:1 1 auto;white-space:pre}
.tok-kw{color:var(--tok-kw);font-weight:700}.tok-ctrl{color:var(--tok-ctrl);font-weight:700}.tok-op{color:var(--tok-op);font-weight:400}.tok-fn{color:var(--tok-fn);font-weight:700}.tok-ty{color:var(--tok-ty);font-weight:400}.tok-prim{color:var(--tok-prim);font-weight:400}.tok-str{color:var(--tok-str);font-weight:400}.tok-num{color:var(--tok-num);font-weight:400}.tok-cmt{color:var(--tok-cmt);font-style:italic;font-weight:400}.tok-mac{color:var(--tok-mac);font-weight:700}.tok-prop{color:var(--tok-prop);font-weight:400}.tok-const{color:var(--tok-const);font-weight:700}.tok-punc{color:var(--tok-punc);font-weight:400}.tok-add{color:var(--tok-add);font-weight:400}.tok-del{color:var(--tok-del);font-weight:400}
.table-wrap{overflow-x:auto;-webkit-overflow-scrolling:touch;margin:16px 0;border:1px solid var(--border);border-radius:var(--radius);max-width:100%}
table{width:100%;border-collapse:collapse;font-size:13.5px}th,td{padding:10px 12px;text-align:left;border-bottom:1px solid var(--border)}th{background:var(--muted);font-weight:600}tbody tr:last-child td{border-bottom:0}
.pager{display:grid;grid-template-columns:1fr 1fr;gap:12px;margin-top:36px;padding-top:20px;border-top:1px solid var(--border)}
.pager-card{display:block;padding:12px 14px;border:1px solid var(--border);border-radius:var(--radius);background:var(--card);color:var(--fg);min-width:0}
.pager-card:hover{border-color:var(--accent);text-decoration:none}.pager-label{font-size:11.5px;color:var(--muted-fg)}.pager-title{font-weight:600;font-size:14px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap}.pager-next{text-align:right}
.toc{position:sticky;top:56px;height:calc(100vh - 56px);overflow:auto;padding:24px 16px;border-left:1px solid var(--border)}
.toc-title{font-size:11.5px;font-weight:700;text-transform:uppercase;letter-spacing:.05em;color:var(--muted-fg);margin-bottom:10px}
.toc a{display:block;font-size:12.5px;color:var(--muted-fg);padding:4px 0 4px 8px;border-left:2px solid transparent;white-space:nowrap;overflow:hidden;text-overflow:ellipsis}
.toc a:hover{color:var(--fg);text-decoration:none}.toc a.active{color:var(--fg);font-weight:600;border-left-color:var(--accent)}.toc a.lvl3{padding-left:18px}.toc a.lvl4{padding-left:28px}
footer{margin-top:40px;padding-top:16px;border-top:1px solid var(--border);font-size:12.5px;color:var(--muted-fg)}
.hmr-err{position:fixed;bottom:16px;right:16px;max-width:min(420px,calc(100vw - 32px));padding:12px 14px;border-radius:10px;background:#7f1d1d;color:#fef2f2;font-size:12.5px;z-index:90;box-shadow:0 10px 28px rgba(0,0,0,.35)}
@media(max-width:1100px){.shell,.shell.no-toc{grid-template-columns:240px minmax(0,1fr)}.toc{display:none}}
@media(max-width:780px){.topbar{padding:0 12px;gap:8px}.brand-name{max-width:112px}.search-wrap{width:124px;max-width:34vw}.btn{padding:0 9px;font-size:12px}.shell,.shell.no-toc{grid-template-columns:minmax(0,1fr)}.menu-btn{display:inline-flex}.sidebar{display:none;position:fixed;left:0;top:56px;width:272px;max-width:82vw;z-index:50;box-shadow:0 12px 32px rgba(0,0,0,.22)}.sidebar.open{display:block}.content{padding:20px 14px 40px;max-width:100%;overflow-x:clip}.content h1{font-size:26px}.pager{grid-template-columns:1fr}}
</style></head><body>"#;
pub(crate) const HTML_TOPBAR_START: &str = "<header class=\"topbar\"><a class=\"brand\" href=\"";
pub(crate) const HTML_TOPBAR_MARK_OPEN: &str = "\"><span class=\"brand-mark\">";
pub(crate) const HTML_TOPBAR_MARK_CLOSE: &str = "</span><span class=\"brand-name\">";
pub(crate) const HTML_TOPBAR_END: &str = "</span></a><div class=\"top-actions\"><div class=\"search-wrap\"><input id=\"euv-search\" class=\"search-input\" type=\"search\" placeholder=\"Search... (/)\" autocomplete=\"off\" aria-label=\"Search documentation\"><div id=\"euv-search-res\" class=\"search-results\" role=\"listbox\"></div></div><button id=\"euv-theme\" class=\"btn\" type=\"button\" aria-label=\"Toggle theme\">Theme</button><button id=\"euv-menu\" class=\"btn menu-btn\" type=\"button\" aria-expanded=\"false\" aria-controls=\"euv-sidebar\">Menu</button></div></header><div id=\"euv-backdrop\" class=\"backdrop\"></div>";
pub(crate) const HTML_SHELL_OPEN: &str = "<div class=\"shell\">";
pub(crate) const HTML_SHELL_NO_TOC_OPEN: &str = "<div class=\"shell no-toc\">";
pub(crate) const HTML_SIDEBAR_OPEN: &str =
    "<aside id=\"euv-sidebar\" class=\"sidebar\" aria-label=\"Sidebar navigation\">";
pub(crate) const HTML_SIDEBAR_CLOSE: &str = "</aside><main class=\"content\">";
pub(crate) const HTML_SEC_OPEN: &str = "<div class=\"sec-title\">";
pub(crate) const HTML_SEC_CLOSE: &str = "</div>";
pub(crate) const HTML_NAV_LINK_OPEN: &str = "<a class=\"nav-link\" href=\"";
pub(crate) const HTML_NAV_LINK_ACTIVE_OPEN: &str =
    "<a class=\"nav-link active\" aria-current=\"page\" href=\"";
pub(crate) const HTML_LINK_MID: &str = "\">";
pub(crate) const HTML_LINK_CLOSE: &str = "</a>";
pub(crate) const HTML_BADGE_OPEN: &str = "<div class=\"badge\">";
pub(crate) const HTML_BADGE_CLOSE: &str = "</div><h1>";
pub(crate) const HTML_H1_CLOSE: &str = "</h1>";
pub(crate) const HTML_LEAD_OPEN: &str = "<p class=\"lead\">";
pub(crate) const HTML_P_OPEN: &str = "<p>";
pub(crate) const HTML_P_CLOSE: &str = "</p>";
pub(crate) const HTML_UL_OPEN: &str = "<ul>";
pub(crate) const HTML_UL_CLOSE: &str = "</ul>";
pub(crate) const HTML_OL_OPEN: &str = "<ol>";
pub(crate) const HTML_OL_CLOSE: &str = "</ol>";
pub(crate) const HTML_LI_OPEN: &str = "<li>";
pub(crate) const HTML_LI_CLOSE: &str = "</li>";
pub(crate) const HTML_STRONG_OPEN: &str = "<strong>";
pub(crate) const HTML_STRONG_CLOSE: &str = "</strong>";
pub(crate) const HTML_EM_OPEN: &str = "<em>";
pub(crate) const HTML_EM_CLOSE: &str = "</em>";
pub(crate) const HTML_DEL_OPEN: &str = "<del>";
pub(crate) const HTML_DEL_CLOSE: &str = "</del>";
pub(crate) const HTML_INLINE_CODE_OPEN: &str = "<code>";
pub(crate) const HTML_INLINE_CODE_CLOSE: &str = "</code>";
pub(crate) const HTML_BR: &str = "<br>";
pub(crate) const HTML_TASK_CHECKED: &str =
    "<input class=\"task-box\" type=\"checkbox\" disabled checked>";
pub(crate) const HTML_TASK_UNCHECKED: &str =
    "<input class=\"task-box\" type=\"checkbox\" disabled>";
pub(crate) const HTML_QUOTE_OPEN: &str = "<blockquote>";
pub(crate) const HTML_QUOTE_CLOSE: &str = "</blockquote>";
pub(crate) const HTML_HR: &str = "<hr>";
pub(crate) const HTML_CODE_WRAP_OPEN: &str =
    "<div class=\"code-wrap\"><div class=\"code-head\"><span>";
pub(crate) const HTML_CODE_HEAD_MID: &str = "</span><button class=\"copy-btn\" type=\"button\">Copy</button></div><pre><code class=\"language-";
pub(crate) const HTML_CODE_CLOSE: &str = "</code></pre></div>";
pub(crate) const HTML_TABLE_OPEN: &str = "<div class=\"table-wrap\"><table>";
pub(crate) const HTML_THEAD_OPEN: &str = "<thead><tr>";
pub(crate) const HTML_THEAD_CLOSE: &str = "</tr></thead>";
pub(crate) const HTML_TBODY_OPEN: &str = "<tbody>";
pub(crate) const HTML_TBODY_CLOSE: &str = "</tbody>";
pub(crate) const HTML_TR_OPEN: &str = "<tr>";
pub(crate) const HTML_TR_CLOSE: &str = "</tr>";
pub(crate) const HTML_TH_OPEN: &str = "<th>";
pub(crate) const HTML_TH_CLOSE: &str = "</th>";
pub(crate) const HTML_TD_OPEN: &str = "<td>";
pub(crate) const HTML_TD_CLOSE: &str = "</td>";
pub(crate) const HTML_TABLE_CLOSE: &str = "</table></div>";
pub(crate) const HTML_ALIGN_CENTER: &str = " style=\"text-align:center\"";
pub(crate) const HTML_ALIGN_RIGHT: &str = " style=\"text-align:right\"";
pub(crate) const HTML_PAGER_OPEN: &str = "<nav class=\"pager\" aria-label=\"Pagination\">";
pub(crate) const HTML_PAGER_PREV_OPEN: &str = "<a class=\"pager-card\" href=\"";
pub(crate) const HTML_PAGER_PREV_LABEL: &str =
    "\"><div class=\"pager-label\">Previous</div><div class=\"pager-title\">";
pub(crate) const HTML_PAGER_NEXT_OPEN: &str = "<a class=\"pager-card pager-next\" href=\"";
pub(crate) const HTML_PAGER_NEXT_LABEL: &str =
    "\"><div class=\"pager-label\">Next</div><div class=\"pager-title\">";
pub(crate) const HTML_PAGER_CARD_CLOSE: &str = "</div></a>";
pub(crate) const HTML_PAGER_EMPTY: &str = "<div></div>";
pub(crate) const HTML_PAGER_CLOSE: &str = "</nav>";
pub(crate) const HTML_FOOTER_PREFIX: &str = "<footer>Generated by Euv Docs &middot; ";
pub(crate) const HTML_FOOTER_SUFFIX: &str = "</footer></main>";
pub(crate) const HTML_TOC_OPEN: &str = "<aside class=\"toc\" aria-label=\"Table of contents\"><div class=\"toc-title\">On this page</div>";
pub(crate) const HTML_TOC_CLOSE: &str = "</aside>";
pub(crate) const HTML_SHELL_CLOSE: &str = "</div>";
pub(crate) const HTML_SCRIPT_PREFIX: &str = "<script>window.__EUV_DOCS_INDEX__=";
pub(crate) const HTML_SCRIPT_BASE_PREFIX: &str = ";window.__EUV_DOCS_BASE__=";
pub(crate) const HTML_SCRIPT_BODY: &str = r##";(function(){
var root=document.documentElement,tKey='euv-docs-theme',tBtn=document.getElementById('euv-theme');
function syncThemeBtn(){if(!tBtn)return;var d=root.getAttribute('data-theme')==='dark';tBtn.textContent=d?'Light':'Dark';}
syncThemeBtn();
if(tBtn){tBtn.onclick=function(){var d=root.getAttribute('data-theme')==='dark';if(d){root.removeAttribute('data-theme');try{localStorage.setItem(tKey,'light');}catch(e){}}else{root.setAttribute('data-theme','dark');try{localStorage.setItem(tKey,'dark');}catch(e){}}syncThemeBtn();};}
var mBtn=document.getElementById('euv-menu'),sb=document.getElementById('euv-sidebar'),bd=document.getElementById('euv-backdrop');
function closeNav(){if(sb)sb.classList.remove('open');if(bd)bd.classList.remove('open');if(mBtn)mBtn.setAttribute('aria-expanded','false');}
function toggleNav(){if(!sb)return;var open=sb.classList.toggle('open');if(bd)bd.classList.toggle('open',open);if(mBtn)mBtn.setAttribute('aria-expanded',open?'true':'false');}
if(mBtn){mBtn.onclick=toggleNav;}
if(bd){bd.onclick=closeNav;}
if(sb){sb.querySelectorAll('a').forEach(function(a){a.addEventListener('click',closeNav);});}
document.querySelectorAll('.copy-btn').forEach(function(b){b.onclick=function(){var w=b.closest('.code-wrap'),c=w&&w.querySelector('code');if(!c)return;var lines=c.querySelectorAll('.code-txt');var txt=lines.length?Array.prototype.map.call(lines,function(n){return n.textContent||'';}).join('\n'):(c.textContent||'');var done=function(){b.textContent='Copied';setTimeout(function(){b.textContent='Copy';},1200);};if(navigator.clipboard&&navigator.clipboard.writeText){navigator.clipboard.writeText(txt).then(done,function(){fallbackCopy(txt,done);});}else{fallbackCopy(txt,done);}};});
function fallbackCopy(t,cb){try{var ta=document.createElement('textarea');ta.value=t;ta.style.position='fixed';ta.style.opacity='0';document.body.appendChild(ta);ta.select();document.execCommand('copy');document.body.removeChild(ta);cb();}catch(e){}}
var inp=document.getElementById('euv-search'),res=document.getElementById('euv-search-res'),idx=window.__EUV_DOCS_INDEX__||[],base=window.__EUV_DOCS_BASE__||'',sel=-1;
function routeHref(r){if(!r||r==='/')return base?base+'/':'/';return base+r+'/';}
function clearEl(el){while(el.firstChild){el.removeChild(el.firstChild);}}
function updateSel(){var items=res?res.querySelectorAll('a.search-item'):[];items.forEach(function(el,i){el.classList.toggle('selected',i===sel);});}
function renderHits(q){var s=(q||'').trim().toLowerCase();sel=-1;clearEl(res);if(!s){res.classList.remove('open');return;}
var hits=idx.filter(function(p){var hay=(p.title+' '+p.section+' '+p.summary+' '+(p.headings||[]).join(' ')+' '+p.excerpt).toLowerCase();return hay.indexOf(s)!==-1;}).slice(0,12);
if(!hits.length){var empty=document.createElement('div');empty.className='search-item';var em=document.createElement('div');em.className='search-item-meta';em.textContent='No matching pages';empty.appendChild(em);res.appendChild(empty);}else{hits.forEach(function(p){var a=document.createElement('a');a.className='search-item';a.href=routeHref(p.route);var t=document.createElement('div');t.className='search-item-title';t.textContent=p.title||'';var m=document.createElement('div');m.className='search-item-meta';m.textContent=(p.section||'')+' · '+(p.route||'/');a.appendChild(t);a.appendChild(m);var sn=p.summary||p.excerpt||'';if(sn){var d=document.createElement('div');d.className='search-item-snip';d.textContent=sn.slice(0,120);a.appendChild(d);}res.appendChild(a);});}
res.classList.add('open');}
if(inp&&res){inp.addEventListener('input',function(){renderHits(inp.value);});inp.addEventListener('keydown',function(e){var items=res.querySelectorAll('a.search-item');if(e.key==='ArrowDown'&&items.length){e.preventDefault();sel=(sel+1)%items.length;updateSel();}else if(e.key==='ArrowUp'&&items.length){e.preventDefault();sel=(sel<=0?items.length-1:sel-1);updateSel();}else if(e.key==='Enter'&&sel>=0&&items[sel]){e.preventDefault();items[sel].click();}else if(e.key==='Escape'){res.classList.remove('open');inp.blur();}});document.addEventListener('keydown',function(e){if((e.key==='/'||(e.key.toLowerCase()==='k'&&(e.metaKey||e.ctrlKey)))&&document.activeElement!==inp){e.preventDefault();inp.focus();inp.select();}else if(e.key==='Escape'){closeNav();res.classList.remove('open');}});document.addEventListener('click',function(e){if(!e.target.closest('.search-wrap'))res.classList.remove('open');});}
var tocLinks=Array.prototype.slice.call(document.querySelectorAll('.toc a[href^="#"]'));
if(tocLinks.length&&window.IntersectionObserver){var map={};tocLinks.forEach(function(a){var id=decodeURIComponent(a.getAttribute('href').slice(1));var h=document.getElementById(id);if(h)map[id]={a:a,h:h};});var io=new IntersectionObserver(function(entries){entries.forEach(function(en){if(en.isIntersecting&&map[en.target.id]){tocLinks.forEach(function(l){l.classList.remove('active');});map[en.target.id].a.classList.add('active');}});},{rootMargin:'-64px 0px -70% 0px'});Object.keys(map).forEach(function(id){io.observe(map[id].h);});}
})();</script>"##;
pub(crate) const HTML_HMR_PREFIX: &str = "<script>(function(){try{var es=new EventSource('";
pub(crate) const HTML_HMR_SUFFIX: &str = "');var showErr=function(msg){var el=document.getElementById('euv-hmr-err');if(!el){el=document.createElement('div');el.id='euv-hmr-err';el.className='hmr-err';document.body.appendChild(el);}el.textContent='Build error: '+msg;};es.addEventListener('reload',function(){location.reload();});es.addEventListener('ota',function(){location.reload();});es.addEventListener('error',function(ev){if(ev&&ev.data){try{var d=JSON.parse(ev.data);showErr(d.message||ev.data);}catch(e){showErr(ev.data);}}});}catch(e){}})();</script>";
pub(crate) const HTML_BODY_END: &str = "</body></html>";
pub(crate) const HTML_ESC_AMP: &str = "&amp;";
pub(crate) const HTML_ESC_LT: &str = "&lt;";
pub(crate) const HTML_ESC_GT: &str = "&gt;";
pub(crate) const HTML_ESC_QUOT: &str = "&quot;";
pub(crate) const SCRIPT_ESC_CLOSE_FROM: &str = "</";
pub(crate) const SCRIPT_ESC_CLOSE_TO: &str = "<\\/";
pub(crate) const HTML_DL_OPEN: &str = "<dl>";
pub(crate) const HTML_DL_CLOSE: &str = "</dl>";
pub(crate) const HTML_DT_OPEN: &str = "<dt>";
pub(crate) const HTML_DT_CLOSE: &str = "</dt>";
pub(crate) const HTML_DD_OPEN: &str = "<dd>";
pub(crate) const HTML_DD_CLOSE: &str = "</dd>";
pub(crate) const HTML_MATH_INLINE_OPEN: &str = "<code class=\"math-inline\">";
pub(crate) const HTML_MATH_INLINE_CLOSE: &str = "</code>";
pub(crate) const HTML_MATH_DISPLAY_OPEN: &str = "<div class=\"math-display\"><code>";
pub(crate) const HTML_MATH_DISPLAY_CLOSE: &str = "</code></div>";
pub(crate) const HTML_CODE_LINE_OPEN: &str = "<span class=\"code-line\"><span class=\"code-ln\">";
pub(crate) const HTML_CODE_LINE_MID: &str = "</span><span class=\"code-txt\">";
pub(crate) const HTML_CODE_LINE_CLOSE: &str = "</span></span>";
pub(crate) const HTML_SPAN_CLOSE: &str = "</span>";
pub(crate) const TOK_CLASS_KW: &str = "tok-kw";
pub(crate) const TOK_CLASS_CTRL: &str = "tok-ctrl";
pub(crate) const TOK_CLASS_OP: &str = "tok-op";
pub(crate) const TOK_CLASS_FN: &str = "tok-fn";
pub(crate) const TOK_CLASS_TY: &str = "tok-ty";
pub(crate) const TOK_CLASS_PRIM: &str = "tok-prim";
pub(crate) const TOK_CLASS_STR: &str = "tok-str";
pub(crate) const TOK_CLASS_NUM: &str = "tok-num";
pub(crate) const TOK_CLASS_CMT: &str = "tok-cmt";
pub(crate) const TOK_CLASS_MAC: &str = "tok-mac";
pub(crate) const TOK_CLASS_PROP: &str = "tok-prop";
pub(crate) const TOK_CLASS_CONST: &str = "tok-const";
pub(crate) const TOK_CLASS_PUNC: &str = "tok-punc";
pub(crate) const TOK_CLASS_ADD: &str = "tok-add";
pub(crate) const TOK_CLASS_DEL: &str = "tok-del";
pub(crate) const BYTE_LOWER_O: u8 = b'o';
pub(crate) const BYTE_LOWER_N: u8 = b'n';
