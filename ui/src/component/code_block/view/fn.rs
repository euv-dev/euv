use super::*;

/// Holds a snippet of code.
///
/// The block is a column: the header names the snippet, the content
/// holds the code and the footer holds whatever the snippet needs to
/// say below it.
///
/// # Arguments
///
/// - `VirtualNode<EuvCodeBlockProps>` - The props node carrying the
///   variant and the size of the block.
///
/// # Returns
///
/// - `VirtualNode` - The euv code block virtual DOM tree.
#[component]
pub fn euv_code_block(node: VirtualNode<EuvCodeBlockProps>) -> VirtualNode {
    let EuvCodeBlockProps { variant, size }: EuvCodeBlockProps =
        node.try_get_props().unwrap_or_default();
    let variant_attr: &str = match variant {
        EuvCodeBlockVariant::Default => CODE_BLOCK_VARIANT_DEFAULT,
        EuvCodeBlockVariant::Destructive => "destructive",
    };
    let variant_class: fn() -> &'static Css = match variant {
        EuvCodeBlockVariant::Default => c_euv_code_block_default,
        EuvCodeBlockVariant::Destructive => c_euv_code_block_destructive,
    };
    let size_attr: &str = match size {
        EuvCodeBlockSize::Default => CODE_BLOCK_SIZE_DEFAULT,
        EuvCodeBlockSize::Sm => "sm",
    };
    let size_class: fn() -> &'static Css = match size {
        EuvCodeBlockSize::Default => c_euv_code_block_size_default,
        EuvCodeBlockSize::Sm => c_euv_code_block_size_sm,
    };
    let children: VirtualNode = node.get_children().into();
    let base_class: &Css = c_euv_code_block();

    html! {
            div {
                "data-slot": "code-block"
                "data-variant": variant_attr
                "data-size": size_attr
                class: base_class

                class: variant_class()
                class: size_class()
                children
            }
    }
}

/// Holds the header of a code block.
///
/// The header names the snippet and holds the actions that belong to
/// it.
///
/// # Arguments
///
/// - `VirtualNode<EuvCodeBlockHeaderProps>` - The props node carrying
///   the settings of the header.
///
/// # Returns
///
/// - `VirtualNode` - The euv code block header virtual DOM tree.
#[component]
pub fn euv_code_block_header(node: VirtualNode<EuvCodeBlockHeaderProps>) -> VirtualNode {
    let EuvCodeBlockHeaderProps {}: EuvCodeBlockHeaderProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let base_class: &Css = c_euv_code_block_header();

    html! {
            div {
                "data-slot": "code-block-header"
                class: base_class

                children
            }
    }
}

/// Holds the name of the snippet.
///
/// The name is cut short with an ellipsis when it is too long, so the
/// actions beside it stay reachable.
///
/// # Arguments
///
/// - `VirtualNode<EuvCodeBlockTitleProps>` - The props node carrying the
///   settings of the title.
///
/// # Returns
///
/// - `VirtualNode` - The euv code block title virtual DOM tree.
#[component]
pub fn euv_code_block_title(node: VirtualNode<EuvCodeBlockTitleProps>) -> VirtualNode {
    let EuvCodeBlockTitleProps {}: EuvCodeBlockTitleProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    html! {
            div {
                "data-slot": "code-block-title"
                class: c_euv_code_block_title()
                children
            }
    }
}

/// Holds the actions that belong to a code block.
///
/// The actions sit at the end of the header.
///
/// # Arguments
///
/// - `VirtualNode<EuvCodeBlockActionsProps>` - The props node carrying
///   the settings of the actions.
///
/// # Returns
///
/// - `VirtualNode` - The euv code block actions virtual DOM tree.
#[component]
pub fn euv_code_block_actions(node: VirtualNode<EuvCodeBlockActionsProps>) -> VirtualNode {
    let EuvCodeBlockActionsProps {}: EuvCodeBlockActionsProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let base_class: &Css = c_euv_code_block_actions();

    html! {
            div {
                "data-slot": "code-block-actions"
                class: base_class

                children
            }
    }
}

/// Puts the snippet on the clipboard.
///
/// The button writes the text it carries to the clipboard and says so
/// on its own element: the confirmation lives in the `data-copied`
/// attribute of the button, which is set on the element itself and
/// taken away again after two seconds. The confirmation never travels
/// through a signal, because a signal the page reads re-renders the
/// page, and the re-render rewrites the button back to its resting
/// state before the reader can see the confirmation.
///
/// # Arguments
///
/// - `VirtualNode<EuvCodeBlockCopyButtonProps>` - The props node carrying
///   the text the button copies.
///
/// # Returns
///
/// - `VirtualNode` - The euv code block copy button virtual DOM tree.
#[component]
pub fn euv_code_block_copy_button(node: VirtualNode<EuvCodeBlockCopyButtonProps>) -> VirtualNode {
    let EuvCodeBlockCopyButtonProps { value }: EuvCodeBlockCopyButtonProps =
        node.try_get_props().unwrap_or_default();
    let clipboard: UseClipboard = use_clipboard();
    let copy: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |event: Event| {
        let started: bool = clipboard.copy(value);
        let Some(target) = event.target() else {
            return;
        };
        let Ok(button) = target.dyn_into::<Element>() else {
            return;
        };
        set_copy_state(&button, started);
        let button: Element = button.clone();
        schedule_after(COPY_FEEDBACK_MILLIS, move || {
            set_copy_state(&button, false);
        });
    }));
    let children: VirtualNode = node.get_children().into();
    html! {
            button {
                "data-slot": "code-block-copy-button"
                "aria-label": "Copy"
                type: "button"
                class: c_euv_code_block_copy_button()
                onclick: copy
                children
            }
    }
}

