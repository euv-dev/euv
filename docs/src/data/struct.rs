use super::*;

/// One feature card on the home page. Adds a `link` over
/// `euv_ui::EuvFeature`; lives here because `EuvFeature` is frozen on
/// the `euv = "0.18"` pin and cannot be augmented upstream.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct DocsFeature {
    /// Card icon (emoji); the literal string `"blog"` is treated as no
    /// icon by `docs_feature_card`.
    pub(crate) icon: &'static str,
    pub(crate) title: &'static str,
    pub(crate) details: &'static str,
    /// Route or external URL; empty means non-clickable.
    pub(crate) link: &'static str,
}

/// One icon+text stat tile on the home page, rendered between the hero
/// actions and the feature grid (mirrors the euv example home stats row).
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct DocsStat {
    /// Tile icon (emoji).
    pub(crate) icon: &'static str,
    /// Bold stat value.
    pub(crate) value: &'static str,
    /// Muted label under the value.
    pub(crate) label: &'static str,
}

/// One rendered markdown page.
#[derive(Clone, Copy, Debug)]
pub(crate) struct DocsPage {
    /// Full route (`/guide/getting-started.html`, `/zh/` …).
    pub(crate) route: &'static str,
    /// Page title.
    pub(crate) title: &'static str,
    /// Content block AST (rendered by `euv_markdown`).
    pub(crate) blocks: &'static [EuvMdBlock],
    /// Anchor TOC entries.
    pub(crate) headings: &'static [EuvTocItem],
    /// Whether this is a home page.
    pub(crate) home: bool,
    /// Hero text (home pages).
    pub(crate) hero_text: &'static str,
    /// Tagline (home pages).
    pub(crate) tagline: &'static str,
    /// Hero actions (home pages).
    pub(crate) actions: &'static [EuvHeroAction],
    /// Feature cards (home pages) — each card carries an optional
    /// `link` so the home grid renders as a clickable navigation tile.
    pub(crate) features: &'static [DocsFeature],
    /// Icon+text stat tiles (home pages), rendered between the hero
    /// actions and the feature grid; empty hides the row.
    pub(crate) stats: &'static [DocsStat],
    /// Frontmatter footer override.
    pub(crate) footer: &'static str,
    /// `true` when the page is gated behind a password form. Direct URL
    /// access (paste / refresh) and in-app navigation both check this
    /// flag together with the localStorage unlock record before rendering
    /// the content block AST.
    pub(crate) private: bool,
    /// Hex-encoded SHA-256 of the password that unlocks a `private`
    /// page. Empty when `private` is `false`. The plaintext password is
    /// **never** compiled into the WASM bundle — only this digest is,
    /// hashed at build time from the markdown frontmatter.
    pub(crate) password_hash: &'static str,
}

/// One locale.
#[derive(Clone, Copy, Debug)]
pub(crate) struct DocsLocale {
    /// Route prefix (`/` or `/en/`).
    pub(crate) prefix: &'static str,
    /// Locale title override.
    pub(crate) title: &'static str,
    /// Footer text.
    pub(crate) footer: &'static str,
    /// Right TOC title label.
    pub(crate) toc_label: &'static str,
    /// Prev-page link label.
    pub(crate) prev_label: &'static str,
    /// Next-page link label.
    pub(crate) next_label: &'static str,
    /// Navbar items.
    pub(crate) navbar: &'static [EuvNavbarItem],
    /// Sidebar tree.
    pub(crate) sidebar: &'static [EuvSidebarItem],
}

/// The whole generated site.
#[derive(Clone, Copy, Debug)]
pub(crate) struct DocsSite {
    /// Site title.
    pub(crate) title: &'static str,
    /// All locales.
    pub(crate) locales: &'static [DocsLocale],
    /// All pages.
    pub(crate) pages: &'static [DocsPage],
}

/// One language entry of the cross-bundle switcher.
///
/// Per-locale bundling serves every locale from its own directory (the
/// default locale at the site root), so switching languages is a
/// cross-directory navigation rather than an in-app route change.
#[derive(Clone, Copy, Debug)]
pub(crate) struct DocsLanguageLink {
    /// Human label shown in the dropdown.
    pub(crate) label: &'static str,
    /// Bundle directory relative to the site root (`""` or `"en/"`).
    pub(crate) dir: &'static str,
}

/// One boot-time redirect for a URL space this bundle does not serve.
#[derive(Clone, Copy, Debug)]
pub(crate) struct DocsRedirect {
    /// Foreign route prefix as it appears after the `#` (`"/en"`, `"/zh"`).
    pub(crate) from: &'static str,
    /// Target bundle directory relative to the site root; `""` rewrites the
    /// hash inside this bundle (alias of the pinned locale's directory).
    pub(crate) to_dir: &'static str,
}
