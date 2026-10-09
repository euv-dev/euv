//! String constants extracted from `docs/build.rs` (rust-standards §1.3c).
//!
//! `build.rs` is the crate root of a build script, so this module is
//! declared from it as `mod r#const;` — the `r#` prefix is required
//! because `const` is a Rust keyword. Every string literal that the
//! §1.3c verifier flags in a function body lives here, verbatim and
//! byte-identical to its original inline form.

// ---------------------------------------------------------------------------
// Build-script environment variables
// ---------------------------------------------------------------------------

/// Overrides the docs content root when the build script starts.
pub(crate) const ENV_DOCS_SRC_DIR: &str = "EUV_DOCS_SRC_DIR";

/// Overrides the generated-site output root when the build script starts.
pub(crate) const ENV_DOCS_OUT_DIR: &str = "EUV_DOCS_OUT_DIR";

/// Fatal message emitted when cargo did not provide [`ENV_OUT_DIR`].
pub(crate) const ERROR_OUT_DIR_MISSING: &str =
    "OUT_DIR is not set; a build script must run under cargo";

/// Fatal message emitted when the README frontmatter has no `site` block.
pub(crate) const ERROR_SITE_CONFIG_MISSING: &str =
    "site-level config (site + locales) missing from README.md frontmatter";

/// Fatal message emitted when `locales:` is present but not a YAML sequence.
pub(crate) const ERROR_LOCALES_NOT_A_SEQUENCE: &str =
    "locales: in README.md frontmatter must be a YAML sequence of [[locales]] entries";

/// Fatal message prefix for a `[[locales]]` entry that is missing a
/// required key. The offending entry's `locales:` index and `prefix`
/// value are appended by the build script so the author can locate it.
pub(crate) const ERROR_LOCALES_MALFORMED: &str = "malformed [[locales]] entry";

/// Fatal message emitted when `locales:` parses but carries no entries.
///
/// An empty list used to reach codegen, which emitted `locales: &[]` and
/// made the runtime router index `site.locales[0]` on an empty slice
/// ("index out of bounds: the len is 0 but the index is 0").
pub(crate) const ERROR_LOCALES_EMPTY: &str = "locales: in README.md frontmatter is empty; declare at least one [[locales]] entry \
     with the required keys prefix, dir and label";

// ---------------------------------------------------------------------------
// Directory and file names
// ---------------------------------------------------------------------------

/// Default docs content directory, relative to the crate manifest dir.
pub(crate) const DIR_DOCS: &str = "docs";

/// Site-level asset directory that is copied verbatim and never walked.
pub(crate) const DIR_PUBLIC: &str = "public";

/// Name of the generated Rust source written into the cargo `OUT_DIR`.
pub(crate) const GENERATED_FILE_NAME: &str = "docs_gen.rs";

/// Site configuration README, relative to the docs content root.
pub(crate) const README_RELATIVE_PATH: &str = "../README.md";

/// File name of a directory-index page in the VuePress convention.
pub(crate) const README_MD: &str = "README.md";

/// Alternative file name of a directory-index page.
pub(crate) const INDEX_MD: &str = "index.md";

/// Extension-bearing suffix of a markdown file.
pub(crate) const MD_SUFFIX: &str = ".md";

/// Trailing-slash variant of [`MD_SUFFIX`] used in link targets.
pub(crate) const MD_DIR_SUFFIX: &str = ".md/";

/// Opening fence of a `:::`-delimited custom container.
pub(crate) const CONTAINER_FENCE: &str = ":::";

// ---------------------------------------------------------------------------
// Frontmatter keys
// ---------------------------------------------------------------------------

/// Frontmatter key holding the site title.
pub(crate) const YAML_TITLE: &str = "title";

/// Frontmatter key holding the `[site]` block.
pub(crate) const YAML_SITE: &str = "site";

/// Frontmatter key holding the `[[locales]]` sequence.
pub(crate) const YAML_LOCALES: &str = "locales";

