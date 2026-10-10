mod codegen;

use std::{
    collections::{HashSet, VecDeque},
    env::var,
    ffi::OsStr,
    fs,
    path::{Component, Path, PathBuf},
};

use {
    http_constant::ROOT_PATH,
    pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd},
    serde_yaml::Value,
};

use codegen::*;

/// Root of the project README.md frontmatter (site + locales block).
#[derive(Debug)]
struct Config {
    /// `[site]` equivalent.
    site: SiteConfig,
    /// `[[locales]]` equivalent.
    locales: Vec<LocaleConfig>,
}

/// Site block.
#[derive(Debug)]
struct SiteConfig {
    /// Site title shown in the navbar.
    title: String,
}

/// One locale entry.
#[derive(Debug, Clone)]
struct LocaleConfig {
    /// Route prefix, e.g. `/` or `/en/`.
    prefix: String,
    /// Content directory under `<SRC_DIR>`, e.g. `en` or `zh`.
    ///
    /// Required. The build refuses to guess: a locale's content directory
    /// is data, not a convention, so it is declared next to the prefix it
    /// serves. (It used to be hardcoded to `zh` for the `/` locale, which
    /// silently produced an empty site.)
    dir: String,
    /// Human label in the language dropdown, e.g. `简体中文`.
    label: String,
    /// Locale-specific site title override.
    title: Option<String>,
    /// Footer text for this locale.
    footer: Option<String>,
    /// Right TOC title label (default `On this page`).
    toc_label: Option<String>,
    /// Prev-page link label (default `Previous`).
    prev_label: Option<String>,
    /// Next-page link label (default `Next`).
    next_label: Option<String>,
    /// Navbar items for this locale.
    navbar: Option<Vec<NavItemConfig>>,
}

/// One navbar item.
#[derive(Debug, Clone)]
struct NavItemConfig {
    /// Display text.
    text: String,
    /// Link target (`/guide/` internal or `https://…` external).
    link: String,
}

/// A heading extracted for the right-side anchor TOC.
#[derive(Debug, Clone)]
struct Heading {
    /// 2 or 3.
    level: u8,
    /// Slug used as the element id.
    id: String,
    /// Plain text content.
    text: String,
}

/// A build-time block node (mirrors `euv_ui::EuvMdBlock`).
/// One block-level markdown node parsed from a source page.
///
/// Named `AstBlock` instead of `Block` so its variants (`CodeBlock`,
/// `BlockQuote`) don't trip `clippy::enum_names`. The variant names
/// intentionally mirror `euv_ui::EuvMdBlock` so the build-script
/// codegen can emit them verbatim — only the surrounding enum name
/// differs.
#[derive(Debug, Clone)]
enum AstBlock {
    /// Heading with slug and permalink.
    Heading {
        /// Level 1–6.
        level: u8,
        /// Slug id.
        id: String,
        /// Full `#<route>#<slug>` href.
        href: String,
        /// Inline content.
        inline: Vec<Inline>,
    },
    /// Paragraph.
    Paragraph(Vec<Inline>),
    /// Fenced code block.
    CodeBlock {
        /// Language tag.
        lang: String,
        /// Raw code.
        code: String,
    },
    /// Block quote.
    BlockQuote(Vec<AstBlock>),
    /// List.
    List {
        /// Ordered flag.
        ordered: bool,
        /// Items (each a block list).
        items: Vec<Vec<AstBlock>>,
    },
    /// GFM table.
    Table {
        /// Header cells.
        head: Vec<Vec<Inline>>,
        /// Body rows.
        rows: Vec<Vec<Vec<Inline>>>,
    },
    /// Custom container.
    Container {
        /// Kind.
        kind: String,
        /// Title label.
        title: String,
        /// Inner blocks.
        blocks: Vec<AstBlock>,
    },
    /// Thematic break.
    Rule,
    /// Raw HTML block.
    Html(String),
}

/// A build-time inline node (mirrors `euv_ui::EuvMdInline`).
#[derive(Debug, Clone)]
enum Inline {
    /// Plain text.
    Text(String),
    /// Bold.
    Strong(Vec<Inline>),
    /// Italic.
    Em(Vec<Inline>),
    /// Strikethrough.
    Del(Vec<Inline>),
    /// Inline code.
    Code(String),
    /// Link.
    Link {
        /// Resolved href.
        href: String,
        /// External flag.
        external: bool,
        /// Link text.
        children: Vec<Inline>,
    },
    /// Image.
    Image {
        /// URL.
        src: String,
        /// Alt text.
        alt: String,
    },
    /// Task-list checkbox.
    TaskMarker(bool),
    /// Soft break.
    SoftBreak,
    /// Hard break.
    HardBreak,
    /// Raw inline HTML.
    Html(String),
}

/// A parsed markdown page.
#[derive(Debug)]
struct Page {
    /// Full route, e.g. `/guide/getting-started.html` or `/zh/`.
    route: String,
    /// Page title (frontmatter `title` or first heading).
    title: String,
    /// Content block AST.
    blocks: Vec<AstBlock>,
    /// Anchor TOC entries (h2/h3).
    headings: Vec<Heading>,
    /// Home page flag (frontmatter `home: true`).
    home: bool,
    /// Hero text (home pages).
    hero_text: String,
    /// Tagline (home pages).
    tagline: String,
    /// Hero actions.
    actions: Vec<(String, String, String)>,
    /// Feature cards (home pages) — each tuple is `(icon, title, details, link)`.
    features: Vec<(String, String, String, String)>,
    /// Home-page hero stats: (icon, value, label) rendered as a row of
    /// icon+text tiles between the hero actions and the feature grid.
    stats: Vec<(String, String, String)>,
    /// Frontmatter footer override.
    footer: String,
    /// Frontmatter `order` (sidebar sorting).
    order: i64,
    /// `true` when the page is gated behind a password form.
    /// The password is **never** shipped in plaintext — only its
    /// hex-encoded SHA-256 digest is emitted into the WASM blob.
    private: bool,
    /// Hex-encoded SHA-256 of the password that unlocks `private: true`
    /// pages. Empty string when `private` is `false`.
    password_hash: String,
    /// Sidebar visibility (frontmatter `sidebar: false` hides the page from
    /// the auto-generated sidebar tree; the route still resolves directly).
    sidebar: bool,
    /// VuePress-style explicit child ordering (frontmatter `sidebar_order`)
    /// for the sidebar group rooted at this page's directory.
    sidebar_order: Vec<String>,
    /// Directory-index rendering (frontmatter `index: false` on a
    /// `README.md` / `index.md` opts the page out of codegen, VuePress-style:
    /// the sidebar group then toggles its children instead of navigating,
    /// and the index route 404s like a directory without a README).
    index: bool,
}

/// A sidebar tree node.
#[derive(Debug)]
struct SideItem {
    /// Display text.
    text: String,
    /// Optional link route (group index or leaf page).
    link: Option<String>,
    /// Nested children (non-empty → group).
    children: Vec<SideItem>,
}

/// Entry point of the build script.
const ENV_OUT_DIR: &str = "OUT_DIR";

/// Generates the documentation site into the output directory.
fn main() {
    let manifest_dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let docs_dir: PathBuf = match var(ENV_DOCS_SRC_DIR) {
        Ok(path) => PathBuf::from(path),
        Err(_) => manifest_dir.join(DIR_DOCS),
    };
    let www_dir: PathBuf = match var(ENV_DOCS_OUT_DIR) {
        Ok(path) => PathBuf::from(path),
        Err(_) => manifest_dir.join("www"),
    };
    let Ok(out_dir) = var(ENV_OUT_DIR) else {
        fail(ERROR_OUT_DIR_MISSING);
    };

    println!("cargo:rerun-if-changed={}", docs_dir.display());
    println!("cargo:rerun-if-env-changed=EUV_DOCS_SRC_DIR");
    println!("cargo:rerun-if-env-changed=EUV_DOCS_OUT_DIR");
    println!("cargo:rerun-if-env-changed=EUV_DOCS_LOCALE");
    println!("cargo:rerun-if-changed=build.rs");

    let config: Config = match load_config_from_readme(&docs_dir) {
        Ok(config) => config,
        Err(message) => fail(&message),
    };

    // Per-locale bundling: every build compiles exactly one locale into the
    // wasm (the CLI invokes one build per locale). `EUV_DOCS_LOCALE` selects
    // the locale by prefix (`/en/`), bare directory (`en`), or label; unset
    // pins the default (prefix `/`) locale. Pages of other locales — and
    // alias routes of the pinned locale's own content directory (e.g. the
    // legacy `/zh/` prefix) — are dropped, and the pinned locale's URL
    // prefix is stripped from every route and internal link, so inside a
    // bundle every route is prefix-free and the runtime needs no locale
    // detection at all.
    let pinned: usize = resolve_pinned_locale(&config);
    let pinned_prefix: String = config.locales[pinned].prefix.clone();
    let default_bundle: bool = pinned_prefix == URL_PREFIX_ROOT;

    let public_dir: PathBuf = docs_dir.join(DIR_PUBLIC);
    // Assets are copied only by the default (site-root) build; non-default
    // bundles live one directory deeper and reference the root copies via
    // rebased `../` URLs, so the bytes are hosted exactly once.
    if default_bundle {
        if public_dir.is_dir() {
            copy_dir(&public_dir, &www_dir);
        }
        copy_doc_assets(&docs_dir, &www_dir);
    }

    // (content directory, every URL prefix served from it) per locale
    // directory. A file directly under <SRC_DIR> belongs to no locale
    // and is not a page.
    let locale_prefixes: Vec<(String, Vec<String>)> = match resolve_locale_prefixes(&config) {
        Ok(index) => index,
        Err(message) => fail(&message),
    };

    let mut md_files: Vec<PathBuf> = Vec::new();
    collect_md(&docs_dir, &docs_dir, &mut md_files);
    md_files.sort();

    let mut pages: Vec<Page> = Vec::new();
    for file in &md_files {
        pages.extend(process_page(&docs_dir, file, &locale_prefixes));
    }

    let locale_roots: Vec<(String, PathBuf, String)> =
        match resolve_locale_roots(&docs_dir, &config) {
            Ok(roots) => roots,
            Err(message) => fail(&message),
        };

    pages.retain(|page: &Page| page_in_locale(&page.route, &pinned_prefix, &config));
    if pinned_prefix != URL_PREFIX_ROOT {
        for page in &mut pages {
            strip_page_prefix(page, &pinned_prefix);
        }
    }

    let mut pinned_locale: LocaleConfig = config.locales[pinned].clone();
    pinned_locale.prefix = URL_PREFIX_ROOT.to_string();
    if pinned_prefix != URL_PREFIX_ROOT
        && let Some(items) = &mut pinned_locale.navbar
    {
        for item in items {
            item.link = strip_url_prefix(&item.link, &pinned_prefix);
        }
    }

    let pinned_root: PathBuf = docs_dir.join(&config.locales[pinned].dir);
    // The sidebar is rebuilt against the stripped route space
    // (`URL_PREFIX_ROOT` as the route prefix), so its links match the
    // stripped page routes exactly.
    let sidebar_items: Vec<SideItem> =
        build_sidebar(&pinned_root, &pinned_root, URL_PREFIX_ROOT, &pages);
    let sidebars: Vec<(String, Vec<SideItem>)> = vec![(URL_PREFIX_ROOT.to_string(), sidebar_items)];
    let _ = locale_roots;

    write_locale_manifest(&www_dir, &config);

    let code: String = codegen(&config, &pinned_locale, &pages, &sidebars);
    if let Err(reason) = fs::write(PathBuf::from(out_dir).join(GENERATED_FILE_NAME), code) {
        fail(&format!("failed to write docs_gen.rs: {reason}"));
    }
}

/// Terminates the build with `message` on stderr.
///
/// # Arguments
///
/// - `&str` - The human-readable reason the build cannot proceed.
///
/// # Returns
///
/// - `!` - Never returns; the process exits with status 1.
fn fail(message: &str) -> ! {
    eprintln!("error: {message}");
    std::process::exit(1);
}

/// Resolves which locale this build compiles into the wasm bundle.
///
/// `EUV_DOCS_LOCALE` selects the locale by URL prefix (`/en/`), bare
/// content directory (`en`), or label (`English`); when unset the build
/// pins the default locale (prefix `/`). Aliases share a content directory
/// with their canonical locale (the legacy `/zh/` entry backs the same
/// `zh/` dir as `/`), so the request is resolved to the FIRST entry
/// declaring that directory — asking for an alias compiles the canonical
/// bundle, and the alias's URL space is served by the runtime redirect
/// table instead of duplicate pages.
///
/// # Arguments
///
/// - `&Config` - The parsed site configuration listing the locales.
///
/// # Returns
///
/// - `usize` - The index into `config.locales` of the canonical entry of
///   the locale this bundle compiles.
fn resolve_pinned_locale(config: &Config) -> usize {
    let requested: Option<String> = var(ENV_DOCS_LOCALE)
        .ok()
        .map(|value: String| value.trim().to_string())
        .filter(|value: &String| !value.is_empty());
    let index: usize = match requested {
        None => config
            .locales
            .iter()
            .position(|locale: &LocaleConfig| locale.prefix == URL_PREFIX_ROOT)
            .unwrap_or(0),
        Some(name) => {
            let bare: &str = name.trim_matches('/');
            let normalized: String = if bare.is_empty() {
                URL_PREFIX_ROOT.to_string()
            } else {
                format!("/{bare}/")
            };
            config
                .locales
                .iter()
                .position(|locale: &LocaleConfig| locale.prefix == normalized)
                .or_else(|| {
                    config
                        .locales
                        .iter()
                        .position(|locale: &LocaleConfig| locale.dir == bare)
                })
                .or_else(|| {
                    config
                        .locales
                        .iter()
                        .position(|locale: &LocaleConfig| locale.label == name)
                })
                .unwrap_or_else(|| {
                    let available: String = config
                        .locales
                        .iter()
                        .map(|locale: &LocaleConfig| {
                            format!("{} (dir {})", locale.prefix, locale.dir)
                        })
                        .collect::<Vec<String>>()
                        .join(", ");
                    fail(&format!(
                        "unknown locale `{name}`; available locales: {available}"
                    ));
                })
        }
    };
    let dir: &str = &config.locales[index].dir;
    config
        .locales
        .iter()
        .position(|locale: &LocaleConfig| locale.dir == dir)
        .unwrap_or(index)
}

