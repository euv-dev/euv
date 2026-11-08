use super::*;

/// A demo page for the display half of the component library.
///
/// The page puts the reading and layout components on one screen: the
/// typography roles, links, an image with its placeholder and failure
/// states, an anchor list, a carousel, a watermarked block, a
/// resizable split view and an empty state. The back-to-top button
/// follows the page's scroll offset, so it appears once the reader has
/// travelled far enough down.
///
/// # Arguments
///
/// - `VirtualNode<PageUiDisplayProps>` - The page component node
///   carrying the page props.
///
/// # Returns
///
/// - `VirtualNode` - The display demo page virtual DOM tree.
#[component]
pub(crate) fn page_ui_display(node: VirtualNode<PageUiDisplayProps>) -> VirtualNode {
    let PageUiDisplayProps: PageUiDisplayProps = node.try_get_props().unwrap_or_default();
    let state: DisplayDemoState = use_display_demo_state();
    let scroll: UseScroll = use_scroll();
    let anchor_items: Vec<EuvAnchorItem> = vec![
        EuvAnchorItem {
            value: DISPLAY_ANCHOR_FIRST,
            label: DISPLAY_ANCHOR_FIRST_LABEL,
        },
        EuvAnchorItem {
            value: DISPLAY_ANCHOR_SECOND,
            label: DISPLAY_ANCHOR_SECOND_LABEL,
        },
    ];
    let slides: Vec<EuvCarouselSlide> = vec![
        EuvCarouselSlide {
            title: DISPLAY_SLIDE_FIRST_TITLE,
            desc: DISPLAY_SLIDE_FIRST_DESC,
        },
        EuvCarouselSlide {
            title: DISPLAY_SLIDE_SECOND_TITLE,
            desc: DISPLAY_SLIDE_SECOND_DESC,
        },
        EuvCarouselSlide {
            title: DISPLAY_SLIDE_THIRD_TITLE,
            desc: DISPLAY_SLIDE_THIRD_DESC,
        },
    ];
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "🧱"
                title: DISPLAY_HEADER_TITLE
                subtitle: DISPLAY_HEADER_SUBTITLE
            }
            euv_card {
                title: DISPLAY_TYPOGRAPHY_CARD_TITLE
                div {
                    euv_text {
                        variant: EuvTextVariant::TitleLg
                        text: DISPLAY_TITLE_LG_TEXT
                    }
                }
                div {
                    euv_text {
                        variant: EuvTextVariant::TitleSm
                        text: DISPLAY_TITLE_SM_TEXT
                    }
                }
                div {
                    euv_text {
                        variant: EuvTextVariant::Paragraph
                        text: DISPLAY_PARAGRAPH_TEXT
                    }
                }
                div {
                    euv_text {
                        variant: EuvTextVariant::Secondary
                        text: DISPLAY_SECONDARY_TEXT
                    }
                }
                div {
                    euv_text {
                        variant: EuvTextVariant::Mono
                        text: DISPLAY_MONO_TEXT
                    }
                }
            }
            euv_card {
                title: DISPLAY_BUTTON_CARD_TITLE
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: DISPLAY_BUTTON_PRIMARY_LABEL
                    }
                    euv_button {
                        variant: EuvButtonVariant::Secondary
                        label: DISPLAY_BUTTON_SECONDARY_LABEL
                    }
                    euv_button {
                        variant: EuvButtonVariant::Destructive
                        label: DISPLAY_BUTTON_DESTRUCTIVE_LABEL
                    }
                    euv_button {
                        variant: EuvButtonVariant::Outline
                        label: DISPLAY_BUTTON_OUTLINE_LABEL
                    }
                    euv_button {
                        variant: EuvButtonVariant::Ghost
                        label: DISPLAY_BUTTON_GHOST_LABEL
                    }
                    euv_button {
                        variant: EuvButtonVariant::Link
                        label: DISPLAY_BUTTON_LINK_LABEL
                    }
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        size: EuvButtonSize::Small
                        label: DISPLAY_BUTTON_SMALL_LABEL
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        label: DISPLAY_BUTTON_DEFAULT_LABEL
                    }
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        size: EuvButtonSize::Large
                        label: DISPLAY_BUTTON_LARGE_LABEL
                    }
                    euv_button {
                        variant: EuvButtonVariant::Outline
                        size: EuvButtonSize::Icon
                        label: DISPLAY_BUTTON_ICON_LABEL
                    }
                }
            }
            euv_card {
                title: DISPLAY_LINK_CARD_TITLE
                div {
                    class: c_button_controls()
                    euv_link {
                        text: DISPLAY_LINK_TEXT
                        href: DISPLAY_LINK_HREF
                        on_click: display_on_link_click()
                    }
                    euv_link {
                        text: DISPLAY_DISABLED_LINK_TEXT
                        href: DISPLAY_LINK_HREF
                        disabled: true
                    }
                }
                div {
                    euv_image {
                        src: DISPLAY_IMAGE_SRC
                        alt: DISPLAY_IMAGE_ALT
                        placeholder: DISPLAY_IMAGE_PLACEHOLDER
                    }
                }
            }
            euv_card {
                title: DISPLAY_NAVIGATION_CARD_TITLE
                euv_anchor {
                    items: anchor_items
                    title: DISPLAY_ANCHOR_TITLE
                    active: state.anchor_active
                    on_select: display_on_anchor_select(state.anchor_active)
                }
                euv_carousel {
                    slides: slides
                    active: state.carousel_index
                    on_change: display_on_carousel_change()
                }
            }
            euv_card {
                title: DISPLAY_SPLIT_CARD_TITLE
                p {
                    class: c_badge_hint()
                    DISPLAY_SPLIT_HINT
                }
                euv_splitter {
                    size: state.split_size
                    min: DISPLAY_SPLIT_MIN_SIZE
                    max: DISPLAY_SPLIT_MAX_SIZE
                    div {
                        euv_text {
                            variant: EuvTextVariant::Secondary
                            text: DISPLAY_SPLIT_LEFT_TEXT
                        }
                    }
                    div {
                        euv_text {
                            variant: EuvTextVariant::Secondary
                            text: DISPLAY_SPLIT_RIGHT_TEXT
                        }
                    }
                }
            }
            euv_card {
                title: DISPLAY_OVERLAY_CARD_TITLE
                euv_watermark {
                    text: DISPLAY_WATERMARK_TEXT
                    p {
                        DISPLAY_WATERMARK_CONTENT
                    }
                }
                euv_empty {
                    title: DISPLAY_EMPTY_TITLE
                    desc: DISPLAY_EMPTY_DESC
                    action_label: DISPLAY_EMPTY_ACTION
                    on_action: display_on_empty_action()
                }
            }
            euv_card {
                title: DISPLAY_ITEM_CARD_TITLE
                euv_item_group {
                    euv_item {
                        variant: EuvItemVariant::Default
                        size: EuvItemSize::Default
                        euv_item_media {
                            variant: EuvItemMediaVariant::Icon
                            DISPLAY_ITEM_FIRST_ICON
                        }
                        euv_item_content {
                            euv_item_title {
                                text: DISPLAY_ITEM_FIRST_TITLE
                            }
                            euv_item_description {
                                text: DISPLAY_ITEM_FIRST_DESC
                            }
                        }
                        euv_item_actions {
                            euv_button {
                                variant: EuvButtonVariant::Outline
                                label: DISPLAY_ITEM_FIRST_ACTION
                            }
                        }
                    }
                    euv_item_separator {}
                    euv_item {
                        variant: EuvItemVariant::Outline
                        size: EuvItemSize::Sm
                        euv_item_media {
                            variant: EuvItemMediaVariant::Icon
                            DISPLAY_ITEM_SECOND_ICON
                        }
                        euv_item_content {
                            euv_item_title {
                                text: DISPLAY_ITEM_SECOND_TITLE
                            }
                            euv_item_description {
                                text: DISPLAY_ITEM_SECOND_DESC
                            }
                        }
                        euv_item_actions {
                            euv_button {
                                variant: EuvButtonVariant::Outline
                                label: DISPLAY_ITEM_SECOND_ACTION
                            }
                        }
                    }
                    euv_item_separator {}
                    euv_item {
                        variant: EuvItemVariant::Muted
                        size: EuvItemSize::Sm
                        euv_item_media {
                            variant: EuvItemMediaVariant::Icon
                            DISPLAY_ITEM_THIRD_ICON
                        }
                        euv_item_content {
                            euv_item_title {
                                text: DISPLAY_ITEM_THIRD_TITLE
                            }
                            euv_item_description {
                                text: DISPLAY_ITEM_THIRD_DESC
                            }
                        }
                    }
                }
            }
            euv_card {
                title: DISPLAY_MEDIA_CARD_TITLE
                euv_spinner {
                    label: DISPLAY_SPINNER_LABEL
                    show_label: state.spinner_caption
                }
                p {
                    class: c_badge_hint()
                    DISPLAY_SPINNER_CAPTION
                }
                euv_aspect_ratio {
                    ratio: DISPLAY_ASPECT_RATIO_VALUE
                    div {
                        class: c_badge_hint()
                        DISPLAY_ASPECT_RATIO_TITLE
                    }
                    p {
                        class: c_badge_hint()
                        DISPLAY_ASPECT_RATIO_BODY
                    }
                }
            }
            euv_back_top {
                visible: state.back_top_visible
                label: DISPLAY_BACK_TOP_LABEL
                on_click: display_on_back_to_top(scroll)
            }
        }
    }
}