/// Frontmatter key holding a locale's URL prefix.
pub(crate) const YAML_PREFIX: &str = "prefix";

/// Frontmatter key holding a locale's content directory.
pub(crate) const YAML_DIR: &str = "dir";

/// Frontmatter key holding a locale's dropdown label.
pub(crate) const YAML_LABEL: &str = "label";

/// Frontmatter key holding the locale-specific footer text.
pub(crate) const YAML_FOOTER: &str = "footer";

/// Frontmatter key holding the right-TOC title label.
pub(crate) const YAML_TOC_LABEL: &str = "toc_label";

/// Frontmatter key holding the prev-page link label.
pub(crate) const YAML_PREV_LABEL: &str = "prev_label";

/// Frontmatter key holding the next-page link label.
pub(crate) const YAML_NEXT_LABEL: &str = "next_label";

/// Frontmatter key holding the locale's navbar items.
pub(crate) const YAML_NAVBAR: &str = "navbar";

/// Frontmatter key holding a navbar item's display text.
pub(crate) const YAML_TEXT: &str = "text";

/// Frontmatter key holding a navbar item's link target.
pub(crate) const YAML_LINK: &str = "link";

/// Frontmatter key holding a hero action's variant.
pub(crate) const YAML_TYPE: &str = "type";

/// Frontmatter key holding a sidebar sorting weight.
pub(crate) const YAML_ORDER: &str = "order";

/// Frontmatter key marking a page as the locale home page.
pub(crate) const YAML_HOME: &str = "home";

/// Frontmatter key holding the camelCase home hero text.
pub(crate) const YAML_HERO_TEXT_CAMEL: &str = "heroText";

/// Frontmatter key holding the snake_case home hero text.
pub(crate) const YAML_HERO_TEXT_SNAKE: &str = "hero_text";

/// Frontmatter key holding the home tagline.
pub(crate) const YAML_TAGLINE: &str = "tagline";

/// Frontmatter key holding the home hero actions.
pub(crate) const YAML_ACTIONS: &str = "actions";

/// Frontmatter key holding the home feature cards.
pub(crate) const YAML_FEATURES: &str = "features";

/// Frontmatter key holding the home hero stats.
pub(crate) const YAML_STATS: &str = "stats";

/// Frontmatter key holding a feature card's icon name.
pub(crate) const YAML_ICON: &str = "icon";

/// Frontmatter key holding a feature card's body text.
pub(crate) const YAML_DETAILS: &str = "details";

/// Frontmatter key holding a stat tile's numeric value.
pub(crate) const YAML_VALUE: &str = "value";

/// Frontmatter key holding the password that unlocks a private page.
pub(crate) const YAML_PASSWORD: &str = "password";

/// Frontmatter key toggling sidebar visibility of a page.
pub(crate) const YAML_SIDEBAR: &str = "sidebar";

/// Frontmatter key pinning explicit sidebar child ordering.
pub(crate) const YAML_SIDEBAR_ORDER: &str = "sidebar_order";

/// Frontmatter key dropping a directory-index page from codegen.
pub(crate) const YAML_INDEX: &str = "index";

// ---------------------------------------------------------------------------
// Private-page markers
// ---------------------------------------------------------------------------

/// Short-hand `private: true` frontmatter key.
pub(crate) const MARKER_PRIVATE: &str = "private";

/// VuePress-shape field whose value may list `private`.
pub(crate) const YAML_CATEGORY: &str = "category";

/// VuePress-shape field holding nested `<meta>` arrays.
pub(crate) const YAML_HEAD: &str = "head";

/// Tag name identifying a `<meta>` entry inside a `head` array.
pub(crate) const HEAD_ENTRY_META: &str = "meta";

/// Attribute name of a `<meta>` entry.
pub(crate) const HEAD_ATTR_NAME: &str = "name";

