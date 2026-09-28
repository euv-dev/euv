use super::*;

/// A generic borderless feature card grid aligned with common site frameworks.
///
/// Renders nothing when `features` is empty; the grid collapses to one column
/// on small viewports. The class names are the historical `c_home_*` /
/// `c_feature_*` set, kept verbatim so existing pages do not visually
/// regress after the move out of the `hero` module.
///
/// # Arguments
///
/// - `VirtualNode<EuvFeatureGridProps>` - The props node.
///
/// # Returns
///
/// - `VirtualNode` - The feature grid virtual DOM tree.
#[component]
pub fn euv_feature_grid(node: VirtualNode<EuvFeatureGridProps>) -> VirtualNode {
    let EuvFeatureGridProps { features }: EuvFeatureGridProps =
        node.try_get_props().unwrap_or_default();
    if features.is_empty() {
        return html! {
            ""
        };
    }
    html! {
        div {
            class: c_home_feature_grid()
            for feature in features.iter() {
                div {
                    class: c_feature_card()
                    key: feature.title
                    div {
                        class: c_feature_header()
                        if !feature.icon.is_empty() {
                            span {
                                class: c_feature_icon()
                                {
                                    feature.icon
                                }
                            }
                        }
                        span {
                            class: c_feature_name()
                            {
                                feature.title
                            }
                        }
                    }
                    p {
                        class: c_feature_desc()
                        {
                            feature.details
                        }
                    }
                }
            }
        }
    }
}
