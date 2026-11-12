use super::*;

class! {
    // The site-wide footer bar: fixed above the viewport's bottom gutter,
    // spanning the full width, with the license line on the left and
    // the site's own notices on the right.
    pub c_docs_site_footer {
        position: "fixed";
        left: "0px";
        right: "0px";
        bottom: var!(edge-gutter-bottom);
        z-index: "90";
        display: "flex";
        align-items: "center";
        justify-content: "space-between";
        gap: "16px";
        height: var!(docs-site-footer-height);
        padding: "0px 24px";
        border-top: format!("1px solid {}", var!(border));
        background: var!(background);
        color: var!(muted-foreground);
        font-size: var!(font-xs);
        line-height: "1.5";
    }

    // One link of the footer bar.
    pub c_docs_site_footer_link {
        display: "inline-flex";
        align-items: "center";
        min-width: "0px";
        overflow: "hidden";
        text-overflow: "ellipsis";
        white-space: "nowrap";
        color: "inherit";
        text-decoration: "none";
        :hover {
            color: var!(foreground);
        }
    }

    // The icon that precedes the text of a footer item.
    pub c_docs_site_footer_icon {
        display: "inline-block";
        height: var!(space-xl);
        width: "auto";
        vertical-align: "middle";
        margin-right: var!(space-2xs);
    }
}

/// The stylesheet that publishes the footer bar's height.
///
/// The reference site keeps the bar inset from the viewport edge by the
/// bottom gutter. The scrolling column reserves room for the bar and a
/// separate content gutter, so its last line clears the bar as well.
/// The bar height is published as a custom property so both regions share
/// one measurement.
pub(crate) const SITE_FOOTER_CSS: &str = ":root { --docs-site-footer-height: 56px; }";
