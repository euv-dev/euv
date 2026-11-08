use super::*;

class! {
    // The site-wide footer bar: pinned to the bottom of the viewport,
    // spanning the full width, with the license line on the left and
    // the site's own notices on the right.
    pub c_docs_site_footer {
        position: "fixed";
        left: "0px";
        right: "0px";
        bottom: "0px";
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
/// The reference site pins a 56px bar to the bottom of the viewport and
/// reserves the same height at the end of the scrolling column, so the
/// last line of a page is never hidden behind the bar. The height is
/// published as a custom property, so the bar and the column that
/// reserves the room read one number.
pub(crate) const SITE_FOOTER_CSS: &str = ":root { --docs-site-footer-height: 56px; }";
