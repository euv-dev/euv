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

/// The section heading for the button card.
pub(crate) const DISPLAY_BUTTON_CARD_TITLE: &str = "Buttons";

/// The label of the primary button in the variant row.
pub(crate) const DISPLAY_BUTTON_DEFAULT_LABEL: &str = "Default";

/// The label of the secondary button in the variant row.
pub(crate) const DISPLAY_BUTTON_SECONDARY_LABEL: &str = "Secondary";

/// The label of the destructive button in the variant row.
pub(crate) const DISPLAY_BUTTON_DESTRUCTIVE_LABEL: &str = "Destructive";

/// The label of the outline button in the variant row.
pub(crate) const DISPLAY_BUTTON_OUTLINE_LABEL: &str = "Outline";

/// The label of the ghost button in the variant row.
pub(crate) const DISPLAY_BUTTON_GHOST_LABEL: &str = "Ghost";

/// The label of the link button in the variant row.
pub(crate) const DISPLAY_BUTTON_LINK_LABEL: &str = "Link";

/// The label of the small button in the size row.
pub(crate) const DISPLAY_BUTTON_SMALL_LABEL: &str = "Small";

/// The label of the default-size button in the size row.
pub(crate) const DISPLAY_BUTTON_SIZE_DEFAULT_LABEL: &str = "Default";

/// The label of the large button in the size row.
pub(crate) const DISPLAY_BUTTON_LARGE_LABEL: &str = "Large";

/// The accessible label of the default square icon button.
pub(crate) const DISPLAY_BUTTON_ICON_LABEL: &str = "Favorite";
/// The decorative glyph shared by the icon-size examples.
pub(crate) const DISPLAY_BUTTON_ICON_GLYPH: &str = "★";

/// The label of the extra-small button in the size row.
pub(crate) const DISPLAY_BUTTON_XS_LABEL: &str = "Extra small";

/// The accessible label of the extra-small icon button in the size row.
pub(crate) const DISPLAY_BUTTON_ICON_XS_LABEL: &str = "Favorite, extra small icon";

/// The accessible label of the small icon button in the size row.
pub(crate) const DISPLAY_BUTTON_ICON_SM_LABEL: &str = "Favorite, small icon";

/// The accessible label of the large icon button in the size row.
pub(crate) const DISPLAY_BUTTON_ICON_LG_LABEL: &str = "Favorite, large icon";

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

/// The section heading for the item card.
pub(crate) const DISPLAY_ITEM_CARD_TITLE: &str = "Items";

/// The title of the first item.
pub(crate) const DISPLAY_ITEM_FIRST_TITLE: &str = "Release notes";

/// The description of the first item.
pub(crate) const DISPLAY_ITEM_FIRST_DESC: &str = "What changed in the last build.";

/// The title of the second item.
pub(crate) const DISPLAY_ITEM_SECOND_TITLE: &str = "Keyboard shortcuts";

/// The description of the second item.
pub(crate) const DISPLAY_ITEM_SECOND_DESC: &str = "The keys that save the most time.";

/// The title of the third item.
pub(crate) const DISPLAY_ITEM_THIRD_TITLE: &str = "Team";

/// The description of the third item.
pub(crate) const DISPLAY_ITEM_THIRD_DESC: &str = "The people who work on the library.";

/// The glyph shown in the icon tile of the first item.
pub(crate) const DISPLAY_ITEM_FIRST_ICON: &str = "R";

/// The glyph shown in the icon tile of the second item.
pub(crate) const DISPLAY_ITEM_SECOND_ICON: &str = "K";

/// The glyph shown in the icon tile of the third item.
pub(crate) const DISPLAY_ITEM_THIRD_ICON: &str = "T";

/// The label of the action of the first item.
pub(crate) const DISPLAY_ITEM_FIRST_ACTION: &str = "Read";

/// The label of the action of the second item.
pub(crate) const DISPLAY_ITEM_SECOND_ACTION: &str = "Open";

/// The section heading for the media card.
pub(crate) const DISPLAY_MEDIA_CARD_TITLE: &str = "Media";

/// The label of the spinner.
pub(crate) const DISPLAY_SPINNER_LABEL: &str = "Loading";

/// The caption shown next to the spinner.
pub(crate) const DISPLAY_SPINNER_CAPTION: &str = "Fetching the latest release";

/// The title shown inside the aspect-ratio box.
pub(crate) const DISPLAY_ASPECT_RATIO_TITLE: &str = "16 : 9";

/// The body shown inside the aspect-ratio box.
pub(crate) const DISPLAY_ASPECT_RATIO_BODY: &str =
    "The box keeps its shape as the column changes width.";