/// Says on the button whether the text was copied.
///
/// The state lives in an attribute of the button, so it survives a
/// re-render of the page and does not travel through a signal.
///
/// # Arguments
///
/// - `&Element` - The button the state is written on.
/// - `bool` - Whether the text was copied.
///
/// # Returns
///
/// - `()` - The state is written on the element.
fn set_copy_state(button: &Element, copied: bool) {
    let attribute: &str = if copied { "true" } else { "false" };
    let _: Result<(), JsValue> = button.set_attribute("data-copied", attribute);
}

/// Holds the code of a snippet.
///
/// The code is read from the left even in a right to left page, because
/// code is not prose. The block scrolls sideways when the lines are
/// long, unless the caller asks for them to wrap.
///
/// # Arguments
///
/// - `VirtualNode<EuvCodeBlockContentProps>` - The props node carrying
///   the wrap and the numbering settings.
///
/// # Returns
///
/// - `VirtualNode` - The euv code block content virtual DOM tree.
#[component]
pub fn euv_code_block_content(node: VirtualNode<EuvCodeBlockContentProps>) -> VirtualNode {
    let EuvCodeBlockContentProps {
        wrap,
        line_numbers,
        start_line,
    }: EuvCodeBlockContentProps = node.try_get_props().unwrap_or_default();
    // The counter of the line numbers starts one below the first line,
    // so the first line is numbered the way the caller asked for.
    let counter_style: String = format!("counter-reset: line {};", start_line - 1);
    let counter_ref: &str = counter_style.as_str();
    let children: VirtualNode = node.get_children().into();
    html! {
        pre {
            "data-slot": "code-block-content"
            "data-wrap": if wrap {
                "true"
            } else {
                "false"
            }
            "data-numbered": if line_numbers {
                "true"
            } else {
                "false"
            }
            "dir": "ltr"
            tabindex: "0"
            style: counter_ref
            class: c_euv_code_block_content()
            class: if wrap {
                c_euv_code_block_content_wrap()
            } else {
                c_euv_code_block_content_no_wrap()
            }
            code {
                "data-slot": "code-block-code"
                class: c_euv_code_block_code()
                children
            }
        }
    }
}

/// Holds one line of a snippet.
///
/// The line carries the state of the line inside the change the snippet
/// shows.
///
/// # Arguments
///
/// - `VirtualNode<EuvCodeBlockLineProps>` - The props node carrying the
///   variant of the line.
///
/// # Returns
///
/// - `VirtualNode` - The euv code block line virtual DOM tree.
#[component]
pub fn euv_code_block_line(node: VirtualNode<EuvCodeBlockLineProps>) -> VirtualNode {
    let EuvCodeBlockLineProps { variant }: EuvCodeBlockLineProps =
        node.try_get_props().unwrap_or_default();
    let variant_attr: &str = match variant {
        EuvCodeBlockLineVariant::Default => CODE_BLOCK_LINE_VARIANT_DEFAULT,
        EuvCodeBlockLineVariant::Added => "added",
        EuvCodeBlockLineVariant::Removed => "removed",
        EuvCodeBlockLineVariant::Highlighted => "highlighted",
    };
    let variant_class: fn() -> &'static Css = match variant {
        EuvCodeBlockLineVariant::Default => c_euv_code_block_line_default,
        EuvCodeBlockLineVariant::Added => c_euv_code_block_line_added,
        EuvCodeBlockLineVariant::Removed => c_euv_code_block_line_removed,
        EuvCodeBlockLineVariant::Highlighted => c_euv_code_block_line_highlighted,
    };
    let children: VirtualNode = node.get_children().into();
    html! {
        span {
            "data-slot": "code-block-line"
            "data-variant": variant_attr
            class: c_euv_code_block_line()
            class: variant_class()
            span {
                "data-slot": "code-block-line-content"
                class: c_euv_code_block_line_content()
                children
            }
        }
    }
}

/// Holds the footer of a code block.
///
/// # Arguments
///
/// - `VirtualNode<EuvCodeBlockFooterProps>` - The props node carrying the
///   settings of the footer.
///
/// # Returns
///
/// - `VirtualNode` - The euv code block footer virtual DOM tree.
#[component]
pub fn euv_code_block_footer(node: VirtualNode<EuvCodeBlockFooterProps>) -> VirtualNode {
    let EuvCodeBlockFooterProps {}: EuvCodeBlockFooterProps =
        node.try_get_props().unwrap_or_default();
    let children: VirtualNode = node.get_children().into();
    let base_class: &Css = c_euv_code_block_footer();

    html! {
        div {
            "data-slot": "code-block-footer"
            class: base_class

            children
        }
    }
}
