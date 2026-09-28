use super::*;

/// A generic hero section aligned with common site frameworks.
///
/// Renders the radial glow, the big title, an optional tagline and optional
/// action buttons (internal hash routes or external URLs).
///
/// # Arguments
///
/// - `VirtualNode<EuvHeroProps>` - The props node.
///
/// # Returns
///
/// - `VirtualNode` - The hero virtual DOM tree.
#[component]
pub fn euv_hero(node: VirtualNode<EuvHeroProps>) -> VirtualNode {
    let EuvHeroProps {
        title,
        subtitle,
        actions,
    }: EuvHeroProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_home()
            div {
                class: c_page_glow()
            }
            div {
                class: c_home_content()
                h1 {
                    class: c_home_title()
                    {
                        title
                    }
                }
                if { !subtitle.is_empty() } {
                    p {
                        class: c_home_subtitle()
                        {
                            subtitle
                        }
                    }
                }
                if { !actions.is_empty() } {
                    div {
                        class: c_home_actions()
                        for action in actions.iter() {
                            euv_hero_action {
                                action: *action
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Renders one hero action button (internal route or external URL).
///
/// # Arguments
///
/// - `VirtualNode<EuvHeroActionProps>` - The props node.
///
/// # Returns
///
/// - `VirtualNode` - The action button virtual DOM tree.
#[component]
pub fn euv_hero_action(node: VirtualNode<EuvHeroActionProps>) -> VirtualNode {
    let EuvHeroActionProps { action }: EuvHeroActionProps =
        node.try_get_props().unwrap_or_default();
    let external: bool = action.link.starts_with("http");
    let button_class: fn() -> &'static Css = if action.primary {
        c_home_btn_primary
    } else {
        c_home_btn_secondary
    };
    if external {
        html! {
            a {
                class: {
                    button_class()
                }
                href: action.link
                target: "_blank"
                onclick: Router::external_link_handler(action.link)
                {
                    action.text
                }
            }
        }
    } else {
        html! {
            a {
                class: {
                    button_class()
                }
                href: {
                    let mut
                    href: String =
                    String::with_capacity(ROUTE_HASH_PREFIX.len() + action.link.len());
                    href.push_str(ROUTE_HASH_PREFIX);
                    href.push_str(action.link);
                    href
                }
                onclick: Router::link_handler(action.link)
                {
                    action.text
                }
            }
        }
    }
}
