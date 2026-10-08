use super::*;

/// A badge demo page showcasing status indicators with click support.
///
/// # Arguments
///
/// - `VirtualNode<PageBadgeProps>` - The page component node carrying the
///   page props.
///
/// # Returns
///
/// - `VirtualNode` - The badge demo page virtual DOM tree.
#[component]
pub(crate) fn page_badge(node: VirtualNode<PageBadgeProps>) -> VirtualNode {
    let PageBadgeProps: PageBadgeProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "🏷️"
                title: BADGE_HEADER_TITLE
                subtitle: BADGE_HEADER_SUBTITLE
            }
            euv_card {
                title: BADGE_SOLID_CARD_TITLE
                p {
                    class: c_badge_hint()
                    BADGE_SOLID_HINT_TEXT
                }
                div {
                    class: c_badge_row()
                    euv_tag {
                        color: EuvTagColor::Black
                        variant: EuvTagVariant::Solid
                        text: BADGE_BLACK_TAG_TEXT
                        on_click: badge_on_click(BADGE_BLACK_TAG_TEXT, LogLevel::Log)
                    }
                }
            }
            euv_card {
                title: BADGE_OUTLINE_CARD_TITLE
                div {
                    class: c_badge_row()
                    euv_tag {
                        color: EuvTagColor::White
                        variant: EuvTagVariant::Outline
                        text: BADGE_WHITE_TAG_TEXT
                        on_click: badge_on_click(BADGE_OUTLINE_WHITE_LOG_NAME, LogLevel::Log)
                    }
                }
            }
        }
    }
}
