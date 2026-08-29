use super::*;

/// A custom attributes demo page showcasing static and dynamic attribute keys and values.
///
/// Static attributes use compile-time constant keys and values from `const.rs`.
/// Dynamic attributes allow runtime key and value input via text fields,
/// demonstrating the `{key}: value` syntax in the `html!` and `class!` macros.
///
/// # Returns
///
/// - `VirtualNode` - The custom attributes demo page virtual DOM tree.
#[component]
pub(crate) fn page_custom_attrs(node: VirtualNode<PageCustomAttrsProps>) -> VirtualNode {
    let PageCustomAttrsProps: PageCustomAttrsProps = node.try_get_props().unwrap_or_default();
    let dynamic_key: Signal<String> = App::use_signal(|| "data-custom".to_string());
    let dynamic_value: Signal<String> = App::use_signal(String::new);
    let class_prop_key: Signal<String> = App::use_signal(String::new);
    let class_prop_value: Signal<String> = App::use_signal(String::new);
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: "⚙️"
                title: "Custom Attributes"
                subtitle: "Dynamic attribute keys and values in both html! and class! macros."
            }
            euv_card {
                title: "HTML Dynamic Attribute (Variable Key & Value)"
                euv_input {
                    id: DYNAMIC_KEY_INPUT_ID
                    name: DYNAMIC_KEY_INPUT_ID
                    label: "Attribute Key"
                    placeholder: DYNAMIC_KEY_PLACEHOLDER
                    value: dynamic_key
                    autocomplete: ATTRS_AUTOCOMPLETE_OFF
                    oninput: attrs_on_input_key(dynamic_key)
                }
                euv_input {
                    id: DYNAMIC_VALUE_INPUT_ID
                    name: DYNAMIC_VALUE_INPUT_ID
                    label: "Attribute Value"
                    placeholder: DYNAMIC_VALUE_PLACEHOLDER
                    value: dynamic_value
                    autocomplete: ATTRS_AUTOCOMPLETE_OFF
                    oninput: attrs_on_input_value(dynamic_value)
                }
                div {
                    {
                        dynamic_key.get()
                    }
                    : dynamic_value
                    class: c_custom_attrs_demo()
                    p {
                        class: c_demo_text()
                        "This div has a dynamic attribute set by the inputs above."
                    }
                    if { !dynamic_key.get().is_empty() } {
                        p {
                            class: c_demo_text_muted()
                            {
                                format!("{}=\"{}\"", dynamic_key.get(), dynamic_value.get())
                            }
                        }
                    } else {
                        p {
                            class: c_demo_text_muted()
                            "Enter an attribute key and value above to see the result."
                        }
                    }
                }
            }
            euv_card {
                title: "CSS Dynamic Key (class! macro)"
                euv_input {
                    id: CLASS_KEY_INPUT_ID
                    name: CLASS_KEY_INPUT_ID
                    label: "CSS Property Key"
                    placeholder: CLASS_KEY_PLACEHOLDER
                    value: class_prop_key
                    autocomplete: ATTRS_AUTOCOMPLETE_OFF
                    oninput: attrs_on_input_key(class_prop_key)
                }
                euv_input {
                    id: CLASS_VALUE_INPUT_ID
                    name: CLASS_VALUE_INPUT_ID
                    label: "CSS Property Value"
                    placeholder: CLASS_VALUE_PLACEHOLDER
                    value: class_prop_value
                    autocomplete: ATTRS_AUTOCOMPLETE_OFF
                    oninput: attrs_on_input_value(class_prop_value)
                }
                div {
                    p {
                        class: c_attrs_dynamic_demo(&class_prop_key.get(), &class_prop_value.get())
                        "This paragraph uses a class with a dynamic CSS property key and value."
                    }
                    if { !class_prop_key.get().is_empty() && !class_prop_value.get().is_empty() } {
                        p {
                            class: c_demo_text_muted()
                            {
                                format!("{}: {}", class_prop_key.get(), class_prop_value.get())
                            }
                        }
                    } else {
                        p {
                            class: c_demo_text_muted()
                            "Enter a CSS property key and value above to see the result."
                        }
                    }
                }
            }
        }
    }
}
