use super::*;

/// A demo page for the advanced half of the component library.
///
/// The page puts the remaining components on one screen: a grid, a
/// sticky block, a time picker, a tree select, a cascader, a mentions
/// field and a transfer. Every control owns its selection through a
/// signal, so the page reads the current value back.
///
/// # Arguments
///
/// - `VirtualNode<PageUiAdvancedProps>` - The page component node
///   carrying the page props.
///
/// # Returns
///
/// - `VirtualNode` - The advanced demo page virtual DOM tree.
#[component]
pub(crate) fn page_ui_advanced(node: VirtualNode<PageUiAdvancedProps>) -> VirtualNode {
    let PageUiAdvancedProps: PageUiAdvancedProps = node.try_get_props().unwrap_or_default();
    let state: AdvancedDemoState = use_advanced_demo_state();
    let grid_cols: Vec<EuvGridCol> = vec![
        EuvGridCol { span: 6, offset: 0 },
        EuvGridCol {
            span: 12,
            offset: 0,
        },
        EuvGridCol { span: 6, offset: 0 },
    ];
    let region_nodes: Vec<EuvTreeNode> = vec![
        EuvTreeNode {
            label: ADVANCED_REGION_EUROPE_LABEL,
            value: ADVANCED_REGION_EUROPE_VALUE,
            children: vec![
                EuvTreeNode {
                    label: ADVANCED_REGION_FRANCE_LABEL,
                    value: ADVANCED_REGION_FRANCE_VALUE,
                    children: Vec::new(),
                },
                EuvTreeNode {
                    label: ADVANCED_REGION_GERMANY_LABEL,
                    value: ADVANCED_REGION_GERMANY_VALUE,
                    children: Vec::new(),
                },
            ],
        },
        EuvTreeNode {
            label: ADVANCED_REGION_ASIA_LABEL,
            value: ADVANCED_REGION_ASIA_VALUE,
            children: vec![EuvTreeNode {
                label: ADVANCED_REGION_JAPAN_LABEL,
                value: ADVANCED_REGION_JAPAN_VALUE,
                children: Vec::new(),
            }],
        },
    ];
    let location_options: Vec<EuvCascaderOption> = vec![
        EuvCascaderOption {
            value: ADVANCED_REGION_EUROPE_VALUE,
            label: ADVANCED_REGION_EUROPE_LABEL,
            children: vec![
                EuvCascaderOption {
                    value: ADVANCED_REGION_FRANCE_VALUE,
                    label: ADVANCED_REGION_FRANCE_LABEL,
                    children: Vec::new(),
                },
                EuvCascaderOption {
                    value: ADVANCED_REGION_GERMANY_VALUE,
                    label: ADVANCED_REGION_GERMANY_LABEL,
                    children: Vec::new(),
                },
            ],
        },
        EuvCascaderOption {
            value: ADVANCED_REGION_ASIA_VALUE,
            label: ADVANCED_REGION_ASIA_LABEL,
            children: vec![EuvCascaderOption {
                value: ADVANCED_REGION_JAPAN_VALUE,
                label: ADVANCED_REGION_JAPAN_LABEL,
                children: Vec::new(),
            }],
        },
    ];
    let mention_items: Vec<EuvMentionItem> = vec![
        EuvMentionItem {
            value: "ada",
            label: "Ada",
        },
        EuvMentionItem {
            value: "grace",
            label: "Grace",
        },
        EuvMentionItem {
            value: "alan",
            label: "Alan",
        },
    ];
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "🧰"
                title: ADVANCED_HEADER_TITLE
                subtitle: ADVANCED_HEADER_SUBTITLE
            }
            euv_card {
                title: ADVANCED_GRID_CARD_TITLE
                p {
                    class: c_badge_hint()
                    ADVANCED_GRID_HINT
                }
                euv_grid {
                    cols: grid_cols
                    gutter: true
                    div {
                        euv_text {
                            variant: EuvTextVariant::Secondary
                            text: ADVANCED_GRID_FIRST
                        }
                    }
                    div {
                        euv_text {
                            variant: EuvTextVariant::Secondary
                            text: ADVANCED_GRID_SECOND
                        }
                    }
                    div {
                        euv_text {
                            variant: EuvTextVariant::Secondary
                            text: ADVANCED_GRID_THIRD
                        }
                    }
                }
            }
            euv_card {
                title: ADVANCED_PICKER_CARD_TITLE
                euv_time_picker {
                    value: state.time_value
                    placeholder: ADVANCED_TIME_PLACEHOLDER
                    label: ADVANCED_TIME_LABEL
                    min_hour: ADVANCED_TIME_MIN_HOUR
                    max_hour: ADVANCED_TIME_MAX_HOUR
                    minute_step: ADVANCED_TIME_MINUTE_STEP
                }
                euv_tree_select {
                    nodes: region_nodes
                    value: state.tree_value
                    placeholder: ADVANCED_TREE_PLACEHOLDER
                    label: ADVANCED_TREE_LABEL
                }
                euv_cascader {
                    options: location_options
                    value: state.cascader_value
                    placeholder: ADVANCED_CASCADER_PLACEHOLDER
                    label: ADVANCED_CASCADER_LABEL
                    on_select: advanced_on_cascader_select()
                }
                euv_mentions {
                    items: mention_items
                    value: state.mention_value
                    placeholder: ADVANCED_MENTION_PLACEHOLDER
                    label: ADVANCED_MENTION_LABEL
                    on_select: advanced_on_mentions_select()
                }
            }
            euv_card {
                title: ADVANCED_TRANSFER_CARD_TITLE
                euv_transfer {
                    source: state.transfer_source.get()
                    target: state.transfer_target.get()
                    selected: state.transfer_selected
                    source_title: ADVANCED_TRANSFER_SOURCE_TITLE
                    target_title: ADVANCED_TRANSFER_TARGET_TITLE
                    on_move_to_target: advanced_on_move_to_target(
                    state.transfer_source,
                    state.transfer_target
                    )
                    on_move_to_source: advanced_on_move_to_source(
                    state.transfer_source,
                    state.transfer_target
                    )
                }
            }
            euv_card {
                title: ADVANCED_AFFIX_CARD_TITLE
                p {
                    class: c_badge_hint()
                    ADVANCED_AFFIX_HINT
                }
                euv_affix {
                    offset: ADVANCED_AFFIX_OFFSET
                    div {
                        euv_text {
                            variant: EuvTextVariant::Secondary
                            text: ADVANCED_AFFIX_TEXT
                        }
                    }
                }
            }
        }
    }
}
