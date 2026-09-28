use super::*;

/// A range slider component with two-way binding via a signal.
///
/// Renders a header row holding the label and a live value readout, then
/// an `<input type="range">` styled by `c_euv_slider_input`. The readout
/// and the track fill share the current value: the value span carries
/// `c_slider_value` so the track gradient stops at the right offset, and
/// the header row carries `c_euv_slider_row` / `c_euv_slider_header`.
/// A caller-supplied `oninput` takes precedence over the built-in
/// clamping handler.
///
/// # Arguments
///
/// - `VirtualNode<EuvSliderProps>` - The props node containing slider configuration.
///
/// # Returns
///
/// - `VirtualNode` - A styled labeled range slider.
#[component]
pub fn euv_slider(node: VirtualNode<EuvSliderProps>) -> VirtualNode {
    let EuvSliderProps {
        id,
        name,
        min,
        max,
        step,
        value,
        label: label_text,
        oninput: custom_oninput,
    }: EuvSliderProps = node.try_get_props().unwrap_or_default();
    let handler: Option<Rc<dyn Fn(Event)>> =
        custom_oninput.or_else(|| EuvSliderHelpers::on_slider_input(value, min, max));
    let percent: f64 = EuvSliderHelpers::percent(value.get(), min, max);
    let fill: String = format!("--value: {percent}%;");
    let readout: String = format!("{percent:.0}%");
    html! {
        div {
            class: c_euv_slider_row()
            div {
                class: c_euv_slider_header()
                label {
                    for: id
                    class: c_form_label()
                    label_text
                }
                span {
                    class: c_binding_slider_value()
                    readout
                }
            }
            input {
                id: id
                name: name
                type: "range"
                min: min.to_string()
                max: max.to_string()
                step: step.to_string()
                value: value.get().to_string()
                class: c_euv_slider_input()
                style: fill
                oninput: handler
            }
        }
    }
}
