use super::*;

/// A list rendering demo page with dynamic item management.
///
/// # Returns
///
/// - `VirtualNode` - The list demo page virtual DOM tree.
#[component]
pub(crate) fn page_list(node: VirtualNode<PageListProps>) -> VirtualNode {
    let PageListProps: PageListProps = node.try_get_props().unwrap_or_default();
    let state: UseTodoList = use_todo_list();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "📝"
                title: "List Rendering"
                subtitle: "Dynamic todo list with Signal-backed add and remove operations. Demonstrates keyed list rendering and reactive state management."
            }
            euv_card {
                title: "Todo List"
                div {
                    class: c_inline_input_row()
                    input {
                        id: LIST_NEW_ITEM_ID
                        name: LIST_NEW_ITEM_NAME
                        type: LIST_TEXT_TYPE
                        autocomplete: LIST_AUTOCOMPLETE_OFF
                        placeholder: LIST_NEW_ITEM_PLACEHOLDER
                        value: state.get_new_item()
                        class: if { state.get_add_error().get().is_empty() } {
                            c_list_input()
                        } else {
                            c_list_input_error()
                        }
                        oninput: todo_list_on_input_new_item(state)
                    }
                    div {
                        class: c_inline_input_button_wrap()
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            label: "Add"
                            onclick: todo_list_on_add(state)
                        }
                    }
                }
                if { !state.get_add_error().get().is_empty() } {
                    p {
                        class: c_list_error_text()
                        state.get_add_error()
                    }
                }
                ul {
                    class: c_list_ul()
                    data_list_container: "true"
                    for (index, item) in state.get_items().get().iter().enumerate() {
                        li {
                            key: index.to_string()
                            class: c_list_item()
                            data_index: index.to_string()
                            span {
                                class: c_list_item_text()
                                item.clone()
                            }
                            div {
                                class: c_list_item_button()
                                euv_button {
                                    variant: EuvButtonVariant::Primary
                                    label: "Remove"
                                    onclick: todo_list_on_remove(state.get_items(), index)
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