/// Whether `route` belongs to the locale with URL prefix `pinned`.
///
/// Every configured prefix ends with `/` and routes are generated as
/// `<prefix><path>`, so prefix matching is boundary-safe. The default
/// locale owns every route no other locale claims — which also drops alias
/// routes (e.g. `/zh/...`) from the default bundle.
///
/// # Arguments
///
/// - `&str` - The page route being tested.
/// - `&str` - The pinned locale's URL prefix.
/// - `&Config` - The parsed site configuration listing every prefix.
///
/// # Returns
///
/// - `bool` - `true` when the route belongs in this bundle.
fn page_in_locale(route: &str, pinned: &str, config: &Config) -> bool {
    if pinned != URL_PREFIX_ROOT {
        return route.starts_with(pinned);
    }
    !config
        .locales
        .iter()
        .filter(|locale: &&LocaleConfig| locale.prefix != URL_PREFIX_ROOT)
        .any(|locale: &LocaleConfig| route.starts_with(&locale.prefix))
}

/// Removes the locale URL prefix from an internal route or link.
///
/// `/en/ltpp/` → `/ltpp/` under a pinned `/en/`; the locale home `/en/`
/// collapses to `/`. External (`https://…`) and already-root links are
/// returned unchanged.
///
/// # Arguments
///
/// - `&str` - The route or internal link.
/// - `&str` - The pinned locale's URL prefix (with trailing slash).
///
/// # Returns
///
/// - `String` - The prefix-free route.
fn strip_url_prefix(url: &str, pinned: &str) -> String {
    let head: &str = pinned.trim_end_matches('/');
    if url == head {
        return URL_PREFIX_ROOT.to_string();
    }
    match url.strip_prefix(pinned) {
        Some(rest) => format!("{URL_PREFIX_ROOT}{rest}"),
        None => url.to_string(),
    }
}

/// Strips the locale prefix from every internal reference of a page: the
/// route, heading permalink hrefs (rebuilt from the stripped route), link
/// destinations inside the block AST, and hero action / feature card links.
///
/// # Arguments
///
/// - `&mut Page` - The page to rewrite in place.
/// - `&str` - The pinned locale's URL prefix.
fn strip_page_prefix(page: &mut Page, pinned: &str) {
    page.route = strip_url_prefix(&page.route, pinned);
    strip_blocks_prefix(&mut page.blocks, pinned);
    for (_text, link, _kind) in &mut page.actions {
        *link = strip_url_prefix(link, pinned);
    }
    for (_icon, _title, _details, link) in &mut page.features {
        *link = strip_url_prefix(link, pinned);
    }
}

/// Strips the locale prefix from link destinations inside a block list.
///
/// # Arguments
///
/// - `&mut [AstBlock]` - The blocks to rewrite in place.
/// - `&str` - The pinned locale's URL prefix.
fn strip_blocks_prefix(blocks: &mut [AstBlock], pinned: &str) {
    for block in blocks {
        match block {
            AstBlock::Heading { inline, .. } => strip_inlines_prefix(inline, pinned),
            AstBlock::Paragraph(inline) => strip_inlines_prefix(inline, pinned),
            AstBlock::BlockQuote(inner) => strip_blocks_prefix(inner, pinned),
            AstBlock::List { items, .. } => {
                for item in items {
                    strip_blocks_prefix(item, pinned);
                }
            }
            AstBlock::Table { head, rows } => {
                for cell in head {
                    strip_inlines_prefix(cell, pinned);
                }
                for row in rows {
                    for cell in row {
                        strip_inlines_prefix(cell, pinned);
                    }
                }
            }
            AstBlock::Container { blocks: inner, .. } => strip_blocks_prefix(inner, pinned),
            AstBlock::Html(raw) => *raw = rebase_html_asset_srcs(raw),
            AstBlock::CodeBlock { .. } | AstBlock::Rule => {}
        }
    }
}

/// Strips the locale prefix from link destinations inside an inline list,
/// and rebases asset URLs (`./img/…` → `../img/…`) for the one-level-deep
/// bundle directory.
///
/// # Arguments
///
/// - `&mut [Inline]` - The inline nodes to rewrite in place.
/// - `&str` - The pinned locale's URL prefix.
fn strip_inlines_prefix(inlines: &mut [Inline], pinned: &str) {
    for inline in inlines {
        match inline {
            Inline::Link { href, children, .. } => {
                *href = strip_url_prefix(href, pinned);
                strip_inlines_prefix(children, pinned);
            }
            Inline::Strong(children) | Inline::Em(children) | Inline::Del(children) => {
                strip_inlines_prefix(children, pinned);
            }
            Inline::Image { src, .. } => *src = rebase_asset_url(src),
            Inline::Html(raw) => *raw = rebase_html_asset_srcs(raw),
            Inline::Text(_)
            | Inline::Code(_)
            | Inline::TaskMarker(_)
            | Inline::SoftBreak
            | Inline::HardBreak => {}
        }
    }
}

/// Rebases a site-root-relative asset URL for a non-default locale bundle.
///
/// The bundle is served one directory below the site root (`<root>/en/`),
/// while asset bytes are hosted only at the root by the default-locale
/// build, so `./img/x.png` (and the bare `img/x.png` form) become
/// `../img/x.png`. Author-written `../…` URLs already resolve to the root
/// from one level deep and pass through unchanged, as do absolute and
/// external URLs.
///
/// # Arguments
///
/// - `&str` - The asset URL as emitted by `rewrite_image_src`.
///
/// # Returns
///
/// - `String` - The URL relative to the bundle directory.
fn rebase_asset_url(url: &str) -> String {
    if url.is_empty()
        || url.starts_with("../")
        || url.starts_with('/')
        || url.starts_with(URL_SCHEME_HTTP)
        || url.starts_with(URL_SCHEME_DATA)
        || url.starts_with(URL_SCHEME_BLOB)
    {
        return url.to_string();
    }
    if let Some(rest) = url.strip_prefix("./") {
        return format!("../{rest}");
    }
    format!("../{url}")
}

/// Rebases every `src="…"` attribute inside a raw HTML block for a
/// non-default locale bundle, mirroring [`rebase_asset_url`].
///
/// # Arguments
///
/// - `&str` - The raw HTML fragment.
///
/// # Returns
///
/// - `String` - The fragment with asset `src` attributes rebased.
fn rebase_html_asset_srcs(html: &str) -> String {
    let mut out: String = String::with_capacity(html.len());
    let mut rest: &str = html;
    while let Some(pos) = rest.find("src=\"") {
        out.push_str(&rest[..pos + 5]);
        let after: &str = &rest[pos + 5..];
        match after.find('"') {
            Some(end) => {
                out.push_str(&rebase_asset_url(&after[..end]));
                rest = &after[end..];
            }
            None => {
                out.push_str(after);
                return out;
            }
        }
    }
    out.push_str(rest);
    out
}

/// Maps a locale URL prefix to the bundle directory it is served from:
/// the default locale lives at the site root (`""`), every other locale in
/// a directory named after its prefix (`/en/` → `en/`).
///
/// # Arguments
///
/// - `&str` - The locale URL prefix.
///
/// # Returns
///
/// - `String` - The directory-relative path fragment.
fn locale_dir_url(prefix: &str) -> String {
    let trimmed: &str = prefix.trim_matches('/');
    if trimmed.is_empty() {
        String::new()
    } else {
        format!("{trimmed}/")
    }
}

/// Writes `<out>/.deploy/locales.tsv` listing every locale (one row per
/// unique content directory: URL prefix, bundle directory, label).
///
/// The `euv-docs` CLI reads the manifest after the first (default-locale)
/// build to discover the remaining locales it must build, so the locale
/// list has exactly one source of truth — the README frontmatter parsed
/// here — and the CLI never needs its own YAML parser. The file also
/// documents the deployed bundle layout for debugging.
///
/// # Arguments
///
/// - `&Path` - The site output directory the current build writes to.
/// - `&Config` - The parsed site configuration listing the locales.
fn write_locale_manifest(www_dir: &Path, config: &Config) {
    let mut seen: Vec<&str> = Vec::new();
    let mut lines: String = String::from("prefix\tdir\tlabel\n");
    for locale in &config.locales {
        if seen.contains(&locale.dir.as_str()) {
            continue;
        }
        seen.push(&locale.dir);
        lines.push_str(&format!(
            "{}\t{}\t{}\n",
            locale.prefix,
            locale_dir_url(&locale.prefix),
            locale.label
        ));
    }
    let deploy_dir: PathBuf = www_dir.join(DEPLOY_DIR_NAME);
    if let Err(reason) = fs::create_dir_all(&deploy_dir)
        .and_then(|()| fs::write(deploy_dir.join(LOCALE_MANIFEST_FILE_NAME), lines))
    {
        fail(&format!("failed to write locale manifest: {reason}"));
    }
}

/// Indexes every configured prefix by the content directory that serves it.
///
/// A content directory may back SEVERAL URL prefixes: `docs-pages/docs`
/// declares `prefix: /` + `dir: zh` and `prefix: /zh/` + `dir: zh` because
/// the same Simplified Chinese tree has to answer both `/<page>` and
/// `/zh/<page>`. The index therefore holds every prefix declared for a
/// directory, in declaration order, and `process_page` emits one route per
/// entry.
///
/// The reverse collision is genuinely ambiguous and is rejected: two
/// directories declaring the same prefix would each emit `/x`, so the
/// generated site would carry two different pages at one route and the
/// router would silently serve whichever was emitted first.
///
/// # Arguments
///
/// - `&Config` - The parsed site configuration listing the locales.
///
/// # Returns
///
/// - `Result<Vec<(String, Vec<String>)>, String>` - The
///   `(content dir, URL prefixes)` pairs, or the reason the build must
///   fail.
fn resolve_locale_prefixes(config: &Config) -> Result<Vec<(String, Vec<String>)>, String> {
    let mut index: Vec<(String, Vec<String>)> = Vec::new();
    let mut owners: Vec<(String, String, usize)> = Vec::new();
    for (entry, locale) in config.locales.iter().enumerate() {
        let claimed: Option<usize> = owners
            .iter()
            .position(|(prefix, _, _): &(String, String, usize)| prefix == &locale.prefix);
        if let Some(at) = claimed {
            let (_, first_dir, first_entry): &(String, String, usize) = &owners[at];
            return Err(format!(
                "locales entry #{entry} declares prefix `{}` for dir `{}`, but locales entry #{first_entry} already declares that prefix for dir `{first_dir}`; one URL prefix can serve only one content directory, because both directories would emit the same routes",
                locale.prefix, locale.dir,
            ));
        }
        let known: Option<usize> = index
            .iter()
            .position(|(dir, _): &(String, Vec<String>)| dir == &locale.dir);
        match known {
            Some(at) => index[at].1.push(locale.prefix.clone()),
            None => index.push((locale.dir.clone(), vec![locale.prefix.clone()])),
        }
        owners.push((locale.prefix.clone(), locale.dir.clone(), entry));
    }
    Ok(index)
}

/// Resolves every configured locale to its content root under `docs_dir`.
///
/// Each locale's content directory comes from its declared `dir`. There
/// is no default and no prefix-derived fallback: a missing or empty locale
/// directory is a build error, because the old behaviour (hardcoding `zh`
/// for the `/` locale) produced a site with one page and no error at all.
///
/// Several locales may share one `dir`; each still contributes its own
/// `(prefix, root, prefix)` triple, so every declared prefix gets a
/// sidebar resolved against that shared directory.
///
/// # Arguments
///
/// - `&Path` - The docs source root that locale `dir` values resolve against.
/// - `&Config` - The parsed site configuration listing the locales.
///
/// # Returns
///
/// - `Result<Vec<(String, PathBuf, String)>, String>` - The
///   `(prefix, root, build_locale)` triple per locale, or the reason the
///   build must fail.
fn resolve_locale_roots(
    docs_dir: &Path,
    config: &Config,
) -> Result<Vec<(String, PathBuf, String)>, String> {
    let mut locale_roots: Vec<(String, PathBuf, String)> = Vec::new();
    for locale in &config.locales {
        let root: PathBuf = docs_dir.join(&locale.dir);
        if !root.is_dir() {
            return Err(format!(
                "locale `{}` declares dir `{}` but <SRC_DIR>/{} is not a directory",
                locale.prefix, locale.dir, locale.dir
            ));
        }
        let markdown_count: usize = collect_md_count(&root);
        if markdown_count == 0 {
            return Err(format!(
                "locale `{}` dir `{}` contains no .md files; every locale must ship content",
                locale.prefix, locale.dir
            ));
        }
        locale_roots.push((locale.prefix.clone(), root, locale.prefix.clone()));
    }
    Ok(locale_roots)
}

