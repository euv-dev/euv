use super::*;

/// A run of text in one of the typographic roles.
///
/// Renders a single element whose class comes from the `variant`: the
/// body, paragraph, secondary, monospace, deletion and underline roles
/// each have their own class, and the three headline roles share the
/// title class with a size modifier on top.
///
/// # Arguments
///
/// - `VirtualNode<EuvTextProps>` - The props node containing the text
///   and its role.
///
/// # Returns
///
/// - `VirtualNode` - The text virtual DOM tree.
#[component]
pub fn euv_text(node: VirtualNode<EuvTextProps>) -> VirtualNode {
    let EuvTextProps { text, variant }: EuvTextProps = node.try_get_props().unwrap_or_default();
    html! {
        span {
            class: text_class(variant)()
            text
        }
    }
}

/// Returns the registry class for a typographic role.
///
/// The headline roles need two classes — the shared title class and a
/// size modifier — so they are handled apart from the single-class
/// roles.
///
/// # Arguments
///
/// - `EuvTextVariant` - The role the text plays.
///
/// # Returns
///
/// - `fn() -> &'static Css` - The class function for that role.
fn text_class(variant: EuvTextVariant) -> fn() -> &'static Css {
    match variant {
        EuvTextVariant::Body => c_euv_text,
        EuvTextVariant::Title => c_euv_text_title,
        EuvTextVariant::TitleLg => c_euv_text_title_lg,
        EuvTextVariant::TitleSm => c_euv_text_title_sm,
        EuvTextVariant::Paragraph => c_euv_text_paragraph,
        EuvTextVariant::Secondary => c_euv_text_secondary,
        EuvTextVariant::Mono => c_euv_text_mono,
        EuvTextVariant::Deletion => c_euv_text_deletion,
        EuvTextVariant::Underline => c_euv_text_underline,
    }
}
