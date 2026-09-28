use super::*;

/// The largest percentage a progress bar fill can occupy.
const PERCENT_MAX: f64 = 100.0;

/// A determinate progress bar component bound to a percentage signal.
///
/// Renders a `c_progress_container` track holding a
/// `c_progress_bar_fill` element whose inline `width` is driven by the
/// `percent` signal, plus the label and a numeric readout. A running bar
/// adds the animated `c_progress_bar_running` class; a settled one falls
/// back to `c_progress_bar_stopped` so the fill keeps its inline width
/// without re-running the keyframe animation.
///
/// # Arguments
///
/// - `VirtualNode<EuvProgressProps>` - The props node containing bar configuration.
///
/// # Returns
///
/// - `VirtualNode` - A styled progress bar.
#[component]
pub fn euv_progress(node: VirtualNode<EuvProgressProps>) -> VirtualNode {
    let EuvProgressProps {
        percent,
        label: label_text,
        active,
    }: EuvProgressProps = node.try_get_props().unwrap_or_default();
    let clamped: f64 = progress_percent_clamp(percent.get());
    let fill_style: String = format!("width: {clamped}%;");
    let fill_style_ref: &str = fill_style.as_str();
    let readout: String = format!("{clamped:.0}%");
    html! {
        div {
            div {
                class: c_progress_container()
                div {
                    class: c_progress_bar_fill()
                    class: if { active } {
                        c_progress_bar_running()
                    } else {
                        c_progress_bar_stopped()
                    }
                    style: fill_style_ref
                }
            }
            div {
                class: c_euv_slider_header()
                span {
                    class: c_form_label()
                    label_text
                }
                span {
                    class: c_binding_slider_value()
                    readout
                }
            }
        }
    }
}

/// Clamps a raw percentage into the `0.0..=100.0` range a bar can render.
///
/// NaN collapses to `0.0` — a `Signal<f64>` carrying NaN (an
/// unreported upload size, a `0.0 / 0.0` ratio) would otherwise emit a
/// `width: NaN%` declaration the browser drops silently.
///
/// # Arguments
///
/// - `f64` - The raw percentage supplied by the caller.
///
/// # Returns
///
/// - `f64` - The percentage clamped into the `0.0..=100.0` range.
pub fn progress_percent_clamp(percent: f64) -> f64 {
    if percent.is_nan() {
        return 0.0;
    }
    percent.clamp(0.0, PERCENT_MAX)
}