/// Loads the site-level configuration (site + locales) from
/// `<SRC_DIR>/../README.md` frontmatter.
///
/// Every `[[locales]]` entry is validated here: a malformed entry aborts
/// the build instead of being dropped, and an empty `locales:` sequence
/// aborts too. Both cases used to degrade silently into a zero-locale
/// site, whose WASM panicked on `site.locales[0]` at load time.
/// # Arguments
///
/// - `&Path` - the docs content root that `../README.md` is resolved against
///
/// # Returns
///
/// - `Result<Config, String>` - the parsed `site` and `locales` blocks, or
///   the reason the build must fail
fn load_config_from_readme(docs_dir: &Path) -> Result<Config, String> {
    let readme_path: PathBuf = docs_dir.join(README_RELATIVE_PATH);
    let Ok(raw) = fs::read_to_string(&readme_path) else {
        return Err(ERROR_SITE_CONFIG_MISSING.to_string());
    };
    let (fm, _body) = split_frontmatter(&raw);
    let Some(site_yaml) = fm.get(Value::String(YAML_SITE.to_string())) else {
        return Err(ERROR_SITE_CONFIG_MISSING.to_string());
    };
    let Some(locales_yaml) = fm.get(Value::String(YAML_LOCALES.to_string())) else {
        return Err(ERROR_SITE_CONFIG_MISSING.to_string());
    };
    let Some(locales_seq) = locales_yaml.as_sequence() else {
        return Err(ERROR_LOCALES_NOT_A_SEQUENCE.to_string());
    };
    let mut locales: Vec<LocaleConfig> = Vec::with_capacity(locales_seq.len());
    for (index, entry) in locales_seq.iter().enumerate() {
        match parse_locale_config(entry) {
            Ok(locale) => locales.push(locale),
            Err(reason) => return Err(describe_locale_entry(index, entry, &reason)),
        }
    }
    if locales.is_empty() {
        return Err(ERROR_LOCALES_EMPTY.to_string());
    }
    let Some(site) = parse_site_config(site_yaml) else {
        return Err(ERROR_SITE_CONFIG_MISSING.to_string());
    };
    Ok(Config { site, locales })
}

/// Names the offending `[[locales]]` entry in a parse failure.
///
/// The message identifies the entry by both its zero-based `locales:`
/// index and its `prefix` when one is readable, so the author can find
/// the broken block in `README.md` without counting list items.
/// # Arguments
///
/// - `usize` - the entry's zero-based position in the `locales:` sequence
/// - `&Value` - the malformed `[[locales]]` YAML mapping
/// - `&str` - the reason `parse_locale_config` rejected the entry
///
/// # Returns
///
/// - `String` - the full error text, including the located entry
fn describe_locale_entry(index: usize, entry: &Value, reason: &str) -> String {
    let located: String = match yaml_str(entry, YAML_PREFIX) {
        Some(prefix) => format!(" (locales entry #{index}, prefix `{prefix}`)"),
        None => format!(" (locales entry #{index})"),
    };
    format!("{ERROR_LOCALES_MALFORMED}{located}: {reason}")
}

/// Reads the `[site]` block of the README frontmatter into a [`SiteConfig`].
///
/// # Arguments
///
/// - `&Value` - the `site` YAML mapping to read
///
/// # Returns
///
/// - `Option<SiteConfig>` - the site block, or `None` when it has no `title` string
fn parse_site_config(yaml: &Value) -> Option<SiteConfig> {
    Some(SiteConfig {
        title: yaml_str(yaml, YAML_TITLE)?,
    })
}

/// Reads one `[[locales]]` frontmatter entry into a [`LocaleConfig`].
/// Every required field (`prefix`, `dir`, `label`) must be a present
/// string; a missing one is a hard error naming the key, because dropping
/// the entry silently produced a site whose runtime panicked. The
/// optional title, footer, navigation-label and navbar fields fall back
/// to `None` and are defaulted later during codegen.
///
/// # Arguments
///
/// - `&Value` - one `[[locales]]` YAML sequence entry to read
///
/// # Returns
///
/// - `Result<LocaleConfig, String>` - the locale entry, or the reason the
///   entry is malformed
fn parse_locale_config(yaml: &Value) -> Result<LocaleConfig, String> {
    let prefix: String = require_locale_str(yaml, YAML_PREFIX)?;
    let dir: String = require_locale_str(yaml, YAML_DIR)?;
    let label: String = require_locale_str(yaml, YAML_LABEL)?;
    let navbar_items: Vec<NavItemConfig> = yaml_list(yaml, YAML_NAVBAR)
        .iter()
        .filter_map(|n: &Value| {
            Some(NavItemConfig {
                text: yaml_str(n, YAML_TEXT)?,
                link: yaml_str(n, YAML_LINK)?,
            })
        })
        .collect();
    Ok(LocaleConfig {
        prefix,
        dir,
        label,
        title: yaml_str(yaml, YAML_TITLE),
        footer: yaml_str(yaml, YAML_FOOTER),
        toc_label: yaml_str(yaml, YAML_TOC_LABEL),
        prev_label: yaml_str(yaml, YAML_PREV_LABEL),
        next_label: yaml_str(yaml, YAML_NEXT_LABEL),
        navbar: if navbar_items.is_empty() {
            None
        } else {
            Some(navbar_items)
        },
    })
}

/// Reads one required `[[locales]]` string field.
///
/// # Arguments
///
/// - `&Value` - the `[[locales]]` YAML mapping to read from
/// - `&str` - the required key to read
///
/// # Returns
///
/// - `Result<String, String>` - the string value at `key`, or the reason the
///   required field is absent or not a string
fn require_locale_str(yaml: &Value, key: &str) -> Result<String, String> {
    match yaml_str(yaml, key) {
        Some(value) => Ok(value),
        None => Err(format!(
            "required key `{key}` is missing or is not a string"
        )),
    }
}

/// Counts the markdown files under `dir`, used to assert that a locale
/// actually ships content before the site is generated.
///
/// # Arguments
///
/// - `&Path` - Root of the locale content directory to walk.
///
/// # Returns
///
/// The number of `*.md` files `collect_md` finds beneath `dir`.
fn collect_md_count(dir: &Path) -> usize {
    let mut files: Vec<PathBuf> = Vec::new();
    collect_md(dir, dir, &mut files);
    files.len()
}

/// Collects every markdown file under `dir` into `out`, recursively,
/// skipping a `public` directory that sits directly below `root`.
///
/// # Arguments
///
/// - `&Path` - the tree root a `public` directory directly below it is skipped from
/// - `&Path` - the directory whose entries are read
/// - `&mut Vec<PathBuf>` - the sink that every `*.md` path is pushed into
///
fn collect_md(root: &Path, dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path: PathBuf = entry.path();
        if path.is_dir() {
            if path.file_name().is_some_and(|n: &OsStr| n == DIR_PUBLIC)
                && path.parent() == Some(root)
            {
                continue;
            }
            collect_md(root, &path, out);
        } else if path.extension().is_some_and(|e: &OsStr| e == "md") {
            out.push(path);
        }
    }
}

/// Walks the docs tree and copies every non-md sibling file (images,
/// fonts, raw assets) into `www/`, preserving the relative path under
/// `docs_dir`. This lets markdown authors keep assets next to their
/// content (`essay/posts/2025/img.jpg`) and reference them as
/// `/essay/posts/2025/img.jpg`. Directories named `public` are skipped
/// because they are handled by `copy_dir(docs/public -> www/)` above
/// and contain site-level assets, not page-level assets.
/// # Arguments
///
/// - `&Path` - the docs content root, also the root the relative paths are kept against
/// - `&Path` - the site output root the assets are copied into
///
fn copy_doc_assets(docs_dir: &Path, www_dir: &Path) {
    copy_doc_assets_recurse(docs_dir, docs_dir, www_dir);
}

/// Recursive worker of [`copy_doc_assets`]: copies every non-markdown
/// file under `dir` into `www_dir`, recreating the directory tree and
/// skipping a `public` directory that sits directly below `root`.
///
/// # Arguments
///
/// - `&Path` - the docs content root, also the root the relative paths are kept against
/// - `&Path` - the directory whose entries are read
/// - `&Path` - the site output root the assets are copied into
///
fn copy_doc_assets_recurse(root: &Path, dir: &Path, www_dir: &Path) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path: PathBuf = entry.path();
        if path.is_dir() {
            if path.file_name().is_some_and(|n: &OsStr| n == DIR_PUBLIC)
                && path.parent() == Some(root)
            {
                continue;
            }
            copy_doc_assets_recurse(root, &path, www_dir);
        } else if path.extension().is_some_and(|e: &OsStr| e != "md") {
            let Ok(rel) = path.strip_prefix(root) else {
                continue;
            };
            let target: PathBuf = www_dir.join(rel);
            if let Some(parent) = target.parent() {
                let _: Result<(), std::io::Error> = fs::create_dir_all(parent);
            }
            let _: Result<u64, std::io::Error> = fs::copy(&path, &target);
        }
    }
}

/// Recursively copies a directory tree.
/// # Arguments
///
/// - `&Path` - the directory tree to copy from
/// - `&Path` - the existing-or-new directory the tree is copied into
///
fn copy_dir(src: &Path, dst: &Path) {
    let Ok(entries) = fs::read_dir(src) else {
        return;
    };
    let _: Result<(), std::io::Error> = fs::create_dir_all(dst);
    for entry in entries.flatten() {
        let path: PathBuf = entry.path();
        let target: PathBuf = dst.join(entry.file_name());
        if path.is_dir() {
            copy_dir(&path, &target);
        } else {
            let _: Result<u64, std::io::Error> = fs::copy(&path, &target);
        }
    }
}

/// Strip the docs_dir (or locale_root) prefix from a path, defending
/// against the two misconfigurations that produced the duplicate
/// `/docs/ltpp/` sidebar entry the live site shipped with:
///
/// 1. `file.strip_prefix(docs_dir)` returns `Err` (e.g. `docs_dir` is the
///    repo root and `file` lives one segment deeper). Fall back to
///    "drop the leading path segment".
///
/// 2. `file.strip_prefix(docs_dir)` succeeds but the first segment of
///    the relative path equals `docs_dir`'s basename (e.g. `docs_dir`
///    is `<repo>/docs` and `file` lives at `<repo>/docs/docs/...md`).
///    Drop the first segment to recover the intended markdown-relative
///    path.
/// # Arguments
///
/// - `&Path` - the markdown file to make relative
/// - `&Path` - the docs or locale root to strip from `file`
///
/// # Returns
///
/// - `PathBuf` - the markdown-relative path, or the recovered path when `file`
///   does not actually live under `prefix`
fn strip_path_prefix(file: &Path, prefix: &Path) -> PathBuf {
    let rel: PathBuf = match file.strip_prefix(prefix) {
        Ok(rel) => rel.to_path_buf(),
        Err(_) => {
            let comps: Vec<Component> = file.components().collect();
            if comps.is_empty() {
                PathBuf::new()
            } else {
                let mut tail: PathBuf = PathBuf::new();
                for c in comps.into_iter().skip(1) {
                    tail.push(c.as_os_str());
                }
                tail
            }
        }
    };
    match prefix.file_name() {
        Some(docs_base) => {
            let mut comps: Vec<Component> = rel.components().collect();
            if comps.len() > 1 {
                let first_str: Option<String> = match comps.first() {
                    Some(Component::Normal(s)) => s.to_str().map(|s: &str| s.to_string()),
                    _ => None,
                };
                let base_str: Option<String> = docs_base.to_str().map(|s: &str| s.to_string());
                if let (Some(first_s), Some(base_s)) = (first_str, base_str)
                    && first_s == base_s
                {
                    comps.remove(0);
                    let mut fixed: PathBuf = PathBuf::new();
                    for c in comps {
                        fixed.push(c.as_os_str());
                    }
                    return fixed;
                }
            }
            rel
        }
        None => rel,
    }
}

/// Parses one markdown file into the [`Page`]s it serves.
///
/// A content directory may be claimed by several locale prefixes, so one
/// markdown file can legitimately produce SEVERAL routes: a page under
/// `dir: zh` declared by both `/` and `/zh/` is emitted once per prefix,
/// with the heading permalinks rebuilt per route. The prefix-independent
/// frontmatter (title, hero, features, ordering, privacy) is re-read per
/// emitted page, so the N routes differ exactly in their route prefix.
/// # Arguments
///
/// - `&Path` - the docs content root, used to resolve the owning locale
/// - `&Path` - the markdown file to parse
/// - `&[(String, Vec<String>)]` - the `(content dir, URL prefixes)` pair
///   per locale directory, as built by `resolve_locale_prefixes`
///
/// # Returns
///
/// - `Vec<Page>` - one parsed page per prefix serving this file's
///   directory, in declaration order; a file whose directory is claimed by
///   exactly one prefix yields exactly one page, as before
fn process_page(
    docs_dir: &Path,
    file: &Path,
    locale_prefixes: &[(String, Vec<String>)],
) -> Vec<Page> {
    let raw: String = match fs::read_to_string(file) {
        Ok(raw) => raw,
        Err(reason) => fail(&format!("failed to read {}: {reason}", file.display())),
    };
    let (frontmatter, body) = split_frontmatter(&raw);

    // See `strip_path_prefix` for the rationale; the helper strips the
    // markdown dir name that leaked in when `EUV_DOCS_SRC_DIR` pointed
    // one level too high (the duplicate `/docs/ltpp/` sidebar entry).
    // Resolve the owning locale from the path RELATIVE TO <SRC_DIR> before
    // any prefix stripping: the locale directory is the first component and
    // it is what selects the URL prefixes. `strip_path_prefix` would
    // otherwise consume it (its fallback drops the first component),
    // leaving the page with no locale and a wrong route.
    let raw_rel: PathBuf = match file.strip_prefix(docs_dir) {
        Ok(rel) => rel.to_path_buf(),
        Err(_) => file.to_path_buf(),
    };
    let first_dir: Option<String> = raw_rel.components().find_map(|c: Component<'_>| match c {
        Component::Normal(s) => Some(s.to_string_lossy().into_owned()),
        _ => None,
    });
    let owned_prefixes: Option<&Vec<String>> = first_dir.as_ref().and_then(|first: &String| {
        locale_prefixes
            .iter()
            .find(|entry: &&(String, Vec<String>)| &entry.0 == first)
            .map(|entry: &(String, Vec<String>)| &entry.1)
    });

    let rel: PathBuf = match owned_prefixes {
        Some(_) => {
            let mut trimmed: PathBuf = PathBuf::new();
            let mut skipped: bool = false;
            for component in raw_rel.components() {
                if !skipped {
                    skipped = true;
                    continue;
                }
                trimmed.push(component);
            }
            trimmed
        }
        None => strip_path_prefix(file, docs_dir),
    };
    let rel: &Path = rel.as_path();
    let segments: Vec<String> = rel
        .components()
        .filter_map(|c: Component<'_>| match c {
            Component::Normal(s) => Some(s.to_string_lossy().into_owned()),
            _ => None,
        })
        .collect();

    // The URL prefix comes from the locale's declared `prefix`, NOT from the
    // directory name. A locale may live in `en/` yet be served at `/`, and a
    // locale served at `/en/` may live in any directory it likes. Decoupling
    // them is what lets the content tree be reorganised without silently
    // moving every public URL. A directory claimed by several prefixes
    // contributes one page per prefix.
    let prefixes: Vec<String> = match owned_prefixes {
        Some(list) => list.clone(),
        None => vec![ROOT_PATH.to_string()],
    };

    prefixes
        .iter()
        .map(|prefix: &String| build_page(&frontmatter, body, &segments, prefix, file))
        .collect()
}

