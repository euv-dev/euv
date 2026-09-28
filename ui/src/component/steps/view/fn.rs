use super::*;

/// A horizontal progress stepper marking each stage of a multi-step flow.
///
/// Renders a `c_euv_steps` row holding one `c_euv_step` per entry, with a
/// `c_euv_step_line` dashed connector drawn between consecutive steps
/// (never after the last one). The caller-owned `current` signal decides
/// each step's state: an index below `current` is done (`c_euv_step_done`,
/// marker showing a check), the index equal to `current` is active
/// (`c_euv_step_active`), and anything above stays on the neutral
/// `c_euv_step`. Because the state lives in reactive `class:` conditionals,
/// advancing `current` repaints the affected markers without rebuilding
/// the list.
///
/// # Arguments
///
/// - `VirtualNode<EuvStepsProps>` - The props node containing steps and the current index.
///
/// # Returns
///
/// - `VirtualNode` - The stepper virtual DOM tree.
#[component]
pub fn euv_steps(node: VirtualNode<EuvStepsProps>) -> VirtualNode {
    let EuvStepsProps { steps, current }: EuvStepsProps = node.try_get_props().unwrap_or_default();
    let total: usize = steps.len();
    let mut parts: Vec<VirtualNode> = Vec::with_capacity(total * 2);
    for (index, step) in steps.into_iter().enumerate() {
        parts.push(euv_step(index, step, current));
        if index + 1 < total {
            parts.push(html! {
                div {
                    class: c_euv_step_line()
                }
            });
        }
    }
    html! {
        div {
            class: c_euv_steps()
            parts
        }
    }
}

/// Renders one step block: its state class, marker, title, and description.
///
/// The marker reads `current` itself rather than taking a pre-computed
/// index, so the `✓` / number swap and the state class always resolve from
/// the same signal read and can never disagree with each other.
///
/// # Arguments
///
/// - `usize` - The zero-based position of this step in the list.
/// - `EuvStep` - The step entry to render.
/// - `Signal<usize>` - The index of the step currently in progress.
///
/// # Returns
///
/// - `VirtualNode` - The step block virtual DOM tree.
fn euv_step(index: usize, step: EuvStep, current: Signal<usize>) -> VirtualNode {
    html! {
        div {
            key: step.title
            class: if { index == current.get() } {
                c_euv_step_active()
            } else if { index < current.get() } {
                c_euv_step_done()
            } else {
                c_euv_step()
            }
            div {
                class: c_euv_step_marker()
                if { index < current.get() } {
                    "✓"
                } else {
                    { (index + 1).to_string() }
                }
            }
            div {
                class: c_euv_step_title()
                step.title
            }
            if { !step.description.is_empty() } {
                div {
                    class: c_euv_step_desc()
                    step.description
                }
            }
        }
    }
}