/// The ratio the demo box keeps.
pub(crate) const DISPLAY_ASPECT_RATIO_VALUE: f32 = 1.7777778;

/// Reference example content for title.
pub(crate) const DISPLAY_REFERENCE_TITLE: &str = "Interactive reference controls";

/// Reference example content for account.
pub(crate) const DISPLAY_REFERENCE_ACCOUNT: &str = "account";

/// Reference example content for account label.
pub(crate) const DISPLAY_REFERENCE_ACCOUNT_LABEL: &str = "Account";

/// Reference example content for password.
pub(crate) const DISPLAY_REFERENCE_PASSWORD: &str = "password";

/// Reference example content for password label.
pub(crate) const DISPLAY_REFERENCE_PASSWORD_LABEL: &str = "Password";

/// Reference example content for volume id.
pub(crate) const DISPLAY_REFERENCE_VOLUME_ID: &str = "reference-volume";

/// Reference example content for volume.
pub(crate) const DISPLAY_REFERENCE_VOLUME: &str = "Volume";

/// Reference example content for progress.
pub(crate) const DISPLAY_REFERENCE_PROGRESS: &str = "Volume progress";

/// Reference example content for switch id.
pub(crate) const DISPLAY_REFERENCE_SWITCH_ID: &str = "reference-airplane";

/// Reference example content for switch.
pub(crate) const DISPLAY_REFERENCE_SWITCH: &str = "Airplane mode";

/// Reference example content for disabled id.
pub(crate) const DISPLAY_REFERENCE_DISABLED_ID: &str = "reference-disabled";

/// Reference example content for disabled.
pub(crate) const DISPLAY_REFERENCE_DISABLED: &str = "Disabled switch";

/// Reference example content for tooltip.
pub(crate) const DISPLAY_REFERENCE_TOOLTIP: &str = "Add to library";

/// Reference example content for trigger.
pub(crate) const DISPLAY_REFERENCE_TRIGGER: &str = "Hover or focus";

/// Reference example content for initials.
pub(crate) const DISPLAY_REFERENCE_INITIALS: &str = "CN";

/// Reference example content for avatar.
pub(crate) const DISPLAY_REFERENCE_AVATAR: &str = "Reference avatar";
/// Local illustration used to demonstrate avatar image loading without a network request.
pub(crate) const DISPLAY_REFERENCE_AVATAR_SRC: &str = "data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20viewBox%3D%220%200%2040%2040%22%3E%3Crect%20width%3D%2240%22%20height%3D%2240%22%20fill%3D%22%230ea5e9%22%2F%3E%3Ccircle%20cx%3D%2220%22%20cy%3D%2215%22%20r%3D%227%22%20fill%3D%22%23fff%22%2F%3E%3Cpath%20d%3D%22M7%2040c1-8%206-12%2013-12s12%204%2013%2012%22%20fill%3D%22%23fff%22%2F%3E%3C%2Fsvg%3E";

/// Reference example content for width.
pub(crate) const DISPLAY_REFERENCE_WIDTH: &str = "60%";

/// Reference example content for height.
pub(crate) const DISPLAY_REFERENCE_HEIGHT: &str = "20px";

/// Reference example content for first.
pub(crate) const DISPLAY_REFERENCE_FIRST: &str = "One";

/// Reference example content for second.
pub(crate) const DISPLAY_REFERENCE_SECOND: &str = "Two";

/// The label of the dialog open button.
pub(crate) const DISPLAY_REFERENCE_DIALOG_TRIGGER: &str = "Open dialog";

/// The title of the reference dialog.
pub(crate) const DISPLAY_REFERENCE_DIALOG_TITLE: &str = "Reference dialog";

/// The description of the reference dialog.
pub(crate) const DISPLAY_REFERENCE_DIALOG_DESCRIPTION: &str =
    "Press Escape or use the close control to dismiss this dialog.";

/// The default Badge example label.
pub(crate) const DISPLAY_BADGE_DEFAULT_LABEL: &str = "Default";

/// The secondary Badge example label.
pub(crate) const DISPLAY_BADGE_SECONDARY_LABEL: &str = "Secondary";

/// The destructive Badge example label.
pub(crate) const DISPLAY_BADGE_DESTRUCTIVE_LABEL: &str = "Destructive";

/// The outline Badge example label.
pub(crate) const DISPLAY_BADGE_OUTLINE_LABEL: &str = "Outline";
/// The ghost Badge example label.
pub(crate) const DISPLAY_BADGE_GHOST_LABEL: &str = "Ghost";
/// The link Badge example label.
pub(crate) const DISPLAY_BADGE_LINK_LABEL: &str = "Link";

/// The clickable Badge example label.
pub(crate) const DISPLAY_BADGE_CLICKABLE_LABEL: &str = "Clickable";