/// Builds one [`Page`] for a file already split into frontmatter and body.
///
/// The route depends on the serving prefix, and so do the rendered heading
/// permalinks (`#<route>#<slug>`), which is why this runs once per prefix.
/// Everything else in the frontmatter is prefix-independent and is read
/// here rather than in the caller so each page carries its own copy.
/// # Arguments
///
/// - `&Value` - the page frontmatter
/// - `&str` - the markdown body, with the frontmatter already removed
/// - `&[String]` - the markdown-relative path segments of the page
/// - `&str` - the URL prefix of the locale serving this page
/// - `&Path` - the source file, named in the privacy diagnostic
///
/// # Returns
///
/// - `Page` - the parsed page, with its route, title, block AST and TOC
fn build_page(
    frontmatter: &Value,
    body: &str,
    segments: &[String],
    prefix: &str,
    file: &Path,
) -> Page {
    let route: String = route_for(segments, prefix);

    let fm_title: Option<String> = yaml_str(frontmatter, YAML_TITLE);
    let order: i64 = yaml_i64(frontmatter, YAML_ORDER).unwrap_or(0);

    let (blocks, headings, first_h1) = render_markdown(body, &route);

    let title: String = fm_title
        .or(first_h1)
        .unwrap_or_else(|| prettify(stem_of(segments)));

    let home: bool = yaml_bool(frontmatter, YAML_HOME);
    let hero_text: String = yaml_str(frontmatter, YAML_HERO_TEXT_CAMEL)
        .or_else(|| yaml_str(frontmatter, YAML_HERO_TEXT_SNAKE))
        .unwrap_or_default();
    let tagline: String = yaml_str(frontmatter, YAML_TAGLINE).unwrap_or_default();
    let footer: String = yaml_str(frontmatter, YAML_FOOTER).unwrap_or_default();

    let actions: Vec<(String, String, String)> = yaml_list(frontmatter, YAML_ACTIONS)
        .iter()
        .map(|item: &Value| {
            (
                yaml_str(item, YAML_TEXT).unwrap_or_default(),
                yaml_str(item, YAML_LINK).unwrap_or_default(),
                yaml_str(item, YAML_TYPE).unwrap_or_else(|| ACTION_KIND_PRIMARY.to_string()),
            )
        })
        .collect();

    let features: Vec<(String, String, String, String)> = yaml_list(frontmatter, YAML_FEATURES)
        .iter()
        .map(|item: &Value| {
            (
                yaml_str(item, YAML_ICON).unwrap_or_default(),
                yaml_str(item, YAML_TITLE).unwrap_or_default(),
                yaml_str(item, YAML_DETAILS).unwrap_or_default(),
                yaml_str(item, YAML_LINK).unwrap_or_default(),
            )
        })
        .collect();

    let stats: Vec<(String, String, String)> = yaml_list(frontmatter, YAML_STATS)
        .iter()
        .map(|item: &Value| {
            (
                yaml_str(item, YAML_ICON).unwrap_or_default(),
                yaml_str(item, YAML_VALUE).unwrap_or_default(),
                yaml_str(item, YAML_LABEL).unwrap_or_default(),
            )
        })
        .collect();

    let private: bool = is_private_page(frontmatter);
    let password_hash: String = if private {
        match yaml_str(frontmatter, YAML_PASSWORD) {
            Some(plain) if !plain.is_empty() => sha256_hex(&plain),
            Some(_) | None => {
                eprintln!(
                    "error: {}: page marked private but frontmatter has no non-empty `password:` field",
                    file.display()
                );
                std::process::exit(1);
            }
        }
    } else {
        String::new()
    };

    Page {
        route,
        title,
        blocks,
        headings,
        home,
        hero_text,
        tagline,
        actions,
        features,
        stats,
        footer,
        order,
        private,
        password_hash,
        sidebar: frontmatter
            .get(YAML_SIDEBAR)
            .and_then(|v: &Value| v.as_bool())
            .unwrap_or(true),
        sidebar_order: yaml_list(frontmatter, YAML_SIDEBAR_ORDER)
            .iter()
            .filter_map(|v: &Value| v.as_str().map(|s: &str| s.to_string()))
            .collect(),
        index: renders_index(frontmatter, segments),
    }
}

/// Reads the VuePress-style `index` flag: `false` on a `README.md` /
/// `index.md` drops the directory index page so its sidebar group only
/// toggles collapse. Non-index pages always render.
/// # Arguments
///
/// - `&Value` - the page frontmatter to read the `index` flag from
/// - `&[String]` - the markdown-relative path segments, used to find the file stem
///
/// # Returns
///
/// - `bool` - `true` when the page is emitted, `false` when it is a directory
///   index page that frontmatter `index: false` opted out of codegen
fn renders_index(frontmatter: &Value, segments: &[String]) -> bool {
    let stem: String = stem_of(segments);
    if stem != README_STEM && stem != INDEX_STEM {
        return true;
    }
    frontmatter
        .get(YAML_INDEX)
        .and_then(|v: &Value| v.as_bool())
        .unwrap_or(true)
}

/// Computes the VuePress-style route for a page.
///
/// - `README.md` / `index.md` → directory route with trailing slash.
/// - `foo.md` → `/foo.html`.
/// # Arguments
///
/// - `&[String]` - the markdown-relative path segments of the page
/// - `&str` - the URL prefix of the locale that owns the page
///
/// # Returns
///
/// - `String` - the site route, e.g. `/guide/` for a directory index or
///   `/guide/foo.html` for a leaf page
fn route_for(segments: &[String], locale: &str) -> String {
    let stem: String = stem_of(segments);
    let dir_parts: &[String] = if segments.is_empty() {
        &[]
    } else {
        &segments[..segments.len() - 1]
    };
    let dir_path: String = if dir_parts.is_empty() {
        String::new()
    } else {
        format!("{}/", dir_parts.join("/"))
    };
    if stem == README_STEM || stem == INDEX_STEM {
        let base: String = format!("/{dir_path}");
        join_locale_route(locale, &base)
    } else {
        let base: String = format!("/{dir_path}{stem}.html");
        join_locale_route(locale, &base)
    }
}

/// Joins a locale prefix with a base route.
/// # Arguments
///
/// - `&str` - the URL prefix of the owning locale, `/` for the root locale
/// - `&str` - the base route the prefix is prepended to
///
/// # Returns
///
/// - `String` - the joined route; `base` unchanged for the `/` locale, otherwise
///   the prefix with any trailing slash removed, then `base`
fn join_locale_route(locale: &str, base: &str) -> String {
    if locale == "/" {
        base.to_string()
    } else {
        format!("{}{}", locale.trim_end_matches('/'), base)
    }
}

/// Returns the file stem of the last segment.
/// # Arguments
///
/// - `&[String]` - the markdown-relative path segments of the page
///
/// # Returns
///
/// - `String` - the last segment with its `.md` extension removed, or the empty
///   string when there is no segment
fn stem_of(segments: &[String]) -> String {
    segments
        .last()
        .map(|s: &String| s.trim_end_matches(".md").to_string())
        .unwrap_or_default()
}

/// Converts a file/dir name into a human title (`getting-started` → `Getting Started`).
/// # Arguments
///
/// - `String` - the file or directory name to prettify
///
/// # Returns
///
/// - `String` - the name with `-` / `_` turned into spaces and each word
///   capitalised, or `Index` when the result would be empty
fn prettify(name: String) -> String {
    let mut out: String = String::new();
    let mut capitalize: bool = true;
    for ch in name.chars() {
        if ch == '-' || ch == '_' {
            out.push(' ');
            capitalize = true;
        } else if capitalize {
            out.extend(ch.to_uppercase());
            capitalize = false;
        } else {
            out.push(ch);
        }
    }
    if out.is_empty() {
        FALLBACK_TITLE_INDEX.to_string()
    } else {
        out
    }
}

/// Splits a markdown source into (frontmatter YAML value, body).
/// # Arguments
///
/// - `&str` - the raw markdown source
///
/// # Returns
///
/// - `(Value, &str)` - the parsed frontmatter YAML and the body that follows it;
///   `(Value::Null, raw)` when there is no frontmatter block
fn split_frontmatter(raw: &str) -> (Value, &str) {
    let trimmed: &str = raw.trim_start();
    if !trimmed.starts_with("---") {
        return (Value::Null, raw);
    }
    let after_open: &str = &trimmed[3..];
    let Some(after_open) = after_open.strip_prefix(['\n', '\r'].as_ref()) else {
        return (Value::Null, raw);
    };
    let Some(end) = after_open.find(FRONTMATTER_TERMINATOR) else {
        return (Value::Null, raw);
    };
    let fm_src: &str = &after_open[..end];
    let body: &str = &after_open[end + 4..];
    let yaml: Value = serde_yaml::from_str(fm_src).unwrap_or(Value::Null);
    (yaml, body)
}

/// Reads a string field from a YAML mapping.
/// # Arguments
///
/// - `&Value` - the YAML mapping to read from
/// - `&str` - the key to read
///
/// # Returns
///
/// - `Option<String>` - the string value at `key`, or `None` when the key is
///   absent or is not a string
fn yaml_str(value: &Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(|v: &Value| v.as_str())
        .map(|s: &str| s.to_string())
}

/// Reads an i64 field from a YAML mapping.
/// # Arguments
///
/// - `&Value` - the YAML mapping to read from
/// - `&str` - the key to read
///
/// # Returns
///
/// - `Option<i64>` - the integer value at `key`, or `None` when the key is
///   absent or is not an integer
fn yaml_i64(value: &Value, key: &str) -> Option<i64> {
    value.get(key).and_then(|v: &Value| v.as_i64())
}

/// Reads a bool field from a YAML mapping.
/// # Arguments
///
/// - `&Value` - the YAML mapping to read from
/// - `&str` - the key to read
///
/// # Returns
///
/// - `bool` - the boolean value at `key`, or `false` when the key is absent
///   or is not a boolean
fn yaml_bool(value: &Value, key: &str) -> bool {
    value
        .get(key)
        .and_then(|v: &Value| v.as_bool())
        .unwrap_or(false)
}

/// Reads a list field from a YAML mapping.
/// # Arguments
///
/// - `&'a Value` - the YAML mapping to read from
/// - `&str` - the key to read
///
/// # Returns
///
/// - `&'a [Value]` - the sequence value at `key`, or an empty slice when the key
///   is absent or is not a sequence
fn yaml_list<'a>(value: &'a Value, key: &str) -> &'a [Value] {
    value
        .get(key)
        .and_then(|v: &Value| v.as_sequence())
        .map(|s: &Vec<Value>| s.as_slice())
        .unwrap_or(&[])
}

