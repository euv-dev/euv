use std::{
    collections::{HashSet, VecDeque},
    env, fs,
    path::{Component, Path, PathBuf},
};

use {
    pulldown_cmark::{Event, HeadingLevel, Options, Parser, Tag, TagEnd},
    serde_yaml::Value as Yaml,
};

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
    /// Site description (meta).
    description: Option<String>,
    /// Emoji / short logo text in the navbar.
    logo: Option<String>,
}

/// One locale entry.
#[derive(Debug)]
struct LocaleConfig {
    /// Route prefix, e.g. `/` or `/zh/`.
    prefix: String,
    /// BCP-47 language tag.
    lang: String,
    /// Human label in the language dropdown, e.g. `简体中文`.
    label: String,
    /// Locale-specific site title override.
    title: Option<String>,
    /// Locale-specific description override.
    description: Option<String>,
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
#[derive(Debug, Clone)]
enum Block {
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
    BlockQuote(Vec<Block>),
    /// List.
    List {
        /// Ordered flag.
        ordered: bool,
        /// Items (each a block list).
        items: Vec<Vec<Block>>,
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
        blocks: Vec<Block>,
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
    /// Locale prefix this page belongs to.
    locale: String,
    /// Page title (frontmatter `title` or first heading).
    title: String,
    /// Content block AST.
    blocks: Vec<Block>,
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
fn main() {
    let manifest_dir: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let docs_dir: PathBuf = match env::var("EUV_DOCS_SRC_DIR") {
        Ok(path) => PathBuf::from(path),
        Err(_) => manifest_dir.join("docs"),
    };
    let www_dir: PathBuf = match env::var("EUV_DOCS_OUT_DIR") {
        Ok(path) => PathBuf::from(path),
        Err(_) => manifest_dir.join("www"),
    };
    let out_dir: String = env::var("OUT_DIR").expect("OUT_DIR");

    println!("cargo:rerun-if-changed={}", docs_dir.display());
    println!("cargo:rerun-if-env-changed=EUV_DOCS_SRC_DIR");
    println!("cargo:rerun-if-env-changed=EUV_DOCS_OUT_DIR");

    let config: Config = load_config_from_readme(&docs_dir).expect(
        "site-level config (site + locales) missing from <SRC_DIR>/../README.md frontmatter",
    );

    let locale_dirs: Vec<String> = config
        .locales
        .iter()
        .filter(|l| l.prefix != "/")
        .map(|l| l.prefix.trim_matches('/').to_string())
        .collect();

    let mut md_files: Vec<PathBuf> = Vec::new();
    collect_md(&docs_dir, &mut md_files);
    md_files.sort();

    let mut pages: Vec<Page> = Vec::new();
    for file in &md_files {
        pages.push(process_page(&docs_dir, file, &locale_dirs));
    }

    let public_dir: PathBuf = docs_dir.join("public");
    if public_dir.is_dir() {
        copy_dir(&public_dir, &www_dir);
    }
    copy_doc_assets(&docs_dir, &www_dir);

    let mut sidebars: Vec<(String, Vec<SideItem>)> = Vec::new();
    for locale in &config.locales {
        let root: PathBuf = if locale.prefix == "/" {
            docs_dir.clone()
        } else {
            docs_dir.join(locale.prefix.trim_matches('/'))
        };
        let items: Vec<SideItem> = build_sidebar(&root, &root, &locale.prefix, &pages);
        sidebars.push((locale.prefix.clone(), items));
    }

    let code: String = codegen(&config, &pages, &sidebars);
    fs::write(PathBuf::from(out_dir).join("docs_gen.rs"), code).expect("write docs_gen.rs");
}

/// Loads the site-level configuration (site + locales) from
/// `<SRC_DIR>/../README.md` frontmatter. Returns `None` when the file is
/// missing or its frontmatter does not contain a `site` block.
fn load_config_from_readme(docs_dir: &Path) -> Option<Config> {
    let readme_path: PathBuf = docs_dir.join("../README.md");
    let raw: String = fs::read_to_string(&readme_path).ok()?;
    let (fm, _body) = split_frontmatter(&raw);
    let site_yaml = fm.get(&Yaml::String("site".to_string()))?;
    let locales_yaml = fm.get(&Yaml::String("locales".to_string()))?;
    let locales_seq: &[Yaml] = locales_yaml.as_sequence()?.as_slice();
    let locales: Vec<LocaleConfig> = locales_seq.iter().filter_map(parse_locale_config).collect();
    Some(Config {
        site: parse_site_config(site_yaml)?,
        locales,
    })
}

fn parse_site_config(yaml: &Yaml) -> Option<SiteConfig> {
    Some(SiteConfig {
        title: yaml_str(yaml, "title")?,
        description: yaml_str(yaml, "description"),
        logo: yaml_str(yaml, "logo"),
    })
}

fn parse_locale_config(yaml: &Yaml) -> Option<LocaleConfig> {
    let navbar_items: Vec<NavItemConfig> = yaml_list(yaml, "navbar")
        .iter()
        .filter_map(|n| {
            Some(NavItemConfig {
                text: yaml_str(n, "text")?,
                link: yaml_str(n, "link")?,
            })
        })
        .collect();
    Some(LocaleConfig {
        prefix: yaml_str(yaml, "prefix")?,
        lang: yaml_str(yaml, "lang")?,
        label: yaml_str(yaml, "label")?,
        title: yaml_str(yaml, "title"),
        description: yaml_str(yaml, "description"),
        footer: yaml_str(yaml, "footer"),
        toc_label: yaml_str(yaml, "toc_label"),
        prev_label: yaml_str(yaml, "prev_label"),
        next_label: yaml_str(yaml, "next_label"),
        navbar: if navbar_items.is_empty() {
            None
        } else {
            Some(navbar_items)
        },
    })
}

/// Recursively collects `*.md` files, skipping `public/`.
fn collect_md(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path: PathBuf = entry.path();
        if path.is_dir() {
            if path.file_name().is_some_and(|n| n == "public") {
                continue;
            }
            collect_md(&path, out);
        } else if path.extension().is_some_and(|e| e == "md") {
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
fn copy_doc_assets(docs_dir: &Path, www_dir: &Path) {
    copy_doc_assets_recurse(docs_dir, docs_dir, www_dir);
}

fn copy_doc_assets_recurse(root: &Path, dir: &Path, www_dir: &Path) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path: PathBuf = entry.path();
        if path.is_dir() {
            if path.file_name().is_some_and(|n| n == "public") {
                continue;
            }
            copy_doc_assets_recurse(root, &path, www_dir);
        } else if path.extension().is_some_and(|e| e != "md") {
            let Ok(rel) = path.strip_prefix(root) else {
                continue;
            };
            let target: PathBuf = www_dir.join(rel);
            if let Some(parent) = target.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let _ = fs::copy(&path, &target);
        }
    }
}

/// Recursively copies a directory tree.
fn copy_dir(src: &Path, dst: &Path) {
    let Ok(entries) = fs::read_dir(src) else {
        return;
    };
    let _ = fs::create_dir_all(dst);
    for entry in entries.flatten() {
        let path: PathBuf = entry.path();
        let target: PathBuf = dst.join(entry.file_name());
        if path.is_dir() {
            copy_dir(&path, &target);
        } else {
            let _ = fs::copy(&path, &target);
        }
    }
}

/// Parses one markdown file into a [`Page`].
fn process_page(docs_dir: &Path, file: &Path, locale_dirs: &[String]) -> Page {
    let raw: String = fs::read_to_string(file).expect("read md");
    let (frontmatter, body) = split_frontmatter(&raw);

    let rel: &Path = file.strip_prefix(docs_dir).expect("strip docs dir");
    let mut segments: Vec<String> = rel
        .components()
        .filter_map(|c| match c {
            Component::Normal(s) => Some(s.to_string_lossy().into_owned()),
            _ => None,
        })
        .collect();

    let locale: String = if let Some(first) = segments.first() {
        if locale_dirs.contains(first) {
            let prefix: String = format!("/{first}/");
            segments.remove(0);
            prefix
        } else {
            "/".to_string()
        }
    } else {
        "/".to_string()
    };

    let route: String = route_for(&segments, &locale);

    let fm_title: Option<String> = yaml_str(&frontmatter, "title");
    let order: i64 = yaml_i64(&frontmatter, "order").unwrap_or(0);

    let (blocks, headings, first_h1) = render_markdown(body, &route);

    let title: String = fm_title
        .or(first_h1)
        .unwrap_or_else(|| prettify(stem_of(&segments)));

    let home: bool = yaml_bool(&frontmatter, "home");
    let hero_text: String = yaml_str(&frontmatter, "heroText")
        .or_else(|| yaml_str(&frontmatter, "hero_text"))
        .unwrap_or_default();
    let tagline: String = yaml_str(&frontmatter, "tagline").unwrap_or_default();
    let footer: String = yaml_str(&frontmatter, "footer").unwrap_or_default();

    let actions: Vec<(String, String, String)> = yaml_list(&frontmatter, "actions")
        .iter()
        .map(|item| {
            (
                yaml_str(item, "text").unwrap_or_default(),
                yaml_str(item, "link").unwrap_or_default(),
                yaml_str(item, "type").unwrap_or_else(|| "primary".to_string()),
            )
        })
        .collect();

    let features: Vec<(String, String, String, String)> = yaml_list(&frontmatter, "features")
        .iter()
        .map(|item| {
            (
                yaml_str(item, "icon").unwrap_or_default(),
                yaml_str(item, "title").unwrap_or_default(),
                yaml_str(item, "details").unwrap_or_default(),
                yaml_str(item, "link").unwrap_or_default(),
            )
        })
        .collect();

    let private: bool = is_private_page(&frontmatter);
    let password_hash: String = if private {
        match yaml_str(&frontmatter, "password") {
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
        locale,
        title,
        blocks,
        headings,
        home,
        hero_text,
        tagline,
        actions,
        features,
        footer,
        order,
        private,
        password_hash,
    }
}

/// Computes the VuePress-style route for a page.
///
/// - `README.md` / `index.md` → directory route with trailing slash.
/// - `foo.md` → `/foo.html`.
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
    if stem == "README" || stem == "index" {
        let base: String = format!("/{dir_path}");
        join_locale_route(locale, &base)
    } else {
        let base: String = format!("/{dir_path}{stem}.html");
        join_locale_route(locale, &base)
    }
}

/// Joins a locale prefix with a base route.
fn join_locale_route(locale: &str, base: &str) -> String {
    if locale == "/" {
        base.to_string()
    } else {
        format!("{}{}", locale.trim_end_matches('/'), base)
    }
}

/// Returns the file stem of the last segment.
fn stem_of(segments: &[String]) -> String {
    segments
        .last()
        .map(|s| s.trim_end_matches(".md").to_string())
        .unwrap_or_default()
}

/// Converts a file/dir name into a human title (`getting-started` → `Getting Started`).
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
        "Index".to_string()
    } else {
        out
    }
}

/// Splits a markdown source into (frontmatter YAML value, body).
fn split_frontmatter(raw: &str) -> (Yaml, &str) {
    let trimmed: &str = raw.trim_start();
    if !trimmed.starts_with("---") {
        return (Yaml::Null, raw);
    }
    let after_open: &str = &trimmed[3..];
    let Some(after_open) = after_open.strip_prefix(['\n', '\r'].as_ref()) else {
        return (Yaml::Null, raw);
    };
    let Some(end) = after_open.find("\n---") else {
        return (Yaml::Null, raw);
    };
    let fm_src: &str = &after_open[..end];
    let body: &str = &after_open[end + 4..];
    let yaml: Yaml = serde_yaml::from_str(fm_src).unwrap_or(Yaml::Null);
    (yaml, body)
}

/// Reads a string field from a YAML mapping.
fn yaml_str(value: &Yaml, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

/// Reads an i64 field from a YAML mapping.
fn yaml_i64(value: &Yaml, key: &str) -> Option<i64> {
    value.get(key).and_then(|v| v.as_i64())
}

/// Reads a bool field from a YAML mapping.
fn yaml_bool(value: &Yaml, key: &str) -> bool {
    value.get(key).and_then(|v| v.as_bool()).unwrap_or(false)
}

/// Reads a list field from a YAML mapping.
fn yaml_list<'a>(value: &'a Yaml, key: &str) -> &'a [Yaml] {
    value
        .get(key)
        .and_then(|v| v.as_sequence())
        .map(|s| s.as_slice())
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
fn is_private_page(value: &Yaml) -> bool {
    if value
        .get("private")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return true;
    }
    if let Some(category) = value.get("category") {
        match category {
            Yaml::String(s) => {
                if s.split(',')
                    .any(|t| t.trim().eq_ignore_ascii_case("private"))
                {
                    return true;
                }
            }
            Yaml::Sequence(seq) => {
                for item in seq {
                    if let Some(s) = item.as_str() {
                        if s.trim().eq_ignore_ascii_case("private") {
                            return true;
                        }
                    }
                }
            }
            _ => {}
        }
    }
    if let Some(head) = value.get("head").and_then(|v| v.as_sequence()) {
        for entry in head {
            let Some(entry_seq) = entry.as_sequence() else {
                continue;
            };
            let mut iter = entry_seq.iter();
            let Some(first) = iter.next() else {
                continue;
            };
            if first.as_str() != Some("meta") {
                continue;
            }
            for attrs in iter {
                let Some(attrs_map) = attrs.as_mapping() else {
                    continue;
                };
                let name = attrs_map
                    .get(&Yaml::String("name".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let content = attrs_map
                    .get(&Yaml::String("content".to_string()))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                if name.eq_ignore_ascii_case("keywords")
                    && content
                        .split(|c: char| c == ',' || c.is_whitespace())
                        .any(|t| t.trim().eq_ignore_ascii_case("private"))
                {
                    return true;
                }
            }
        }
    }
    false
}

/// Renders markdown source into a block AST, extracting headings + first h1.
fn render_markdown(body: &str, route: &str) -> (Vec<Block>, Vec<Heading>, Option<String>) {
    let segments: Vec<Segment> = split_containers(&transform_github_alerts(body));
    let mut blocks: Vec<Block> = Vec::new();
    let mut headings: Vec<Heading> = Vec::new();
    let mut first_h1: Option<String> = None;
    let mut used_slugs: HashSet<String> = HashSet::new();

    for segment in segments {
        match segment {
            Segment::Markdown(src) => {
                let mut ctx: ParseCtx = ParseCtx {
                    route,
                    headings: &mut headings,
                    first_h1: &mut first_h1,
                    used_slugs: &mut used_slugs,
                };
                blocks.extend(parse_blocks(&src, &mut ctx));
            }
            Segment::Container { kind, title, body } => {
                let mut ctx: ParseCtx = ParseCtx {
                    route,
                    headings: &mut headings,
                    first_h1: &mut first_h1,
                    used_slugs: &mut used_slugs,
                };
                let inner: Vec<Block> = parse_blocks(&body, &mut ctx);
                blocks.push(Block::Container {
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

/// Recognised GitHub-flavoured alert kinds accepted on blockquote markers.
const GITHUB_ALERT_KINDS: &[&str] = &["tip", "note", "warning", "danger", "important", "caution"];

/// Rewrites `> [!TIP]` / `> [!NOTE]` / `> [!WARNING]` / `> [!DANGER]`
/// / `> [!IMPORTANT]` / `> [!CAUTION]` blockquotes into the
/// `::: kind [title]\n…\n:::` form so `split_containers` picks them up.
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
        }
        let title_line: Option<String> = (idx + 1 < lines.len())
            .then(|| {
                lines[idx + 1]
                    .trim_start()
                    .strip_prefix('>')
                    .unwrap_or("")
                    .trim()
            })
            .filter(|s: &&str| !s.is_empty() && !s.starts_with("[!"))
            .map(str::to_string);
        out.push_str(&format!("::: {kind_candidate}"));
        if let Some(title) = &title_line {
            out.push(' ');
            out.push_str(title);
        }
        out.push('\n');
        let mut body_idx: usize = idx + 1;
        if title_line.is_some() {
            body_idx += 1;
        }
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
        out.push_str(":::\n");
        idx = body_idx;
    }
    out
}

/// Splits source into markdown / container segments (containers do not nest).
fn split_containers(src: &str) -> Vec<Segment> {
    let mut segments: Vec<Segment> = Vec::new();
    let mut buf: String = String::new();
    let mut in_container: bool = false;
    let mut kind: String = String::new();
    let mut title: Option<String> = None;
    let mut body: String = String::new();

    for line in src.lines() {
        let trimmed: &str = line.trim_end();
        if !in_container && trimmed.starts_with(":::") {
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
            let mut parts = rest.splitn(2, char::is_whitespace);
            kind = parts.next().unwrap_or("info").to_string();
            title = parts
                .next()
                .map(str::trim)
                .filter(|t| !t.is_empty())
                .map(str::to_string);
            in_container = true;
            body.clear();
            continue;
        }
        if in_container && trimmed == ":::" {
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
fn parse_blocks(src: &str, ctx: &mut ParseCtx) -> Vec<Block> {
    let options: Options = Options::ENABLE_TABLES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_HEADING_ATTRIBUTES;
    let events: VecDeque<Event> = Parser::new_ext(src, options).collect();
    let mut iter = events.into_iter().peekable();
    parse_block_stream(&mut iter, ctx, EndCtx::Top)
}

/// The peekable event iterator type used across the parser.
type EventIter<'a> = std::iter::Peekable<std::collections::vec_deque::IntoIter<Event<'a>>>;

/// Whether an event starts an inline run (tight list items have no
/// paragraph wrapper, so inline events can appear at block level).
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
fn parse_block_stream(it: &mut EventIter, ctx: &mut ParseCtx, end: EndCtx) -> Vec<Block> {
    let mut blocks: Vec<Block> = Vec::new();
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
                if n == 1 && ctx.first_h1.is_none() {
                    *ctx.first_h1 = Some(text);
                }
                blocks.push(Block::Heading {
                    level: n,
                    href: format!("#{}#{}", ctx.route, slug),
                    id: slug,
                    inline,
                });
            }
            Event::Start(Tag::Paragraph) => {
                let inline: Vec<Inline> = parse_inlines(it, ctx, true);
                blocks.push(Block::Paragraph(inline));
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
                blocks.push(Block::CodeBlock { lang, code });
            }
            Event::Start(Tag::BlockQuote(_)) => {
                let inner: Vec<Block> = parse_block_stream(it, ctx, EndCtx::Quote);
                blocks.push(Block::BlockQuote(inner));
            }
            Event::Start(Tag::List(first)) => {
                let ordered: bool = first.is_some();
                let mut items: Vec<Vec<Block>> = Vec::new();
                loop {
                    match it.next() {
                        Some(Event::Start(Tag::Item)) => {
                            items.push(parse_block_stream(it, ctx, EndCtx::Item));
                        }
                        Some(Event::End(TagEnd::List(_))) | None => break,
                        _ => {}
                    }
                }
                blocks.push(Block::List { ordered, items });
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
                blocks.push(Block::Table { head, rows });
            }
            Event::Rule => blocks.push(Block::Rule),
            Event::Html(html) | Event::InlineHtml(html) => {
                blocks.push(Block::Html(html.to_string()));
            }
            Event::Start(Tag::FootnoteDefinition(name)) => {
                let mut inner: Vec<Block> = parse_block_stream(it, ctx, EndCtx::Top);
                inner.insert(
                    0,
                    Block::Paragraph(vec![Inline::Text(format!("[^{name}]"))]),
                );
                blocks.push(Block::BlockQuote(inner));
            }
            Event::End(tag_end) => {
                let matches_end: bool = match (end, tag_end) {
                    (EndCtx::Quote, TagEnd::BlockQuote(_)) => true,
                    (EndCtx::Item, TagEnd::Item) => true,
                    _ => false,
                };
                if matches_end {
                    break;
                }
            }
            other if is_inline_event(&other) => {
                let mut inlines: Vec<Inline> = Vec::new();
                collect_inline(it, ctx, other, &mut inlines);
                inlines.extend(parse_inlines(it, ctx, false));
                blocks.push(Block::Paragraph(inlines));
            }
            _ => {}
        }
    }
    blocks
}

/// Parses one table row (head or body) into a vector of cell inlines.
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
        let event: Event = it.next().expect("peeked");
        collect_inline(it, ctx, event, &mut inlines);
    }
    inlines
}

/// Converts one event into inline nodes, recursing for container tags.
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
        Event::InlineHtml(html) => inlines.push(Inline::Html(html.to_string())),
        _ => {}
    }
}

/// Extracts plain text from inline nodes.
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
        "section".to_string()
    } else {
        out
    }
}

