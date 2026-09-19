use super::*;

/// One feature card on the home page. Adds a `link` over
/// `euv_ui::EuvFeature`; lives here because `EuvFeature` is frozen on
/// the `euv = "0.18"` pin and cannot be augmented upstream.
#[derive(Clone, Copy, Debug, Default)]
pub struct DocsFeature {
    /// Card icon (emoji); the literal string `"blog"` is treated as no
    /// icon by `docs_feature_card`.
    pub icon: &'static str,
    pub title: &'static str,
    pub details: &'static str,
    /// Route or external URL; empty means non-clickable.
    pub link: &'static str,
}

/// One rendered markdown page.
#[derive(Clone, Copy, Debug)]
pub struct DocsPage {
    /// Full route (`/guide/getting-started.html`, `/zh/` …).
    pub route: &'static str,
    /// Page title.
    pub title: &'static str,
    /// Content block AST (rendered by `euv_markdown`).
    pub blocks: &'static [EuvMdBlock],
    /// Anchor TOC entries.
    pub headings: &'static [EuvTocItem],
    /// Whether this is a home page.
    pub home: bool,
    /// Hero text (home pages).
    pub hero_text: &'static str,
    /// Tagline (home pages).
    pub tagline: &'static str,
    /// Hero actions (home pages).
    pub actions: &'static [EuvHeroAction],
    /// Feature cards (home pages) — each card carries an optional
    /// `link` so the home grid renders as a clickable navigation tile.
    pub features: &'static [DocsFeature],
    /// Frontmatter footer override.
    pub footer: &'static str,
    /// `true` when the page is gated behind a password form. Direct URL
    /// access (paste / refresh) and in-app navigation both check this
    /// flag together with the localStorage unlock record before rendering
    /// the content block AST.
    pub private: bool,
    /// Hex-encoded SHA-256 of the password that unlocks a `private`
    /// page. Empty when `private` is `false`. The plaintext password is
    /// **never** compiled into the WASM bundle — only this digest is,
    /// hashed at build time from the markdown frontmatter.
    pub password_hash: &'static str,
}

/// One locale.
#[derive(Clone, Copy, Debug)]
pub struct DocsLocale {
    /// Route prefix (`/` or `/zh/`).
    pub prefix: &'static str,
    /// Human label for the language dropdown.
    pub label: &'static str,
    /// Locale title override.
    pub title: &'static str,
    /// Footer text.
    pub footer: &'static str,
    /// Right TOC title label.
    pub toc_label: &'static str,
    /// Prev-page link label.
    pub prev_label: &'static str,
    /// Next-page link label.
    pub next_label: &'static str,
    /// Navbar items.
    pub navbar: &'static [EuvNavbarItem],
    /// Sidebar tree.
    pub sidebar: &'static [EuvSidebarItem],
}

/// The whole generated site.
#[derive(Clone, Copy, Debug)]
pub struct DocsSite {
    /// Site title.
    pub title: &'static str,
    /// All locales.
    pub locales: &'static [DocsLocale],
    /// All pages.
    pub pages: &'static [DocsPage],
}