/// Attribute value of a `<meta>` entry.
pub(crate) const HEAD_ATTR_CONTENT: &str = "content";

/// `<meta name="keywords">` marker that hides a page behind the password form.
pub(crate) const HEAD_ATTR_KEYWORDS: &str = "keywords";

// ---------------------------------------------------------------------------
// Markdown source syntax
// ---------------------------------------------------------------------------

/// File stem of a `README.md` directory index.
pub(crate) const README_STEM: &str = "README";

/// File stem of an `index.md` directory index.
pub(crate) const INDEX_STEM: &str = "index";

/// Closing fence line of a `:::`-delimited custom container.
pub(crate) const CONTAINER_FENCE_CLOSE: &str = ":::\n";

/// Kind assumed for a `:::` container written without one.
pub(crate) const CONTAINER_DEFAULT_KIND: &str = "info";

/// Frontmatter closing delimiter, including its leading newline.
pub(crate) const FRONTMATTER_TERMINATOR: &str = "\n---";

/// GitHub-flavoured alert kinds accepted on blockquote markers.
pub(crate) const GITHUB_ALERT_KINDS: &[&str] =
    &["tip", "note", "warning", "danger", "important", "caution"];

/// HTML attribute prefix whose value is an asset URL.
pub(crate) const HTML_SRC_ATTR: &str = "src=";

// ---------------------------------------------------------------------------
// URL schemes
// ---------------------------------------------------------------------------

/// Absolute `http://` URL scheme prefix.
pub(crate) const URL_SCHEME_HTTP: &str = "http://";

/// Absolute `https://` URL scheme prefix.
pub(crate) const URL_SCHEME_HTTPS: &str = "https://";

/// `mailto:` URL scheme prefix.
pub(crate) const URL_SCHEME_MAILTO: &str = "mailto:";

/// Inline `data:` URL scheme prefix.
pub(crate) const URL_SCHEME_DATA: &str = "data:";

/// Object-URL `blob:` scheme prefix.
pub(crate) const URL_SCHEME_BLOB: &str = "blob:";

// ---------------------------------------------------------------------------
// Display fallbacks
// ---------------------------------------------------------------------------

/// Title used when a prettified file name collapses to the empty string.
pub(crate) const FALLBACK_TITLE_INDEX: &str = "Index";

/// Slug used when a heading has no slug-safe characters at all.
pub(crate) const FALLBACK_SLUG_SECTION: &str = "section";

/// Default right-TOC title label for a locale that declares none.
pub(crate) const DEFAULT_TOC_LABEL: &str = "On this page";

/// Default prev-page link label for a locale that declares none.
pub(crate) const DEFAULT_PREV_LABEL: &str = "Previous";

/// Default next-page link label for a locale that declares none.
pub(crate) const DEFAULT_NEXT_LABEL: &str = "Next";

/// Frontmatter `type` value that renders a hero action as the primary one.
pub(crate) const ACTION_KIND_PRIMARY: &str = "primary";

// ---------------------------------------------------------------------------
// Generated-source fragments
// ---------------------------------------------------------------------------

/// Banner prepended to the generated `docs_gen.rs`.
pub(crate) const GENERATED_BANNER: &str = "// @generated by build.rs — do not edit.\n//\n// Constructed from docs/config.toml and docs/**/*.md at build time.\n\n";

/// Emitted expression for a markdown thematic break.
pub(crate) const CODE_MD_BLOCK_RULE: &str = "euv_ui::EuvMdBlock::Rule";

/// Emitted expression for a markdown soft break.
pub(crate) const CODE_MD_INLINE_SOFT_BREAK: &str = "euv_ui::EuvMdInline::SoftBreak";

/// Emitted expression for a markdown hard break.
pub(crate) const CODE_MD_INLINE_HARD_BREAK: &str = "euv_ui::EuvMdInline::HardBreak";

/// Emitted `Option::None` expression for a sidebar item without a link.
pub(crate) const CODE_NONE: &str = "None";