/// Ensures slug uniqueness within a page.
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
fn rewrite_link(dest: &str, route: &str) -> (String, bool) {
    if dest.starts_with("http://") || dest.starts_with("https://") || dest.starts_with("mailto:") {
        return (dest.to_string(), true);
    }
    if let Some(anchor) = dest.strip_prefix('#') {
        return (format!("#{route}#{anchor}"), false);
    }
    let (path_part, anchor_part) = match dest.split_once('#') {
        Some((p, a)) => (p, Some(a)),
        None => (dest, None),
    };
    let mut href: String = if path_part.ends_with(".md") || path_part.ends_with(".md/") {
        let resolved: String = resolve_relative(route, path_part);
        format!("#{resolved}")
    } else if path_part.starts_with('/') {
        format!("#{path_part}")
    } else {
        dest.to_string()
    };
    if let Some(anchor) = anchor_part {
        if path_part.ends_with(".md") || path_part.starts_with('/') {
            href = format!("{href}#{anchor}");
        }
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
fn rewrite_image_src(dest: &str, _route: &str) -> String {
    if dest.starts_with("http://")
        || dest.starts_with("https://")
        || dest.starts_with("data:")
        || dest.starts_with("blob:")
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

/// Resolves a relative markdown path against the current page route.
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
fn md_path_to_route(path: &str) -> String {
    let path: &str = path.trim_end_matches('/');
    if path.ends_with("README.md") || path.ends_with("index.md") {
        let dir: &str = &path[..path.rfind('/').unwrap_or(0) + 1];
        return dir.to_string();
    }
    if let Some(stripped) = path.strip_suffix(".md") {
        return format!("{stripped}.html");
    }
    path.to_string()
}

/// Recursively builds the sidebar tree for one locale directory.
fn build_sidebar(dir: &Path, locale_root: &Path, locale: &str, pages: &[Page]) -> Vec<SideItem> {
    let mut items: Vec<SideItem> = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return items;
    };
    let mut entries: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
    entries.sort();

    for path in entries {
        let name: String = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        if path.is_dir() {
            if name == "public" {
                continue;
            }
            let children: Vec<SideItem> = build_sidebar(&path, locale_root, locale, pages);
            if children.is_empty() {
                continue;
            }
            let rel_segments: Vec<String> = path
                .strip_prefix(locale_root)
                .map(|p| {
                    p.components()
                        .filter_map(|c| match c {
                            Component::Normal(s) => Some(s.to_string_lossy().into_owned()),
                            _ => None,
                        })
                        .collect::<Vec<String>>()
                })
                .unwrap_or_default();
            let mut segs: Vec<String> = rel_segments;
            segs.push("README.md".to_string());
            let readme_route: String = route_for(&segs, locale);
            let readme_page: Option<&Page> = pages
                .iter()
                .find(|p| p.route == readme_route && p.locale == locale);
            let (text, link) = match readme_page {
                Some(page) => (page.title.clone(), Some(page.route.clone())),
                None => (prettify(name), None),
            };
            items.push(SideItem {
                text,
                link,
                children,
            });
        } else if name.ends_with(".md") && name != "README.md" && name != "index.md" {
            let rel_segments: Vec<String> = path
                .strip_prefix(locale_root)
                .map(|p| {
                    p.components()
                        .filter_map(|c| match c {
                            Component::Normal(s) => Some(s.to_string_lossy().into_owned()),
                            _ => None,
                        })
                        .collect::<Vec<String>>()
                })
                .unwrap_or_default();
            let route: String = route_for(&rel_segments, locale);
            let Some(page) = pages
                .iter()
                .find(|p| p.route == route && p.locale == locale)
            else {
                continue;
            };
            items.push(SideItem {
                text: page.title.clone(),
                link: Some(route),
                children: Vec::new(),
            });
        }
    }

    let order_of = |item: &SideItem| -> i64 {
        item.link
            .as_ref()
            .and_then(|route| {
                pages
                    .iter()
                    .find(|p| &p.route == route && p.locale == locale)
            })
            .map(|p| p.order)
            .unwrap_or(0)
    };
    items.sort_by(|a, b| {
        order_of(a)
            .cmp(&order_of(b))
            .then_with(|| a.text.cmp(&b.text))
    });
    items
}

/// Emits the generated Rust source.
fn codegen(config: &Config, pages: &[Page], sidebars: &[(String, Vec<SideItem>)]) -> String {
    let mut code: String = String::new();
    code.push_str(
        "// @generated by build.rs — do not edit.\n//\n// Constructed from docs/config.toml and docs/**/*.md at build time.\n\n",
    );

    let mut pages_code: String = String::new();
    for page in pages {
        let headings: String = page
            .headings
            .iter()
            .map(|h| {
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
            .map(|(text, link, kind)| {
                format!(
                    "euv_ui::EuvHeroAction {{ text: {:?}, link: {:?}, primary: {:?} }}",
                    text,
                    link,
                    kind == "primary"
                )
            })
            .collect::<Vec<String>>()
            .join(", ");
        let features: String = page
            .features
            .iter()
            .map(|(icon, title, details, link)| {
                format!(
                    "crate::data::DocsFeature {{ icon: {:?}, title: {:?}, details: {:?}, link: {:?} }}",
                    icon, title, details, link
                )
            })
            .collect::<Vec<String>>()
            .join(", ");
        let blocks: String = emit_blocks(&page.blocks);
        pages_code.push_str(&format!(
            "crate::data::DocsPage {{ route: {:?}, locale: {:?}, title: {:?}, blocks: {}, headings: &[{}], home: {}, hero_text: {:?}, tagline: {:?}, actions: &[{}], features: &[{}], footer: {:?}, private: {}, password_hash: {:?} }},\n",
            page.route,
            page.locale,
            page.title,
            blocks,
            headings,
            page.home,
            page.hero_text,
            page.tagline,
            actions,
            features,
            page.footer,
            page.private,
            page.password_hash,
        ));
    }

    let mut locales_code: String = String::new();
    for locale in &config.locales {
        let sidebar_src: &Vec<SideItem> = sidebars
            .iter()
            .find(|(prefix, _)| prefix == &locale.prefix)
            .map(|(_, items)| items)
            .expect("sidebar for locale");
        let navbar: String = locale
            .navbar
            .as_ref()
            .map(|items| {
                items
                    .iter()
                    .map(|item| {
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
            "crate::data::DocsLocale {{ prefix: {:?}, lang: {:?}, label: {:?}, title: {:?}, description: {:?}, footer: {:?}, toc_label: {:?}, prev_label: {:?}, next_label: {:?}, navbar: &[{}], sidebar: {} }},\n",
            locale.prefix,
            locale.lang,
            locale.label,
            locale.title.clone().unwrap_or_default(),
            locale.description.clone().unwrap_or_default(),
            locale.footer.clone().unwrap_or_default(),
            locale
                .toc_label
                .clone()
                .unwrap_or_else(|| "On this page".to_string()),
            locale
                .prev_label
                .clone()
                .unwrap_or_else(|| "Previous".to_string()),
            locale
                .next_label
                .clone()
                .unwrap_or_else(|| "Next".to_string()),
            navbar,
            sidebar_code,
        ));
    }

    code.push_str(&format!(
        "/// The generated site model.\npub static SITE: crate::data::DocsSite = crate::data::DocsSite {{\n    title: {:?},\n    description: {:?},\n    logo: {:?},\n    locales: &[{}],\n    pages: &[{}],\n}};\n",
        config.site.title,
        config.site.description.clone().unwrap_or_default(),
        config.site.logo.clone().unwrap_or_else(|| "📘".to_string()),
        locales_code,
        pages_code,
    ));
    code
}

/// Emits a `&'static [DocsBlock]` expression.
fn emit_blocks(blocks: &[Block]) -> String {
    let inner: String = blocks
        .iter()
        .map(|block| match block {
            Block::Heading {
                level,
                id,
                href,
                inline,
            } => format!(
                "euv_ui::EuvMdBlock::Heading {{ level: {level}, id: {id:?}, href: {href:?}, inline: {} }}",
                emit_inlines(inline)
            ),
            Block::Paragraph(inline) => {
                format!("euv_ui::EuvMdBlock::Paragraph({})", emit_inlines(inline))
            }
            Block::CodeBlock { lang, code } => {
                format!("euv_ui::EuvMdBlock::CodeBlock {{ lang: {lang:?}, code: {code:?} }}")
            }
            Block::BlockQuote(inner) => {
                format!("euv_ui::EuvMdBlock::BlockQuote({})", emit_blocks(inner))
            }
            Block::List { ordered, items } => {
                let items_code: String = items
                    .iter()
                    .map(|item| emit_blocks(item))
                    .collect::<Vec<String>>()
                    .join(", ");
                format!(
                    "euv_ui::EuvMdBlock::List {{ ordered: {ordered}, items: &[{items_code}] }}"
                )
            }
            Block::Table { head, rows } => {
                let head_code: String = head
                    .iter()
                    .map(|cell| emit_inlines(cell))
                    .collect::<Vec<String>>()
                    .join(", ");
                let rows_code: String = rows
                    .iter()
                    .map(|row| {
                        let cells: String = row
                            .iter()
                            .map(|cell| emit_inlines(cell))
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
            Block::Container {
                kind,
                title,
                blocks,
            } => format!(
                "euv_ui::EuvMdBlock::Container {{ kind: {kind:?}, title: {title:?}, blocks: {} }}",
                emit_blocks(blocks)
            ),
            Block::Rule => "euv_ui::EuvMdBlock::Rule".to_string(),
            Block::Html(html) => format!("euv_ui::EuvMdBlock::Html({html:?})"),
        })
        .collect::<Vec<String>>()
        .join(", ");
    format!("&[{inner}]")
}

/// Emits a `&'static [DocsInline]` expression.
fn emit_inlines(inlines: &[Inline]) -> String {
    let inner: String = inlines
        .iter()
        .map(|inline| match inline {
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
            Inline::SoftBreak => "euv_ui::EuvMdInline::SoftBreak".to_string(),
            Inline::HardBreak => "euv_ui::EuvMdInline::HardBreak".to_string(),
            Inline::Html(html) => format!("euv_ui::EuvMdInline::Html({html:?})"),
        })
        .collect::<Vec<String>>()
        .join(", ");
    format!("&[{inner}]")
}

/// Recursively emits a sidebar slice expression.
fn emit_sidebar(items: &[SideItem]) -> String {
    let inner: String = items
        .iter()
        .map(|item| {
            let link: String = match &item.link {
                Some(route) => format!("Some({route:?})"),
                None => "None".to_string(),
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
fn sha256_hex(input: &str) -> String {
    let digest: [u8; 32] = sha256(input.as_bytes());
    let mut out: String = String::with_capacity(64);
    for byte in digest {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

/// Computes the SHA-256 message digest of `message`. Returns the
/// 32-byte raw digest; use [`sha256_hex`] for a printable form.
///
/// Reference: FIPS PUB 180-4 §6.2.
fn sha256(message: &[u8]) -> [u8; 32] {
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
        for (i, word_bytes) in chunk.chunks_exact(4).enumerate() {
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
    let mut out: [u8; 32] = [0; 32];
    for (i, word) in hash.iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    out
}
