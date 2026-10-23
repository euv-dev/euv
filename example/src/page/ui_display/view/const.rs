/// The title text for the header on the display demo page.
pub(crate) const DISPLAY_HEADER_TITLE: &str = "Display Components";

/// The subtitle text for the header on the display demo page.
pub(crate) const DISPLAY_HEADER_SUBTITLE: &str = "Typography, links, images, anchors, carousels, watermarks, split views and empty states — the reading and layout half of the component library.";

/// The section heading for the typography card.
pub(crate) const DISPLAY_TYPOGRAPHY_CARD_TITLE: &str = "Typography";

/// The body text rendered in the paragraph role.
pub(crate) const DISPLAY_PARAGRAPH_TEXT: &str = "Running text is set in the body role, which is the default. Weight and size carry the hierarchy, never colour.";

/// The headline rendered in the large title role.
pub(crate) const DISPLAY_TITLE_LG_TEXT: &str = "Section Headline";

/// The headline rendered in the small title role.
pub(crate) const DISPLAY_TITLE_SM_TEXT: &str = "Sub-headline";

/// The supporting text rendered in the secondary role.
pub(crate) const DISPLAY_SECONDARY_TEXT: &str = "Supporting text in the muted style.";

/// The identifier rendered in the monospace role.
pub(crate) const DISPLAY_MONO_TEXT: &str = "euv_ui::euv_button";

/// The section heading for the link and image card.
pub(crate) const DISPLAY_LINK_CARD_TITLE: &str = "Links & Images";

/// The text of the navigable link.
pub(crate) const DISPLAY_LINK_TEXT: &str = "The component library";

/// The destination the navigable link points at.
pub(crate) const DISPLAY_LINK_HREF: &str = "https://example.com";

/// The text of the disabled link.
pub(crate) const DISPLAY_DISABLED_LINK_TEXT: &str = "Disabled link";

/// The alternative text of the demo image.
pub(crate) const DISPLAY_IMAGE_ALT: &str = "A placeholder image";

/// The source of the demo image.
pub(crate) const DISPLAY_IMAGE_SRC: &str = "https://example.invalid/placeholder.png";

/// The text shown while the demo image loads.
pub(crate) const DISPLAY_IMAGE_PLACEHOLDER: &str = "Loading the image…";

/// The section heading for the anchor and carousel card.
pub(crate) const DISPLAY_NAVIGATION_CARD_TITLE: &str = "Navigation";

/// The heading of the anchor list.
pub(crate) const DISPLAY_ANCHOR_TITLE: &str = "On this page";

/// The value of the first anchor entry.
pub(crate) const DISPLAY_ANCHOR_FIRST: &str = "intro";

/// The label of the first anchor entry.
pub(crate) const DISPLAY_ANCHOR_FIRST_LABEL: &str = "Introduction";

/// The value of the second anchor entry.
pub(crate) const DISPLAY_ANCHOR_SECOND: &str = "details";

/// The label of the second anchor entry.
pub(crate) const DISPLAY_ANCHOR_SECOND_LABEL: &str = "Details";

/// The heading of the first carousel slide.
pub(crate) const DISPLAY_SLIDE_FIRST_TITLE: &str = "First slide";

/// The description of the first carousel slide.
pub(crate) const DISPLAY_SLIDE_FIRST_DESC: &str =
    "Every slide stays mounted, so the transition is one animated property.";

/// The heading of the second carousel slide.
pub(crate) const DISPLAY_SLIDE_SECOND_TITLE: &str = "Second slide";

/// The description of the second carousel slide.
pub(crate) const DISPLAY_SLIDE_SECOND_DESC: &str =
    "The dots below move the track by a whole slide.";

/// The heading of the third carousel slide.
pub(crate) const DISPLAY_SLIDE_THIRD_TITLE: &str = "Third slide";

/// The description of the third carousel slide.
pub(crate) const DISPLAY_SLIDE_THIRD_DESC: &str = "The active dot carries the accent fill.";

/// The section heading for the split view card.
pub(crate) const DISPLAY_SPLIT_CARD_TITLE: &str = "Split View";

/// The hint text explaining the split view.
pub(crate) const DISPLAY_SPLIT_HINT: &str = "Drag the divider between the two panes. The first pane's size is a signal, so the caller owns it.";

/// The text of the left pane.
pub(crate) const DISPLAY_SPLIT_LEFT_TEXT: &str = "Left pane";

/// The text of the right pane.
pub(crate) const DISPLAY_SPLIT_RIGHT_TEXT: &str = "Right pane";

/// The default share of the split view the first pane takes, in percent.
pub(crate) const DISPLAY_SPLIT_DEFAULT_SIZE: f64 = 40.0;

/// The smallest share the first pane of the split view may take.
pub(crate) const DISPLAY_SPLIT_MIN_SIZE: f64 = 15.0;

/// The largest share the first pane of the split view may take.
pub(crate) const DISPLAY_SPLIT_MAX_SIZE: f64 = 80.0;

/// The section heading for the watermark and empty state card.
pub(crate) const DISPLAY_OVERLAY_CARD_TITLE: &str = "Watermark & Empty State";

/// The text stamped over the watermarked content.
pub(crate) const DISPLAY_WATERMARK_TEXT: &str = "DRAFT";

/// The content the watermark is stamped over.
pub(crate) const DISPLAY_WATERMARK_CONTENT: &str = "The watermark sits over this paragraph without taking any pointer events, so the text stays selectable.";

/// The headline of the empty state.
pub(crate) const DISPLAY_EMPTY_TITLE: &str = "Nothing here yet";

/// The explanation under the empty state's headline.
pub(crate) const DISPLAY_EMPTY_DESC: &str =
    "An empty state says what is missing and offers a way out of it.";

/// The label of the empty state's action.
pub(crate) const DISPLAY_EMPTY_ACTION: &str = "Create the first one";

/// The label of the back-to-top button.
pub(crate) const DISPLAY_BACK_TOP_LABEL: &str = "↑";

/// The scroll offset, in pixels, past which the back-to-top button appears.
pub(crate) const DISPLAY_BACK_TOP_THRESHOLD: f64 = 200.0;

// The lines the demo writes into the console panel when a callback runs.
pub(crate) const DISPLAY_LINK_LOG: &str = "link opened";
pub(crate) const DISPLAY_CAROUSEL_LOG: &str = "slide {index} is on screen";
pub(crate) const DISPLAY_EMPTY_LOG: &str = "empty state action asked for";