/// Detects whether a markdown frontmatter marks the page as private.
///
/// Three equivalent shapes are recognised, mirroring the conventions
/// used by the `docs-pages/docs` VuePress site and the euv-docs
/// extension added on top:
///
/// 1. The **docs-pages convention** — a `head` array containing nested
///    arrays that wrap `<meta>` entries. A page is private when at
///    least one of those metas carries `name: keywords` and a
///    `content` value that contains the literal `private` substring
///    (case-insensitive). Example:
///
///    ```yaml
///    head:
///      - - meta
///        - name: keywords
///          content: private
///    ```
///
/// 2. A top-level `category` field whose string or array contains
///    `private`. Same VuePress-shape, different field name. Example:
///
///    ```yaml
///    category: private          # or:
///    category: [some, private]  # any entry equal to "private"
///    ```
///
/// 3. A short-hand `private: true` boolean for files that don't
///    already follow the head-meta / category shape. euv-docs adds
///    this so a brand-new markdown can opt into the gate without
///    having to learn the head-array syntax.
///
/// Whichever marker the page uses, [`process_page`] requires a
/// non-empty `password:` field next to it — the build panics
/// otherwise, on the principle that the user explicitly opted into
/// a gate and a silent fall-back would be worse than a hard failure.
/// # Arguments
///
/// - `&Value` - the page frontmatter to inspect
///
/// # Returns
///
/// - `bool` - `true` when any of the three recognised private markers is present
fn is_private_page(value: &Value) -> bool {
    if value
        .get(MARKER_PRIVATE)
        .and_then(|v: &Value| v.as_bool())
        .unwrap_or(false)
    {
        return true;
    }
    if let Some(category) = value.get(YAML_CATEGORY) {
        match category {
            Value::String(s) => {
                if s.split(',')
                    .any(|t: &str| t.trim().eq_ignore_ascii_case(MARKER_PRIVATE))
                {
                    return true;
                }
            }
            Value::Sequence(seq) => {
                for item in seq {
                    if let Some(s) = item.as_str()
                        && s.trim().eq_ignore_ascii_case(MARKER_PRIVATE)
                    {
                        return true;
                    }
                }
            }
            _ => {}
        }
    }
    if let Some(head) = value.get(YAML_HEAD).and_then(|v: &Value| v.as_sequence()) {
        for entry in head {
            let Some(entry_seq) = entry.as_sequence() else {
                continue;
            };
            let mut iter: std::slice::Iter<'_, Value> = entry_seq.iter();
            let Some(first) = iter.next() else {
                continue;
            };
            if first.as_str() != Some(HEAD_ENTRY_META) {
                continue;
            }
            for attrs in iter {
                let Some(attrs_map) = attrs.as_mapping() else {
                    continue;
                };
                let name: &str = attrs_map
                    .get(Value::String(HEAD_ATTR_NAME.to_string()))
                    .and_then(|v: &Value| v.as_str())
                    .unwrap_or("");
                let content: &str = attrs_map
                    .get(Value::String(HEAD_ATTR_CONTENT.to_string()))
                    .and_then(|v: &Value| v.as_str())
                    .unwrap_or("");
                if name.eq_ignore_ascii_case(HEAD_ATTR_KEYWORDS)
                    && content
                        .split(|c: char| c == ',' || c.is_whitespace())
                        .any(|t: &str| t.trim().eq_ignore_ascii_case(MARKER_PRIVATE))
                {
                    return true;
                }
            }
        }
    }
    false
}

/// Renders markdown source into a block AST, extracting headings + first h1.
/// # Arguments
///
/// - `&str` - the markdown body, frontmatter already stripped
/// - `&str` - the page route, used to build heading permalinks
///
/// # Returns
///
/// - `(Vec<AstBlock>, Vec<Heading>, Option<String>)` - the block AST, the h2/h3 anchor TOC entries, and the first h1 text
fn render_markdown(body: &str, route: &str) -> (Vec<AstBlock>, Vec<Heading>, Option<String>) {
    let segments: Vec<Segment> = split_containers(&transform_github_alerts(body));
    let mut blocks: Vec<AstBlock> = Vec::new();
    let mut headings: Vec<Heading> = Vec::new();
    let mut first_h1: Option<String> = None;
    let mut used_slugs: HashSet<String> = HashSet::new();
    // Tracks whether the leading h1 has been consumed, carried across
    // segments so an h1 at the top of a later segment is still dropped.
    let mut h1_done: bool = false;

    for segment in segments {
        match segment {
            Segment::Markdown(src) => {
                let mut ctx: ParseCtx = ParseCtx {
                    route,
                    headings: &mut headings,
                    first_h1: &mut first_h1,
                    used_slugs: &mut used_slugs,
                    h1_seen: h1_done,
                };
                blocks.extend(parse_blocks(&src, &mut ctx));
                h1_done = ctx.h1_seen;
            }
            Segment::Container { kind, title, body } => {
                let mut ctx: ParseCtx = ParseCtx {
                    route,
                    headings: &mut headings,
                    first_h1: &mut first_h1,
                    used_slugs: &mut used_slugs,
                    h1_seen: h1_done,
                };
                let inner: Vec<AstBlock> = parse_blocks(&body, &mut ctx);
                h1_done = ctx.h1_seen;
                blocks.push(AstBlock::Container {
                    title: title.unwrap_or_else(|| kind.to_uppercase()),
                    kind,
                    blocks: inner,
                });
            }
        }
    }
    (blocks, headings, first_h1)
}

/// Shared mutable parse state.
struct ParseCtx<'a> {
    /// Current page route (for link rewriting).
    route: &'a str,
    /// TOC sink.
    headings: &'a mut Vec<Heading>,
    /// First h1 text sink.
    first_h1: &'a mut Option<String>,
    /// Slug dedup set.
    used_slugs: &'a mut HashSet<String>,
    /// Whether the first h1 has already been consumed.
    ///
    /// The page shell renders the title as an `<h1>` of its own, so a
    /// leading `# Title` in the markdown body would show the same words
    /// twice with a divider between them. The first h1 is captured into
    /// `first_h1` (as the title fallback) and then dropped from the
    /// block stream. Only the leading one is dropped — a second h1 later
    /// in the body is a real section heading and is kept.
    h1_seen: bool,
}

/// A source segment: plain markdown or a `:::` custom container.
enum Segment {
    /// Plain markdown.
    Markdown(String),
    /// `::: kind [title]` container.
    Container {
        /// Container kind (tip / warning / danger / …).
        kind: String,
        /// Optional custom title.
        title: Option<String>,
        /// Raw markdown body.
        body: String,
    },
}

/// Rewrites `> [!TIP]` / `> [!NOTE]` / `> [!WARNING]` / `> [!DANGER]`
/// / `> [!IMPORTANT]` / `> [!CAUTION]` blockquotes into the
/// `::: kind [title]\n…\n:::` form so `split_containers` picks them up.
/// # Arguments
///
/// - `&str` - the markdown body to rewrite
///
/// # Returns
///
/// - `String` - the body with every GitHub alert blockquote replaced by the
///   equivalent `:::` container block
fn transform_github_alerts(body: &str) -> String {
    let mut out: String = String::with_capacity(body.len());
    let lines: Vec<&str> = body.lines().collect();
    let mut idx: usize = 0;
    while idx < lines.len() {
        let line: &str = lines[idx];
        let stripped: &str = line.trim_start().strip_prefix('>').unwrap_or("");
        if !line.trim_start().starts_with('>') || !stripped.trim_start().starts_with("[!") {
            out.push_str(line);
            out.push('\n');
            idx += 1;
            continue;
        }
        let after_marker: &str = stripped.trim_start().trim_start_matches("[!");
        let close: Option<usize> = after_marker.find(']');
        let Some(close) = close else {
            out.push_str(line);
            out.push('\n');
            idx += 1;
            continue;
        };
        let kind_candidate: String = after_marker[..close].trim().to_ascii_lowercase();
        if !GITHUB_ALERT_KINDS.contains(&kind_candidate.as_str()) {
            out.push_str(line);
            out.push('\n');
            idx += 1;
            continue;
        };
        // GitHub alerts carry no title syntax — text after `]` on the
        // marker line is the only accepted custom title. The following
        // `>` lines are always body content; treating the first of them
        // as the title would hijack content like `> [!tip]\n> LTPP
        // \`WEB\` …` into an unparsed (and uppercased) title label.
        let inline_title: &str = after_marker[close + 1..].trim();
        out.push_str(&format!("::: {kind_candidate}"));
        if !inline_title.is_empty() {
            out.push(' ');
            out.push_str(inline_title);
        }
        out.push('\n');
        let mut body_idx: usize = idx + 1;
        while body_idx < lines.len() {
            let next: &str = lines[body_idx];
            if let Some(rest) = next.trim_start().strip_prefix('>') {
                out.push_str(rest.trim_start());
                out.push('\n');
                body_idx += 1;
            } else {
                break;
            }
        }
        out.push_str(CONTAINER_FENCE_CLOSE);
        idx = body_idx;
    }
    out
}

/// Splits source into markdown / container segments (containers do not nest).
/// # Arguments
///
/// - `&str` - the markdown source to split
///
/// # Returns
///
/// - `Vec<Segment>` - one segment per markdown run or `:::` container, in source order
fn split_containers(src: &str) -> Vec<Segment> {
    let mut segments: Vec<Segment> = Vec::new();
    let mut buf: String = String::new();
    let mut in_container: bool = false;
    let mut kind: String = String::new();
    let mut title: Option<String> = None;
    let mut body: String = String::new();

    for line in src.lines() {
        let trimmed: &str = line.trim_end();
        if !in_container && trimmed.starts_with(CONTAINER_FENCE) {
            let rest: &str = trimmed[3..].trim();
            if rest.is_empty() {
                buf.push_str(line);
                buf.push('\n');
                continue;
            }
            if !buf.trim().is_empty() {
                segments.push(Segment::Markdown(std::mem::take(&mut buf)));
            } else {
                buf.clear();
            }
            let mut parts: std::str::SplitN<'_, fn(char) -> bool> =
                rest.splitn(2, char::is_whitespace);
            kind = parts.next().unwrap_or(CONTAINER_DEFAULT_KIND).to_string();
            title = parts
                .next()
                .map(str::trim)
                .filter(|t: &&str| !t.is_empty())
                .map(str::to_string);
            in_container = true;
            body.clear();
            continue;
        }
        if in_container && trimmed == CONTAINER_FENCE {
            segments.push(Segment::Container {
                kind: std::mem::take(&mut kind),
                title: title.take(),
                body: std::mem::take(&mut body),
            });
            in_container = false;
            continue;
        }
        if in_container {
            body.push_str(line);
            body.push('\n');
        } else {
            buf.push_str(line);
            buf.push('\n');
        }
    }
    if in_container {
        buf.push_str(&format!("::: {kind}\n{body}"));
    }
    if !buf.trim().is_empty() {
        segments.push(Segment::Markdown(buf));
    }
    segments
}

/// Which block-level end tag terminates the current parse frame.
#[derive(Clone, Copy, PartialEq)]
enum EndCtx {
    /// Top level (never ends early).
    Top,
    /// `BlockQuote`.
    Quote,
    /// `Item` (list item).
    Item,
}

/// Parses a block sequence until the matching end tag.
/// # Arguments
///
/// - `&str` - the markdown source of the block run to parse
/// - `&mut ParseCtx` - the shared parse state collecting headings, the first h1 and used slugs
///
/// # Returns
///
/// - `Vec<AstBlock>` - the parsed block sequence
fn parse_blocks(src: &str, ctx: &mut ParseCtx) -> Vec<AstBlock> {
    let options: Options = Options::ENABLE_TABLES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_HEADING_ATTRIBUTES;
    let events: VecDeque<Event> = Parser::new_ext(src, options).collect();
    let mut iter: EventIter<'_> = events.into_iter().peekable();
    parse_block_stream(&mut iter, ctx, EndCtx::Top)
}

/// The peekable event iterator type used across the parser.
type EventIter<'a> = std::iter::Peekable<std::collections::vec_deque::IntoIter<Event<'a>>>;

/// Whether an event starts an inline run (tight list items have no
/// paragraph wrapper, so inline events can appear at block level).
/// # Arguments
///
/// - `&Event` - the event to classify
///
/// # Returns
///
/// - `bool` - `true` when the event opens an inline run, including the inline
///   container tags
fn is_inline_event(event: &Event) -> bool {
    matches!(
        event,
        Event::Text(_)
            | Event::Code(_)
            | Event::TaskListMarker(_)
            | Event::SoftBreak
            | Event::HardBreak
            | Event::InlineHtml(_)
            | Event::FootnoteReference(_)
            | Event::Start(
                Tag::Strong
                    | Tag::Emphasis
                    | Tag::Strikethrough
                    | Tag::Link { .. }
                    | Tag::Image { .. }
            )
    )
}

