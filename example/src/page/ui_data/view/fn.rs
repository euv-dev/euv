use super::*;

/// A demo page for the data-reading and feedback half of the library.
///
/// The page puts the remaining components on one screen: a list, a
/// description grid, a tree, the toast and notification stacks, and a
/// confirmation bubble. The toast and the notification are driven by
/// buttons, so a reader can push one and dismiss it again.
///
/// # Arguments
///
/// - `VirtualNode<PageUiDataProps>` - The page component node carrying
///   the page props.
///
/// # Returns
///
/// - `VirtualNode` - The data demo page virtual DOM tree.
#[component]
pub(crate) fn page_ui_data(node: VirtualNode<PageUiDataProps>) -> VirtualNode {
    let PageUiDataProps: PageUiDataProps = node.try_get_props().unwrap_or_default();
    let state: DataDemoState = use_data_demo_state();
    let list_items: Vec<EuvListItem> = vec![
        EuvListItem {
            title: DATA_LIST_FIRST_TITLE,
            desc: DATA_LIST_FIRST_DESC,
            meta: DATA_LIST_FIRST_META,
        },
        EuvListItem {
            title: DATA_LIST_SECOND_TITLE,
            desc: DATA_LIST_SECOND_DESC,
            meta: DATA_LIST_SECOND_META,
        },
        EuvListItem {
            title: DATA_LIST_THIRD_TITLE,
            desc: DATA_LIST_THIRD_DESC,
            meta: DATA_LIST_THIRD_META,
        },
    ];
    let description_items: Vec<EuvDescriptionsItem> = vec![
        EuvDescriptionsItem {
            label: DATA_DESCRIPTION_FIRST_LABEL,
            value: DATA_DESCRIPTION_FIRST_VALUE,
        },
        EuvDescriptionsItem {
            label: DATA_DESCRIPTION_SECOND_LABEL,
            value: DATA_DESCRIPTION_SECOND_VALUE,
        },
        EuvDescriptionsItem {
            label: DATA_DESCRIPTION_THIRD_LABEL,
            value: DATA_DESCRIPTION_THIRD_VALUE,
        },
        EuvDescriptionsItem {
            label: DATA_DESCRIPTION_FOURTH_LABEL,
            value: DATA_DESCRIPTION_FOURTH_VALUE,
        },
    ];
    let table_columns: Vec<EuvTableColumn> = vec![
        EuvTableColumn {
            key: DATA_TABLE_FIRST_ROW_KEY,
            label: DATA_TABLE_NAME_COLUMN,
            align: EuvTableAlign::Left,
        },
        EuvTableColumn {
            key: DATA_TABLE_SECOND_ROW_KEY,
            label: DATA_TABLE_VERSION_COLUMN,
            align: EuvTableAlign::Right,
        },
        EuvTableColumn {
            key: DATA_TABLE_THIRD_ROW_KEY,
            label: DATA_TABLE_TARGET_COLUMN,
            align: EuvTableAlign::Center,
        },
        EuvTableColumn {
            key: DATA_TABLE_STATE_COLUMN,
            label: DATA_TABLE_STATE_COLUMN,
            align: EuvTableAlign::Right,
        },
    ];
    let table_rows: Vec<EuvTableRow> = vec![
        EuvTableRow {
            key: DATA_TABLE_FIRST_ROW_KEY,
            cells: vec![
                DATA_TABLE_FIRST_NAME,
                DATA_TABLE_FIRST_VERSION,
                DATA_TABLE_FIRST_TARGET,
                DATA_TABLE_FIRST_STATE,
            ],
        },
        EuvTableRow {
            key: DATA_TABLE_SECOND_ROW_KEY,
            cells: vec![
                DATA_TABLE_SECOND_NAME,
                DATA_TABLE_SECOND_VERSION,
                DATA_TABLE_SECOND_TARGET,
                DATA_TABLE_SECOND_STATE,
            ],
        },
        EuvTableRow {
            key: DATA_TABLE_THIRD_ROW_KEY,
            cells: vec![
                DATA_TABLE_THIRD_NAME,
                DATA_TABLE_THIRD_VERSION,
                DATA_TABLE_THIRD_TARGET,
                DATA_TABLE_THIRD_STATE,
            ],
        },
    ];
    let message_items: Vec<EuvMessageItem> = (0..state.message_count.get())
        .map(|_| EuvMessageItem {
            text: DATA_MESSAGE_TEXT,
            action_label: DATA_MESSAGE_ACTION,
            on_action: data_on_message_action(),
            on_close: data_on_message_close(state.message_count),
            dismissible: true,
        })
        .collect();
    let notification_items: Vec<EuvNotificationItem> = (0..state.notification_count.get())
        .map(|_| EuvNotificationItem {
            title: DATA_NOTIFICATION_TITLE,
            text: DATA_NOTIFICATION_TEXT,
            on_close: data_on_notification_close(state.notification_count),
            dismissible: true,
        })
        .collect();
    let tree_nodes: Vec<EuvTreeNode> = vec![
        EuvTreeNode {
            label: DATA_TREE_ROOT_LABEL,
            value: DATA_TREE_ROOT_VALUE,
            children: vec![
                EuvTreeNode {
                    label: DATA_TREE_LEAF_FIRST_LABEL,
                    value: DATA_TREE_LEAF_FIRST_VALUE,
                    children: Vec::new(),
                },
                EuvTreeNode {
                    label: DATA_TREE_LEAF_SECOND_LABEL,
                    value: DATA_TREE_LEAF_SECOND_VALUE,
                    children: Vec::new(),
                },
            ],
        },
        EuvTreeNode {
            label: DATA_TREE_ROOT_SECOND_LABEL,
            value: DATA_TREE_ROOT_SECOND_VALUE,
            children: vec![EuvTreeNode {
                label: DATA_TREE_LEAF_THIRD_LABEL,
                value: DATA_TREE_LEAF_THIRD_VALUE,
                children: Vec::new(),
            }],
        },
    ];
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "🗂️"
                title: DATA_HEADER_TITLE
                subtitle: DATA_HEADER_SUBTITLE
            }
            euv_card {
                title: DATA_LIST_CARD_TITLE
                p {
                    class: c_badge_hint()
                    DATA_LIST_HINT
                }
                euv_list {
                    items: list_items
                    empty_text: ""
                }
            }
            euv_card {
                title: DATA_DESCRIPTIONS_CARD_TITLE
                p {
                    class: c_badge_hint()
                    DATA_DESCRIPTIONS_HINT
                }
                euv_descriptions {
                    items: description_items
                    columns: 2
                }
            }
            euv_card {
                title: DATA_TABLE_CARD_TITLE
                p {
                    class: c_badge_hint()
                    DATA_TABLE_HINT
                }
                euv_table {
                    columns: table_columns
                    rows: table_rows
                    caption: DATA_TABLE_CAPTION
                    zebra: true
                }
            }
            euv_card {
                title: DATA_PAGINATION_CARD_TITLE
                p {
                    class: c_badge_hint()
                    DATA_PAGINATION_HINT
                }
                euv_pagination {
                    euv_pagination_content {
                        euv_pagination_item {
                            label: DATA_PAGINATION_PREV_LABEL
                            text: DATA_PAGINATION_PREV_TEXT
                            link: DATA_PAGINATION_PREV_LINK
                            next: false
                        }
                        euv_pagination_link {
                            label: DATA_PAGINATION_PAGE_ONE
                            link: DATA_PAGINATION_PAGE_ONE_LINK
                            active: true
                        }
                        euv_pagination_link {
                            label: DATA_PAGINATION_PAGE_TWO
                            link: DATA_PAGINATION_PAGE_TWO_LINK
                        }
                        euv_pagination_link {
                            label: DATA_PAGINATION_PAGE_THREE
                            link: DATA_PAGINATION_PAGE_THREE_LINK
                        }
                        euv_pagination_ellipsis {}
                        euv_pagination_link {
                            label: DATA_PAGINATION_PAGE_LAST
                            link: DATA_PAGINATION_PAGE_LAST_LINK
                        }
                        euv_pagination_item {
                            label: DATA_PAGINATION_NEXT_LABEL
                            text: DATA_PAGINATION_NEXT_TEXT
                            link: DATA_PAGINATION_NEXT_LINK
                            next: true
                        }
                    }
                }
            }
            euv_card {
                title: DATA_TREE_CARD_TITLE
                p {
                    class: c_badge_hint()
                    DATA_TREE_HINT
                }
                euv_tree {
                    nodes: tree_nodes
                    selected: state.selected
                    on_select: data_on_tree_select(state.selected)
                    default_expanded: true
                }
            }
            euv_card {
                title: DATA_FEEDBACK_CARD_TITLE
                p {
                    class: c_badge_hint()
                    DATA_FEEDBACK_HINT
                }
                div {
                    class: c_button_controls()
                    euv_button {
                        variant: EuvButtonVariant::Outline
                        label: DATA_MESSAGE_PUSH_LABEL
                        onclick: data_push_message(state.message_count)
                    }
                    euv_button {
                        variant: EuvButtonVariant::Outline
                        label: DATA_MESSAGE_POP_LABEL
                        onclick: data_pop_message(state.message_count)
                    }
                    euv_button {
                        variant: EuvButtonVariant::Outline
                        label: DATA_NOTIFICATION_PUSH_LABEL
                        onclick: data_push_notification(state.notification_count)
                    }
                    euv_button {
                        variant: EuvButtonVariant::Outline
                        label: DATA_NOTIFICATION_POP_LABEL
                        onclick: data_pop_notification(state.notification_count)
                    }
                    euv_popconfirm {
                        title: DATA_POPCONFIRM_TITLE
                        ok_label: DATA_POPCONFIRM_OK_LABEL
                        cancel_label: DATA_POPCONFIRM_CANCEL_LABEL
                        on_ok: data_on_popconfirm_ok()
                        on_cancel: data_on_popconfirm_cancel()
                        euv_button {
                            variant: EuvButtonVariant::Destructive
                            label: DATA_POPCONFIRM_TRIGGER_LABEL
                        }
                    }
                }
            }
            euv_message {
                items: message_items
            }
            euv_notification {
                items: notification_items
            }
        }
    }
}
