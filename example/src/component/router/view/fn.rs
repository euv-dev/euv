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
                ROUTE_HOME | ROUTE_ABOUT => {
                    page_about {}
                }
                ROUTE_ANIMATION => {
                    page_animation {}
                }
                ROUTE_CUSTOM_ATTRS => {
                    page_custom_attrs {}
                }
                ROUTE_BADGE => {
                    page_badge {}
                }
                ROUTE_COMPONENT_BINDING => {
                    page_component_binding {}
                }
                ROUTE_BROWSER => {
                    page_browser {}
                }
                ROUTE_CAMERA => {
                    page_camera {}
                }
                ROUTE_CANVAS => {
                    page_canvas {}
                }
                ROUTE_CONDITIONAL => {
                    page_conditional {}
                }
                ROUTE_COUNTER => {
                    page_counter {}
                }
                ROUTE_DYNAMIC_COMPONENT => {
                    page_dynamic_component {}
                }
                ROUTE_EVENT => {
                    page_event {}
                }
                ROUTE_FORM => {
                    page_form {}
                }
                ROUTE_HOOKS_TIMING => {
                    page_hooks_timing {}
                }
                ROUTE_HOOKS_ASYNC => {
                    page_hooks_async {}
                }
                ROUTE_HOOKS_PROTECT => {
                    page_hooks_protect {}
                }
                ROUTE_HOOKS_I18N => {
                    page_hooks_i18n {}
                }
                ROUTE_GAME_2D => {
                    page_game_2d {}
                }
                ROUTE_GAME_3D => {
                    page_game_3d {}
                }
                GESTURE_ROUTE => {
                    page_gesture {}
                }
                ROUTE_KEEP_ALIVE => {
                    page_keep_alive {}
                }
                ROUTE_LIFECYCLE => {
                    page_lifecycle {}
                }
                ROUTE_LIGHTING => {
                    page_lighting {}
                }
                ROUTE_LIST => {
                    page_list {}
                }
                ROUTE_MODAL => {
                    page_modal {}
                }
                ROUTE_OBSERVER => {
                    page_observer {}
                }
                ROUTE_RAYTRACE => {
                    page_raytrace {}
                }
                ROUTE_SSE => {
                    page_sse {}
                }
                ROUTE_SELECT => {
                    page_select {}
                }
                ROUTE_TIMER => {
                    page_timer {}
                }
                ROUTE_FILE_UPLOAD => {
                    page_file_upload {}
                }
                ROUTE_VIRTUAL_LIST => {
                    page_virtual_list {}
                }
                ROUTE_WEBSOCKET => {
                    page_websocket {}
                }
                _ => {
                    page_not_found {}
                }
            }
        }
    }
}