/// Core block-stream parser.
/// # Arguments
///
/// - `&mut EventIter` - the peekable event stream, advanced as blocks are parsed
/// - `&mut ParseCtx` - the shared parse state collecting headings, the first h1 and used slugs
/// - `EndCtx` - which block-level end tag terminates this parse frame
///
/// # Returns
///
/// - `Vec<AstBlock>` - the parsed block sequence, up to and including the frame's end tag
fn parse_block_stream(it: &mut EventIter, ctx: &mut ParseCtx, end: EndCtx) -> Vec<AstBlock> {
    let mut blocks: Vec<AstBlock> = Vec::new();
    while let Some(event) = it.next() {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                let inline: Vec<Inline> = parse_inlines(it, ctx, true);
                let text: String = inline_plain_text(&inline);
                let slug: String = unique_slug(&slugify(&text), ctx.used_slugs);
                let n: u8 = heading_level_num(level);
                if n == 2 || n == 3 {
                    ctx.headings.push(Heading {
                        level: n,
                        id: slug.clone(),
                        text: text.clone(),
                    });
                }
                if n == 1 && !ctx.h1_seen {
                    // The leading h1 becomes the page title, which the
                    // shell already renders as its own `<h1>`. Emitting
                    // the block too would print the title twice with a
                    // heading rule between the copies.
                    ctx.h1_seen = true;
                    if ctx.first_h1.is_none() {
                        *ctx.first_h1 = Some(text);
                    }
                    continue;
                }
                blocks.push(AstBlock::Heading {
                    level: n,
                    href: format!("#{}#{}", ctx.route, slug),
                    id: slug,
                    inline,
                });
            }
            Event::Start(Tag::Paragraph) => {
                let inline: Vec<Inline> = parse_inlines(it, ctx, true);
                blocks.push(AstBlock::Paragraph(inline));
            }
            Event::Start(Tag::CodeBlock(kind)) => {
                let lang: String = match &kind {
                    pulldown_cmark::CodeBlockKind::Fenced(lang) => lang.to_string(),
                    pulldown_cmark::CodeBlockKind::Indented => String::new(),
                };
                let mut code: String = String::new();
                for ev in &mut *it {
                    match ev {
                        Event::Text(text) => code.push_str(&text),
                        Event::End(TagEnd::CodeBlock) => break,
                        _ => {}
                    }
                }
                blocks.push(AstBlock::CodeBlock { lang, code });
            }
            Event::Start(Tag::BlockQuote(_)) => {
                let inner: Vec<AstBlock> = parse_block_stream(it, ctx, EndCtx::Quote);
                blocks.push(AstBlock::BlockQuote(inner));
            }
            Event::Start(Tag::List(first)) => {
                let ordered: bool = first.is_some();
                let mut items: Vec<Vec<AstBlock>> = Vec::new();
                loop {
                    match it.next() {
                        Some(Event::Start(Tag::Item)) => {
                            items.push(parse_block_stream(it, ctx, EndCtx::Item));
                        }
                        Some(Event::End(TagEnd::List(_))) | None => break,
                        _ => {}
                    }
                }
                blocks.push(AstBlock::List { ordered, items });
            }
            Event::Start(Tag::Table(_alignments)) => {
                let mut head: Vec<Vec<Inline>> = Vec::new();
                let mut rows: Vec<Vec<Vec<Inline>>> = Vec::new();
                loop {
                    match it.next() {
                        Some(Event::Start(Tag::TableHead)) => {
                            head = parse_table_row(it, ctx);
                        }
                        Some(Event::Start(Tag::TableRow)) => {
                            rows.push(parse_table_row(it, ctx));
                        }
                        Some(Event::End(TagEnd::Table)) | None => break,
                        _ => {}
                    }
                }
                blocks.push(AstBlock::Table { head, rows });
            }
            Event::Rule => blocks.push(AstBlock::Rule),
            Event::Html(html) | Event::InlineHtml(html) => {
                blocks.push(AstBlock::Html(rewrite_html_asset_src(&html)));
            }
            Event::Start(Tag::FootnoteDefinition(name)) => {
                let mut inner: Vec<AstBlock> = parse_block_stream(it, ctx, EndCtx::Top);
                inner.insert(
                    0,
                    AstBlock::Paragraph(vec![Inline::Text(format!("[^{name}]"))]),
                );
                blocks.push(AstBlock::BlockQuote(inner));
            }
            Event::End(tag_end) => {
                let matches_end: bool = matches!(
                    (end, tag_end),
                    (EndCtx::Quote, TagEnd::BlockQuote(_)) | (EndCtx::Item, TagEnd::Item)
                );
                if matches_end {
                    break;
                }
            }
            other if is_inline_event(&other) => {
                let mut inlines: Vec<Inline> = Vec::new();
                collect_inline(it, ctx, other, &mut inlines);
                inlines.extend(parse_inlines(it, ctx, false));
                blocks.push(AstBlock::Paragraph(inlines));
            }
            _ => {}
        }
    }
    blocks
}

/// Parses one table row (head or body) into a vector of cell inlines.
/// # Arguments
///
/// - `&mut EventIter` - the peekable event stream, advanced as cells are parsed
/// - `&mut ParseCtx` - the shared parse state collecting headings, the first h1 and used slugs
///
/// # Returns
///
/// - `Vec<Vec<Inline>>` - the parsed inline nodes of each cell in the row
fn parse_table_row(it: &mut EventIter, ctx: &mut ParseCtx) -> Vec<Vec<Inline>> {
    let mut cells: Vec<Vec<Inline>> = Vec::new();
    loop {
        match it.next() {
            Some(Event::Start(Tag::TableCell)) => {
                cells.push(parse_inlines(it, ctx, true));
            }
            Some(Event::End(TagEnd::TableHead | TagEnd::TableRow)) | None => break,
            _ => {}
        }
    }
    cells
}

/// Parses an inline sequence until the current block-level end tag.
///
/// When `consume_end` is false the terminating `End` event is left on the
/// iterator (used for tight list items whose inline run ends at `End(Item)`).
/// # Arguments
///
/// - `&mut EventIter` - the peekable event stream, advanced as inlines are parsed
/// - `&mut ParseCtx` - the shared parse state collecting headings, the first h1 and used slugs
/// - `bool` - whether the terminating `End` event is consumed or left on the iterator
///
/// # Returns
///
/// - `Vec<Inline>` - the parsed inline nodes, after the `**strong**` rescue pass
fn parse_inlines(it: &mut EventIter, ctx: &mut ParseCtx, consume_end: bool) -> Vec<Inline> {
    let mut inlines: Vec<Inline> = Vec::new();
    loop {
        match it.peek() {
            Some(Event::End(_)) => {
                if consume_end {
                    it.next();
                }
                break;
            }
            None => break,
            _ => {}
        }
        // `peek` said there is an event, so `next` must have one too. Taking
        // it as `let ... else` rather than `expect` keeps a malformed event
        // stream from aborting the whole doc build: an unparseable tail ends
        // the inline run instead of panicking with no location.
        let Some(event): Option<Event> = it.next() else {
            break;
        };
        collect_inline(it, ctx, event, &mut inlines);
    }
    rescue_strong(inlines)
}

/// Rescue pass for `**strong**` spans that pulldown-cmark leaves literal
/// when CJK prose touches the delimiters: CommonMark flanking rules reject
/// a `**` opener that follows a CJK letter and precedes punctuation such
/// as `"`, so `而在于**"…"**` renders as raw asterisks. The pass coalesces
/// the collected inline list, splits every `**` inside text nodes into a
/// delimiter token and wraps nearest-pair contents in `Inline::Strong`.
/// Code spans and other non-text inlines are opaque to the scan, so a
/// literal `**` inside code is never rewritten.
/// # Arguments
///
/// - `Vec<Inline>` - the inline nodes to coalesce and re-parse
///
/// # Returns
///
/// - `Vec<Inline>` - the same nodes with every literal `**`-delimited span that holds
///   content wrapped in `Inline::Strong`
fn rescue_strong(inlines: Vec<Inline>) -> Vec<Inline> {
    enum Tok {
        El(Inline),
        Delim,
    }
    let mut merged: Vec<Inline> = Vec::new();
    for inline in inlines {
        match (merged.last_mut(), inline) {
            (Some(Inline::Text(prev)), Inline::Text(text)) => prev.push_str(&text),
            (_, other) => merged.push(other),
        }
    }
    let mut toks: Vec<Option<Tok>> = Vec::new();
    for inline in merged {
        match inline {
            Inline::Text(text) => {
                let mut rest: &str = &text;
                while let Some(pos) = rest.find("**") {
                    if pos > 0 {
                        toks.push(Some(Tok::El(Inline::Text(rest[..pos].to_string()))));
                    }
                    toks.push(Some(Tok::Delim));
                    rest = &rest[pos + 2..];
                }
                if !rest.is_empty() {
                    toks.push(Some(Tok::El(Inline::Text(rest.to_string()))));
                }
            }
            other => toks.push(Some(Tok::El(other))),
        }
    }
    if !toks
        .iter()
        .any(|t: &Option<Tok>| matches!(t, Some(Tok::Delim)))
    {
        return toks
            .into_iter()
            .filter_map(|t: Option<Tok>| match t {
                Some(Tok::El(inline)) => Some(inline),
                _ => None,
            })
            .collect();
    }
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    let mut open: Option<usize> = None;
    for idx in 0..toks.len() {
        if !matches!(toks[idx], Some(Tok::Delim)) {
            continue;
        }
        match open {
            None => open = Some(idx),
            Some(o) => {
                let has_content: bool = toks[o + 1..idx].iter().any(|t: &Option<Tok>| match t {
                    Some(Tok::El(Inline::Text(s))) => !s.trim().is_empty(),
                    Some(Tok::El(Inline::SoftBreak)) | Some(Tok::El(Inline::HardBreak)) => false,
                    Some(Tok::El(_)) => true,
                    Some(Tok::Delim) => false,
                    None => false,
                });
                if has_content {
                    pairs.push((o, idx));
                    open = None;
                } else {
                    open = Some(idx);
                }
            }
        }
    }
    let mut out: Vec<Inline> = Vec::new();
    let mut i: usize = 0;
    while i < toks.len() {
        let close: Option<usize> = pairs
            .iter()
            .find(|(o, _): &&(usize, usize)| *o == i)
            .map(|(_, c): &(usize, usize)| *c);
        match close {
            Some(c) => {
                let mut children: Vec<Inline> = Vec::new();
                for t in toks[i + 1..c].iter_mut() {
                    match t.take() {
                        Some(Tok::El(inline)) => children.push(inline),
                        Some(Tok::Delim) => children.push(Inline::Text("**".to_string())),
                        None => {}
                    }
                }
                out.push(Inline::Strong(children));
                i = c + 1;
            }
            None => {
                match toks[i].take() {
                    Some(Tok::El(inline)) => out.push(inline),
                    Some(Tok::Delim) => out.push(Inline::Text("**".to_string())),
                    None => {}
                }
                i += 1;
            }
        }
    }
    out
}

/// Converts one event into inline nodes, recursing for container tags.
/// # Arguments
///
/// - `&mut EventIter` - the peekable event stream, advanced for container tags
/// - `&mut ParseCtx` - the shared parse state, used for link and image rewriting
/// - `Event` - the single event to convert into inline nodes
/// - `&mut Vec<Inline>` - the sink the resulting inline nodes are pushed into
///
fn collect_inline(it: &mut EventIter, ctx: &mut ParseCtx, event: Event, inlines: &mut Vec<Inline>) {
    match event {
        Event::Text(text) => inlines.push(Inline::Text(text.to_string())),
        Event::Code(code) => inlines.push(Inline::Code(code.to_string())),
        Event::Start(Tag::Strong) => {
            inlines.push(Inline::Strong(parse_inlines(it, ctx, true)));
        }
        Event::Start(Tag::Emphasis) => {
            inlines.push(Inline::Em(parse_inlines(it, ctx, true)));
        }
        Event::Start(Tag::Strikethrough) => {
            inlines.push(Inline::Del(parse_inlines(it, ctx, true)));
        }
        Event::Start(Tag::Link { dest_url, .. }) => {
            let (href, external) = rewrite_link(&dest_url, ctx.route);
            inlines.push(Inline::Link {
                href,
                external,
                children: parse_inlines(it, ctx, true),
            });
        }
        Event::Start(Tag::Image { dest_url, .. }) => {
            let alt_inlines: Vec<Inline> = parse_inlines(it, ctx, true);
            inlines.push(Inline::Image {
                src: rewrite_image_src(&dest_url, ctx.route),
                alt: inline_plain_text(&alt_inlines),
            });
        }
        Event::TaskListMarker(checked) => inlines.push(Inline::TaskMarker(checked)),
        Event::FootnoteReference(name) => {
            inlines.push(Inline::Text(format!("[^{name}]")));
        }
        Event::SoftBreak => inlines.push(Inline::SoftBreak),
        Event::HardBreak => inlines.push(Inline::HardBreak),
        Event::InlineHtml(html) => inlines.push(Inline::Html(rewrite_html_asset_src(&html))),
        _ => {}
    }
}

/// Extracts plain text from inline nodes.
/// # Arguments
///
/// - `&[Inline]` - the inline nodes to flatten
///
/// # Returns
///
/// - `String` - the concatenated plain text, with breaks turned into spaces and
///   the result trimmed
fn inline_plain_text(inlines: &[Inline]) -> String {
    let mut text: String = String::new();
    for inline in inlines {
        match inline {
            Inline::Text(t) | Inline::Code(t) => text.push_str(t),
            Inline::Strong(children) | Inline::Em(children) | Inline::Del(children) => {
                text.push_str(&inline_plain_text(children));
            }
            Inline::Link { children, .. } => text.push_str(&inline_plain_text(children)),
            Inline::SoftBreak | Inline::HardBreak => text.push(' '),
            _ => {}
        }
    }
    text.trim().to_string()
}

