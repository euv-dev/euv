use super::*;

/// A generic status/result page aligned with common component libraries
/// (Ant Design `Result`, Element Plus `Result`): a big status code, a title,
/// a description and an action area.
///
/// Typical uses are 404 / 403 / 500 pages; the action buttons (or links) are
/// passed in as children.
///
/// # Arguments
///
/// - `VirtualNode<EuvResultProps>` - The props node.
///
/// # Returns
///
/// - `VirtualNode` - The result page virtual DOM tree.
#[component]
pub fn euv_result(node: VirtualNode<EuvResultProps>) -> VirtualNode {
    let EuvResultProps {
        code,
        title,
        description,
        level,
    }: EuvResultProps = node.try_get_props().unwrap_or_default();
    let level: EuvHeadingLevel = level.unwrap_or(EuvHeadingLevel::H1);
    // The emptiness of each text is read from a copy, so the text itself
    // stays whole for the tree to carry.
    // The tree is written inside a closure that may run again, and the
    // reactive branch inside it owns a copy of its own.
    let code: ::euv::CowText = code.clone();
    let title: ::euv::CowText = title.clone();
    let description: ::euv::CowText = description.clone();
    let code_is_empty: bool = code.is_empty();
    let title_is_empty: bool = title.is_empty();
    let description_is_empty: bool = description.is_empty();
    let has_code: Signal<bool> = Signal::create(!code_is_empty);
    let has_title: Signal<bool> = Signal::create(!title_is_empty);
    let has_description: Signal<bool> = Signal::create(!description_is_empty);
    let children: VirtualNode = node.get_children().into();
    html! {
        div {
            class: c_euv_result()
            if { has_code.get() } {
                result_code_block(code.clone())
            }
            if { has_title.get() } {
                result_title_heading(level, title.clone())
            }
            if { has_description.get() } {
                result_description_block(description.clone())
            }
            div {
                class: c_euv_result_actions()
                children
            }
        }
    }
}

/// Builds the heading element a result title is written as.
///
/// The level is a setting of the component, so the element is chosen by a
/// match over the levels rather than by a fixed tag: the result is the whole
/// page on one route and a panel inside another, so the library cannot know
/// the outline of the page it sits in.
///
/// # Arguments
///
/// - `EuvHeadingLevel` - The level the title is written at.
/// - `::euv::CowText` - The title.
///
/// # Returns
///
/// - `VirtualNode` - The heading element.
fn result_title_heading(level: EuvHeadingLevel, title: ::euv::CowText) -> VirtualNode {
    match level {
        EuvHeadingLevel::H1 => html! {
        h1 {
            class:c_euv_result_title() {
                title.clone()
            }
        }
        },
        EuvHeadingLevel::H2 => html! {
        h2 {
            class:c_euv_result_title() {
                title.clone()
            }
        }
        },
        EuvHeadingLevel::H3 => html! {
        h3 {
            class:c_euv_result_title() {
                title.clone()
            }
        }
        },
        EuvHeadingLevel::H4 => html! {
        h4 {
            class:c_euv_result_title() {
                title.clone()
            }
        }
        },
        EuvHeadingLevel::H5 => html! {
        h5 {
            class:c_euv_result_title() {
                title.clone()
            }
        }
        },
        EuvHeadingLevel::H6 => html! {
        h6 {
            class:c_euv_result_title() {
                title.clone()
            }
        }
        },
    }
}

/// Builds the block that holds the status code of a result.
///
/// # Arguments
///
/// - `::euv::CowText` - The status code.
///
/// # Returns
///
/// - `VirtualNode` - The block.
fn result_code_block(code: ::euv::CowText) -> VirtualNode {
    html! {
        div {
            class: c_euv_result_code()
            {
                code.clone()
            }
        }
    }
}

/// Builds the paragraph that explains a result.
///
/// # Arguments
///
/// - `::euv::CowText` - The description.
///
/// # Returns
///
/// - `VirtualNode` - The paragraph.
fn result_description_block(description: ::euv::CowText) -> VirtualNode {
    html! {
        p {
            class: c_euv_result_description()
            {
                description.clone()
            }
        }
    }
}
