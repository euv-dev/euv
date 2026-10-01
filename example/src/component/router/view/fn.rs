use super::*;

/// Resolves the current route to the corresponding page virtual DOM tree.
///
/// Matches the route string against all registered page paths and returns
/// the appropriate page component. Falls back to a 404 page for unknown routes.
///
/// # Arguments
///
/// - `VirtualNode<PageRouterProps>` - The typed props containing the route signal.
///
/// # Returns
///
/// - `VirtualNode` - The virtual DOM tree of the matched page.
#[component]
pub(crate) fn page_router(node: VirtualNode<PageRouterProps>) -> VirtualNode {
    let PageRouterProps { route_signal }: PageRouterProps =
        node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_page_router()
            match { route_signal.get().as_str() } {
                "/" | "/about" => {
                    page_about {}
                }
                "/animation" => {
                    page_animation {}
                }
                "/custom-attrs" => {
                    page_custom_attrs {}
                }
                "/badge" => {
                    page_badge {}
                }
                "/component-binding" => {
                    page_component_binding {}
                }
                "/browser" => {
                    page_browser {}
                }
                "/camera" => {
                    page_camera {}
                }
                "/canvas" => {
                    page_canvas {}
                }
                "/conditional" => {
                    page_conditional {}
                }
                "/counter" => {
                    page_counter {}
                }
                "/dynamic-component" => {
                    page_dynamic_component {}
                }
                "/event" => {
                    page_event {}
                }
                "/form" => {
                    page_form {}
                }
                "/hooks-timing" => {
                    page_hooks_timing {}
                }
                "/hooks-async" => {
                    page_hooks_async {}
                }
                "/hooks-protect" => {
                    page_hooks_protect {}
                }
                "/hooks-i18n" => {
                    page_hooks_i18n {}
                }
                "/game-2d" => {
                    page_game_2d {}
                }
                "/game-3d" => {
                    page_game_3d {}
                }
                "/gesture" => {
                    page_gesture {}
                }
                "/keep-alive" => {
                    page_keep_alive {}
                }
                "/lifecycle" => {
                    page_lifecycle {}
                }
                "/lighting" => {
                    page_lighting {}
                }
                "/list" => {
                    page_list {}
                }
                "/modal" => {
                    page_modal {}
                }
                "/observer" => {
                    page_observer {}
                }
                "/raytrace" => {
                    page_raytrace {}
                }
                "/sse" => {
                    page_sse {}
                }
                "/select" => {
                    page_select {}
                }
                "/timer" => {
                    page_timer {}
                }
                "/file-upload" => {
                    page_file_upload {}
                }
                "/virtual-list" => {
                    page_virtual_list {}
                }
                "/websocket" => {
                    page_websocket {}
                }
                _ => {
                    page_not_found {}
                }
            }
        }
    }
}