/// Converts a heading level enum to a number.
/// # Arguments
///
/// - `HeadingLevel` - the pulldown-cmark heading level
///
/// # Returns
///
/// - `u8` - the level as a number, `1` for `H1` through `6` for `H6`
fn heading_level_num(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

/// Slugifies heading text (keeps CJK characters, VuePress-style).
/// # Arguments
///
/// - `&str` - the heading text to slugify
///
/// # Returns
///
/// - `String` - the lowercase alphanumeric slug with single dashes, or `section`
///   when the text has no slug-safe character at all
fn slugify(text: &str) -> String {
    let mut out: String = String::new();
    let mut last_dash: bool = false;
    for ch in text.chars().flat_map(char::to_lowercase) {
        if ch.is_alphanumeric() {
            out.push(ch);
            last_dash = false;
        } else if !last_dash && !out.is_empty() {
            out.push('-');
            last_dash = true;
        }
    }
    while out.ends_with('-') {
        out.pop();
    }
    if out.is_empty() {
        FALLBACK_SLUG_SECTION.to_string()
    } else {
        out
    }
}

/// Ensures slug uniqueness within a page.
/// # Arguments
///
/// - `&str` - the slug to register
/// - `&mut HashSet<String>` - the per-page set of already-used slugs
///
/// # Returns
///
/// - `String` - the slug, or the first `-N` suffixed variant that is still free
fn unique_slug(base: &str, used: &mut HashSet<String>) -> String {
    if used.insert(base.to_string()) {
        return base.to_string();
    }
    let mut i: usize = 2;
    loop {
        let candidate: String = format!("{base}-{i}");
        if used.insert(candidate.clone()) {
            return candidate;
        }
        i += 1;
    }
}

/// Rewrites a markdown link target into a site URL.
///
/// Returns `(href, external)` where internal hrefs carry the `#` hash-router
/// prefix (plus an optional `#anchor` suffix).
/// # Arguments
///
/// - `&str` - the raw markdown link target
/// - `&str` - the route of the page holding the link
///
/// # Returns
///
/// - `(String, bool)` - the rewritten href and whether the target is an external URL
fn rewrite_link(dest: &str, route: &str) -> (String, bool) {
    if dest.starts_with(URL_SCHEME_HTTP)
        || dest.starts_with(URL_SCHEME_HTTPS)
        || dest.starts_with(URL_SCHEME_MAILTO)
    {
        return (dest.to_string(), true);
    }
    if let Some(anchor) = dest.strip_prefix('#') {
        return (format!("#{route}#{anchor}"), false);
    }
    let (path_part, anchor_part) = match dest.split_once('#') {
        Some((p, a)) => (p, Some(a)),
        None => (dest, None),
    };
    let mut href: String = if path_part.ends_with(MD_SUFFIX) || path_part.ends_with(MD_DIR_SUFFIX) {
        let resolved: String = resolve_relative(route, path_part);
        format!("#{resolved}")
    } else if path_part.starts_with('/') {
        format!("#{path_part}")
    } else {
        dest.to_string()
    };
    if let Some(anchor) = anchor_part
        && (path_part.ends_with(MD_SUFFIX) || path_part.starts_with('/'))
    {
        href = format!("{href}#{anchor}");
    }
    (href, false)
}

/// Rewrites an image `src` so it resolves against the SPA's
/// `<base href="./">` (the deployed `index.html`).
///
/// External URLs (`http://`, `https://`, `data:`) are left untouched.
/// Site-root-absolute paths (`/essay/09-19/img.jpg`) become
/// `./essay/09-19/img.jpg` — the leading slash is dropped so the
/// browser resolves the URL relative to the current page, which is
/// exactly where `copy_doc_assets` (and `copy_dir(docs/public -> www)`)
/// have placed the asset. Without this rewrite the browser fetches
/// `/essay/09-19/img.jpg` and the SPA's nginx rule serves the index
/// shell (0 bytes, text/html), so `<img>.naturalWidth` ends up 0.
/// # Arguments
///
/// - `&str` - the raw markdown image `src`
/// - `&str` - the route of the page holding the image; kept for signature symmetry
///
/// # Returns
///
/// - `String` - the SPA-relative src, or `dest` unchanged when it is an
///   external, inline or object URL
fn rewrite_image_src(dest: &str, _route: &str) -> String {
    if dest.starts_with(URL_SCHEME_HTTP)
        || dest.starts_with(URL_SCHEME_HTTPS)
        || dest.starts_with(URL_SCHEME_DATA)
        || dest.starts_with(URL_SCHEME_BLOB)
    {
        return dest.to_string();
    }
    let trimmed: &str = dest.trim_start_matches('/');
    if trimmed.is_empty() {
        return dest.to_string();
    }
    if dest.starts_with('/') {
        return format!("./{trimmed}");
    }
    dest.to_string()
}

/// Rewrites site-root-absolute `src="/…"` attributes inside raw HTML
/// blocks (e.g. `<img src="/img/logo.png">`) to the SPA-relative form
/// (`./img/…`), mirroring [`rewrite_image_src`] for markdown images so
/// the browser resolves the URL against the deployed `<base href="./">`.
/// Protocol-relative (`//host/x`) and external URLs are left untouched.
/// # Arguments
///
/// - `&str` - the raw inline or block HTML to rewrite
///
/// # Returns
///
/// - `String` - the HTML with every root-absolute `src=` attribute rewritten to
///   the SPA-relative `./…` form
fn rewrite_html_asset_src(html: &str) -> String {
    if !html.contains(HTML_SRC_ATTR) {
        return html.to_string();
    }
    let mut out: String = String::with_capacity(html.len() + 8);
    let mut rest: &str = html;
    while let Some(idx) = rest.find(HTML_SRC_ATTR) {
        out.push_str(&rest[..idx + 4]);
        let after: &str = &rest[idx + 4..];
        match after.strip_prefix(['"', '\'']) {
            Some(quoted) if quoted.starts_with('/') && !quoted.starts_with("//") => {
                out.push_str(&after[..1]);
                out.push('.');
                rest = quoted;
            }
            _ => rest = after,
        }
    }
    out.push_str(rest);
    out
}

/// Resolves a relative markdown path against the current page route.
/// # Arguments
///
/// - `&str` - the route of the page holding the link
/// - `&str` - the relative markdown path from that link
///
/// # Returns
///
/// - `String` - the resolved route the relative path points at
fn resolve_relative(route: &str, rel: &str) -> String {
    let rel: &str = rel.trim_end_matches('/');
    let mut stack: Vec<String> = Vec::new();
    if let Some(stripped) = rel.strip_prefix('/') {
        for seg in stripped.split('/') {
            stack.push(seg.to_string());
        }
        return md_path_to_route(&format!("/{}", stack.join("/")));
    }
    let dir: &str = if route.ends_with('/') {
        route
    } else {
        match route.rfind('/') {
            Some(idx) => &route[..=idx],
            None => "/",
        }
    };
    for seg in dir.trim_matches('/').split('/') {
        if !seg.is_empty() {
            stack.push(seg.to_string());
        }
    }
    for seg in rel.split('/') {
        match seg {
            "" | "." => {}
            ".." => {
                stack.pop();
            }
            s => stack.push(s.to_string()),
        }
    }
    md_path_to_route(&format!("/{}", stack.join("/")))
}

/// Converts a markdown path (`/guide/foo.md`) to a route (`/guide/foo.html`).
/// # Arguments
///
/// - `&str` - the markdown path to convert
///
/// # Returns
///
/// - `String` - the route it maps to: the parent directory for a `README.md` /
///   `.html` path otherwise
fn md_path_to_route(path: &str) -> String {
    let path: &str = path.trim_end_matches('/');
    if path.ends_with(README_MD) || path.ends_with(INDEX_MD) {
        let dir: &str = &path[..path.rfind('/').unwrap_or(0) + 1];
        return dir.to_string();
    }
    if let Some(stripped) = path.strip_suffix(".md") {
        return format!("{stripped}.html");
    }
    path.to_string()
}

/// Recursively builds the sidebar tree for one locale directory.
/// # Arguments
///
/// - `&Path` - the directory whose entries become sidebar items
/// - `&Path` - the locale root that relative paths are kept against
/// - `&str` - the URL prefix of the locale owning `dir`
/// - `&[Page]` - every parsed page, used to resolve titles, routes and ordering
///
/// # Returns
///
/// - `Vec<SideItem>` - the sidebar tree of `dir`, ordered by `sidebar_order`, then
///   frontmatter `order`, then title
fn build_sidebar(dir: &Path, locale_root: &Path, locale: &str, pages: &[Page]) -> Vec<SideItem> {
    let mut items: Vec<(String, SideItem)> = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut entries: Vec<PathBuf> = entries.flatten().map(|e: fs::DirEntry| e.path()).collect();
    entries.sort();

    for path in entries {
        let name: String = path
            .file_name()
            .map(|n: &OsStr| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        if path.is_dir() {
            if name == DIR_PUBLIC && path.parent() == Some(locale_root) {
                continue;
            }
            let children: Vec<SideItem> = build_sidebar(&path, locale_root, locale, pages);
            let rel_segments: Vec<String> = strip_path_prefix(&path, locale_root)
                .components()
                .filter_map(|c: Component<'_>| match c {
                    Component::Normal(s) => Some(s.to_string_lossy().into_owned()),
                    _ => None,
                })
                .collect();
            let mut segs: Vec<String> = rel_segments;
            segs.push(README_MD.to_string());
            let readme_route: String = route_for(&segs, locale);
            let readme_page: Option<&Page> = pages.iter().find(|p: &&Page| p.route == readme_route);
            let index_page: Option<&Page> = readme_page.filter(|page: &&Page| page.index);
            if children.is_empty() {
                // A directory whose only page is its README is still listed as
                // a leaf link; otherwise single-page projects would vanish.
                if let Some(page) = index_page {
                    items.push((
                        name,
                        SideItem {
                            text: page.title.clone(),
                            link: Some(page.route.clone()),
                            children: Vec::new(),
                        },
                    ));
                }
                continue;
            }
            // A README with `index: false` still lends its title to the group
            // but drops the link, so the group only toggles its children.
            let (text, link) = match readme_page {
                Some(page) if page.index => (page.title.clone(), Some(page.route.clone())),
                Some(page) => (page.title.clone(), None),
                None => (prettify(name.clone()), None),
            };
            items.push((
                name,
                SideItem {
                    text,
                    link,
                    children,
                },
            ));
        } else if name.ends_with(MD_SUFFIX) && name != README_MD && name != INDEX_MD {
            let rel_segments: Vec<String> = strip_path_prefix(&path, locale_root)
                .components()
                .filter_map(|c: Component<'_>| match c {
                    Component::Normal(s) => Some(s.to_string_lossy().into_owned()),
                    _ => None,
                })
                .collect();
            let route: String = route_for(&rel_segments, locale);
            let Some(page) = pages.iter().find(|p: &&Page| p.route == route) else {
                continue;
            };
            if !page.sidebar {
                continue;
            }
            let key: String = name.trim_end_matches(".md").to_string();
            items.push((
                key,
                SideItem {
                    text: page.title.clone(),
                    link: Some(route),
                    children: Vec::new(),
                },
            ));
        }
    }

    // VuePress-style explicit ordering: the directory README's frontmatter
    // `sidebar_order: [name, ...]` pins children (directory names or file
    // stems, `.md` suffix optional) into the listed positions; unlisted
    // entries sort after the listed ones by (`order`, title).
    let readme_route: String = {
        let rel_segments: Vec<String> = strip_path_prefix(dir, locale_root)
            .components()
            .filter_map(|c: Component<'_>| match c {
                Component::Normal(s) => Some(s.to_string_lossy().into_owned()),
                _ => None,
            })
            .collect();
        let mut segs: Vec<String> = rel_segments;
        segs.push(README_MD.to_string());
        route_for(&segs, locale)
    };
    let order_list: &[String] = pages
        .iter()
        .find(|p: &&Page| p.route == readme_route)
        .map(|p: &Page| p.sidebar_order.as_slice())
        .unwrap_or(&[]);
    let pin_pos: &dyn Fn(&str) -> Option<i64> = &|key: &str| -> Option<i64> {
        order_list
            .iter()
            .position(|n: &String| {
                let n: &str = n.trim().trim_start_matches("./").trim_end_matches('/');
                let n: &str = n.strip_suffix(".md").unwrap_or(n);
                n == key
            })
            .map(|i: usize| i as i64)
    };

    let order_of: Box<dyn Fn(&SideItem) -> i64> = Box::new(|item: &SideItem| -> i64 {
        item.link
            .as_ref()
            .and_then(|route: &String| pages.iter().find(|p: &&Page| &p.route == route))
            .map(|p: &Page| p.order)
            .unwrap_or(0)
    });
    items.sort_by(|a: &(String, SideItem), b: &(String, SideItem)| {
        pin_pos(&a.0)
            .unwrap_or(i64::MAX)
            .cmp(&pin_pos(&b.0).unwrap_or(i64::MAX))
            .then_with(|| order_of(&a.1).cmp(&order_of(&b.1)))
            .then_with(|| a.1.text.cmp(&b.1.text))
    });
    items
        .into_iter()
        .map(|(_, item): (String, SideItem)| item)
        .collect()
}

/// Emits the generated Rust source.
/// # Arguments
///
/// - `&Config` - the site-level configuration to emit
/// - `&[Page]` - every page to emit
/// - `&[(String, Vec<SideItem>)]` - the `(prefix, sidebar tree)` pair per locale
///
/// # Returns
///
/// - `String` - the generated Rust source defining `crate::data::SITE`
fn codegen(
    config: &Config,
    pinned: &LocaleConfig,
    pages: &[Page],
    sidebars: &[(String, Vec<SideItem>)],
) -> String {
    let mut code: String = String::new();
    code.push_str(GENERATED_BANNER);

    let mut pages_code: String = String::new();
    for page in pages {
        // `index: false` directory READMEs are not emitted: the route 404s
        // exactly like a directory without a README file.
        if !page.index {
            continue;
        }
        let headings: String = page
            .headings
            .iter()
            .map(|h: &Heading| {
                format!(
                    "euv_ui::EuvTocItem {{ level: {}, text: {:?}, href: {:?} }}",
                    h.level,
                    h.text,
                    format!("#{}#{}", page.route, h.id)
                )
            })
            .collect::<Vec<String>>()
            .join(", ");
        let actions: String = page
            .actions
            .iter()
            .map(|(text, link, kind): &(String, String, String)| {
                format!(
                    "euv_ui::EuvHeroAction {{ text: {:?}, link: {:?}, primary: {:?} }}",
                    text,
                    link,
                    kind == ACTION_KIND_PRIMARY
                )
            })
            .collect::<Vec<String>>()
            .join(", ");
        let features: String = page
            .features
            .iter()
            .map(|(icon, title, details, link): &(String, String, String, String)| {
                format!(
                    "crate::data::DocsFeature {{ icon: {:?}, title: {:?}, details: {:?}, link: {:?} }}",
                    icon, title, details, link
                )
            })
            .collect::<Vec<String>>()
            .join(", ");
        let stats: String = page
            .stats
            .iter()
            .map(|(icon, value, label): &(String, String, String)| {
                format!(
                    "crate::data::DocsStat {{ icon: {:?}, value: {:?}, label: {:?} }}",
                    icon, value, label
                )
            })
            .collect::<Vec<String>>()
            .join(", ");
        let blocks: String = emit_blocks(&page.blocks);
        pages_code.push_str(&format!(
            "crate::data::DocsPage {{ route: {:?}, title: {:?}, blocks: {}, headings: &[{}], home: {}, hero_text: {:?}, tagline: {:?}, actions: &[{}], features: &[{}], stats: &[{}], footer: {:?}, private: {}, password_hash: {:?} }},\n",
            page.route,
            page.title,
            blocks,
            headings,
            page.home,
            page.hero_text,
            page.tagline,
            actions,
            features,
            stats,
            page.footer,
            page.private,
            page.password_hash,
        ));
    }

    let mut locales_code: String = String::new();
    // Exactly one locale is compiled into a bundle: the pinned one, with its
    // URL prefix rewritten to `/` and its navbar links already stripped, so
    // the runtime's locale resolution degenerates to a constant.
    for locale in std::slice::from_ref(pinned) {
        let Some((_, items)): Option<&(String, Vec<SideItem>)> = sidebars
            .iter()
            .find(|(prefix, _): &&(String, Vec<SideItem>)| prefix == &locale.prefix)
        else {
            fail(&format!(
                "locale `{}` has no sidebar entry; every configured locale must have one",
                locale.prefix
            ));
        };
        let sidebar_src: &Vec<SideItem> = items;
        let navbar: String = locale
            .navbar
            .as_ref()
            .map(|items: &Vec<NavItemConfig>| {
                items
                    .iter()
                    .map(|item: &NavItemConfig| {
                        format!(
                            "euv_ui::EuvNavbarItem {{ text: {:?}, link: {:?} }}",
                            item.text, item.link
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(", ")
            })
            .unwrap_or_default();
        let sidebar_code: String = emit_sidebar(sidebar_src);
        locales_code.push_str(&format!(
            "crate::data::DocsLocale {{ prefix: {:?}, title: {:?}, footer: {:?}, toc_label: {:?}, prev_label: {:?}, next_label: {:?}, navbar: &[{}], sidebar: {} }},\n",
            locale.prefix,
            locale.title.clone().unwrap_or_default(),
            locale.footer.clone().unwrap_or_default(),
            locale
                .toc_label
                .clone()
                .unwrap_or_else(|| DEFAULT_TOC_LABEL.to_string()),
            locale
                .prev_label
                .clone()
                .unwrap_or_else(|| DEFAULT_PREV_LABEL.to_string()),
            locale
                .next_label
                .clone()
                .unwrap_or_else(|| DEFAULT_NEXT_LABEL.to_string()),
            navbar,
            sidebar_code,
        ));
    }

    code.push_str(&format!(
        "/// The generated site model.\npub static SITE: crate::data::DocsSite = crate::data::DocsSite {{\n    title: {:?},\n    locales: &[{}],\n    pages: &[{}],\n}};\n",
        config.site.title,
        locales_code,
        pages_code,
    ));

    // Language switcher entries: one per unique content directory (aliases
    // deduplicated). `dir` is the bundle directory relative to the site
    // root — "" for the default locale, "en/" for `/en/`, and so on.
    let mut seen_dirs: Vec<&str> = Vec::new();
    let mut languages_code: String = String::new();
    for locale in &config.locales {
        if seen_dirs.contains(&locale.dir.as_str()) {
            continue;
        }
        seen_dirs.push(&locale.dir);
        languages_code.push_str(&format!(
            "crate::data::DocsLanguageLink {{ label: {:?}, dir: {:?} }},",
            locale.label,
            locale_dir_url(&locale.prefix)
        ));
    }
    code.push_str(&format!(
        "/// Every site language for the cross-bundle switcher.\npub(crate) static SITE_LANGUAGES: &[crate::data::DocsLanguageLink] = &[{languages_code}];\n"
    ));
    code.push_str(&format!(
        "/// The bundle directory this build is served from (relative to the site root).\npub(crate) const SITE_LOCALE_DIR: &str = {:?};\n",
        config
            .locales
            .iter()
            .find(|locale: &&LocaleConfig| locale.dir == pinned.dir)
            .map(|locale: &LocaleConfig| locale_dir_url(&locale.prefix))
            .unwrap_or_default()
    ));

    // Boot-time redirects for URL spaces this bundle does not serve: any
    // other locale's prefix (an old single-bundle link, or a mispaste) jumps
    // to that locale's bundle directory; an alias of this bundle's own
    // directory (e.g. `/zh/`) is rewritten in place to the canonical
    // prefix-free route.
    let mut redirects_code: String = String::new();
    for locale in &config.locales {
        if locale.prefix == URL_PREFIX_ROOT {
            continue;
        }
        let from: &str = locale.prefix.trim_end_matches('/');
        let to_dir: String = if locale.dir == pinned.dir {
            String::new()
        } else {
            let canonical: &LocaleConfig = config
                .locales
                .iter()
                .find(|candidate: &&LocaleConfig| candidate.dir == locale.dir)
                .unwrap_or(locale);
            locale_dir_url(&canonical.prefix)
        };
        redirects_code.push_str(&format!(
            "crate::data::DocsRedirect {{ from: {:?}, to_dir: {:?} }},",
            from, to_dir
        ));
    }
    code.push_str(&format!(
        "/// Foreign-prefix redirects applied once at boot.\npub(crate) static SITE_REDIRECTS: &[crate::data::DocsRedirect] = &[{redirects_code}];\n"
    ));
    code
}

/// Emits a `&'static [DocsBlock]` expression.
/// # Arguments
///
/// - `&[AstBlock]` - the block nodes to emit
///
/// # Returns
///
/// - `String` - a `&'static [EuvMdBlock]`-shaped expression for `blocks`
fn emit_blocks(blocks: &[AstBlock]) -> String {
    let inner: String = blocks
        .iter()
        .map(|block: &AstBlock| match block {
            AstBlock::Heading {
                level,
                id,
                href,
                inline,
            } => format!(
                "euv_ui::EuvMdBlock::Heading {{ level: {level}, id: {id:?}, href: {href:?}, inline: {} }}",
                emit_inlines(inline)
            ),
            AstBlock::Paragraph(inline) => {
                format!("euv_ui::EuvMdBlock::Paragraph({})", emit_inlines(inline))
            }
            AstBlock::CodeBlock { lang, code } => {
                format!("euv_ui::EuvMdBlock::CodeBlock {{ lang: {lang:?}, code: {code:?} }}")
            }
            AstBlock::BlockQuote(inner) => {
                format!("euv_ui::EuvMdBlock::BlockQuote({})", emit_blocks(inner))
            }
            AstBlock::List { ordered, items } => {
                let items_code: String = items
                    .iter()
                    .map(|item: &Vec<AstBlock>| emit_blocks(item))
                    .collect::<Vec<String>>()
                    .join(", ");
                format!(
                    "euv_ui::EuvMdBlock::List {{ ordered: {ordered}, items: &[{items_code}] }}"
                )
            }
            AstBlock::Table { head, rows } => {
                let head_code: String = head
                    .iter()
                    .map(|cell: &Vec<Inline>| emit_inlines(cell))
                    .collect::<Vec<String>>()
                    .join(", ");
                let rows_code: String = rows
                    .iter()
                    .map(|row: &Vec<Vec<Inline>>| {
                        let cells: String = row
                            .iter()
                            .map(|cell: &Vec<Inline>| emit_inlines(cell))
                            .collect::<Vec<String>>()
                            .join(", ");
                        format!("&[{cells}]")
                    })
                    .collect::<Vec<String>>()
                    .join(", ");
                format!(
                    "euv_ui::EuvMdBlock::Table {{ head: &[{head_code}], rows: &[{rows_code}] }}"
                )
            }
            AstBlock::Container {
                kind,
                title,
                blocks,
            } => format!(
                "euv_ui::EuvMdBlock::Container {{ kind: {kind:?}, title: {title:?}, blocks: {} }}",
                emit_blocks(blocks)
            ),
            AstBlock::Rule => CODE_MD_BLOCK_RULE.to_string(),
            AstBlock::Html(html) => format!("euv_ui::EuvMdBlock::Html({html:?})"),
        })
        .collect::<Vec<String>>()
        .join(", ");
    format!("&[{inner}]")
}

/// Emits a `&'static [DocsInline]` expression.
/// # Arguments
///
/// - `&[Inline]` - the inline nodes to emit
///
/// # Returns
///
/// - `String` - a `&'static [EuvMdInline]`-shaped expression for `inlines`
fn emit_inlines(inlines: &[Inline]) -> String {
    let inner: String = inlines
        .iter()
        .map(|inline: &Inline| match inline {
            Inline::Text(text) => format!("euv_ui::EuvMdInline::Text({text:?})"),
            Inline::Strong(children) => {
                format!("euv_ui::EuvMdInline::Strong({})", emit_inlines(children))
            }
            Inline::Em(children) => {
                format!("euv_ui::EuvMdInline::Em({})", emit_inlines(children))
            }
            Inline::Del(children) => {
                format!("euv_ui::EuvMdInline::Del({})", emit_inlines(children))
            }
            Inline::Code(code) => format!("euv_ui::EuvMdInline::Code({code:?})"),
            Inline::Link {
                href,
                external,
                children,
            } => format!(
                "euv_ui::EuvMdInline::Link {{ href: {href:?}, external: {external}, children: {} }}",
                emit_inlines(children)
            ),
            Inline::Image { src, alt } => {
                format!("euv_ui::EuvMdInline::Image {{ src: {src:?}, alt: {alt:?} }}")
            }
            Inline::TaskMarker(checked) => {
                format!("euv_ui::EuvMdInline::TaskMarker({checked})")
            }
            Inline::SoftBreak => CODE_MD_INLINE_SOFT_BREAK.to_string(),
            Inline::HardBreak => CODE_MD_INLINE_HARD_BREAK.to_string(),
            Inline::Html(html) => format!("euv_ui::EuvMdInline::Html({html:?})"),
        })
        .collect::<Vec<String>>()
        .join(", ");
    format!("&[{inner}]")
}

/// Recursively emits a sidebar slice expression.
/// # Arguments
///
/// - `&[SideItem]` - the sidebar tree to emit
///
/// # Returns
///
/// - `String` - a `&'static [EuvSidebarItem]`-shaped expression for `items`
fn emit_sidebar(items: &[SideItem]) -> String {
    let inner: String = items
        .iter()
        .map(|item: &SideItem| {
            let link: String = match &item.link {
                Some(route) => format!("Some({route:?})"),
                None => CODE_NONE.to_string(),
            };
            let children: String = emit_sidebar(&item.children);
            format!(
                "euv_ui::EuvSidebarItem {{ text: {:?}, link: {}, children: {} }}",
                item.text, link, children
            )
        })
        .collect::<Vec<String>>()
        .join(", ");
    format!("&[{inner}]")
}

// Hand-rolled so the build script stays free of new third-party
// dependencies (rust-standards §13.1). SHA-256 is small enough that
// embedding the ~80-line reference implementation is cheaper than
// pulling in the `sha2` crate, and it never runs in the browser —
// only at build time, while compiling `docs/*.md` into the generated
// `docs_gen.rs`.

/// Returns the hex-encoded SHA-256 digest of `input`.
/// # Arguments
///
/// - `&str` - the message to hash; hashed as its UTF-8 bytes
///
/// # Returns
///
/// - `String` - the 64-character lowercase hex SHA-256 digest
fn sha256_hex(input: &str) -> String {
    let digest: Sha256Digest = sha256(input.as_bytes());
    let mut out: String = String::with_capacity(64);
    for byte in digest {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

/// The 32-byte raw SHA-256 digest, as returned by [`sha256`].
type Sha256Digest = [u8; 32];

/// Computes the SHA-256 message digest of `message`. Returns the
/// 32-byte raw digest; use [`sha256_hex`] for a printable form.
///
/// Reference: FIPS PUB 180-4 §6.2.
/// # Arguments
///
/// - `&[u8]` - the message bytes to hash
///
/// # Returns
///
/// - `Sha256Digest` - the 32-byte raw digest; use [`sha256_hex`] for a printable form
fn sha256(message: &[u8]) -> Sha256Digest {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];
    const INITIAL: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];
    let bit_len: u64 = (message.len() as u64).wrapping_mul(8);
    let mut padded: Vec<u8> = Vec::with_capacity(message.len() + 1 + 64);
    padded.extend_from_slice(message);
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&bit_len.to_be_bytes());
    let mut hash: [u32; 8] = INITIAL;
    for chunk in padded.chunks(64) {
        let mut w: [u32; 64] = [0; 64];
        for (i, word_bytes) in chunk.as_chunks::<4>().0.iter().enumerate() {
            w[i] = u32::from_be_bytes([word_bytes[0], word_bytes[1], word_bytes[2], word_bytes[3]]);
        }
        for i in 16..64 {
            let s0: u32 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1: u32 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let mut a: u32 = hash[0];
        let mut b: u32 = hash[1];
        let mut c: u32 = hash[2];
        let mut d: u32 = hash[3];
        let mut e: u32 = hash[4];
        let mut f: u32 = hash[5];
        let mut g: u32 = hash[6];
        let mut h: u32 = hash[7];
        for i in 0..64 {
            let s1: u32 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch: u32 = (e & f) ^ ((!e) & g);
            let temp1: u32 = h
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0: u32 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj: u32 = (a & b) ^ (a & c) ^ (b & c);
            let temp2: u32 = s0.wrapping_add(maj);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }
        hash[0] = hash[0].wrapping_add(a);
        hash[1] = hash[1].wrapping_add(b);
        hash[2] = hash[2].wrapping_add(c);
        hash[3] = hash[3].wrapping_add(d);
        hash[4] = hash[4].wrapping_add(e);
        hash[5] = hash[5].wrapping_add(f);
        hash[6] = hash[6].wrapping_add(g);
        hash[7] = hash[7].wrapping_add(h);
    }
    let mut out: Sha256Digest = [0; 32];
    for (i, word) in hash.iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    out
}
