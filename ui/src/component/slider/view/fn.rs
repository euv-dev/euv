use super::*;

/// A range slider component with two-way binding via a signal.
///
/// Renders a header row holding the label and a live value readout, then
/// the slider itself: a track, a range that fills it up to the current
/// value, a thumb that rides on the fill, and the native `<input
/// type="range">` that stays the accessible control.
///
/// The input sits over the painted parts rather than replacing them: the
/// browser still owns the keyboard behaviour, the form value and the
/// announced range, while the track, the range and the thumb carry the
/// appearance. The painted slider and the real control therefore never
/// disagree.
///
/// The readout and the range share the current value: the range's width is
/// the percentage and the thumb's offset is the same percentage, while the
/// value span carries `c_binding_slider_value` so the track gradient stops
/// at the right offset. A caller-supplied `oninput` takes precedence over
/// the built-in clamping handler.
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
    let range_fill: String = format!("width: {percent}%;");
    let thumb_offset: String = format!("inset-inline-start: {percent}%;");
    html! {
        div {
            "data-slot": "slider"
            "data-orientation": SLIDER_ORIENTATION_HORIZONTAL
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
            div {
                "data-slot": "slider-track"
                "data-orientation": SLIDER_ORIENTATION_HORIZONTAL
                class: c_euv_slider_track()
                div {
                    "data-slot": "slider-range"
                    "data-orientation": SLIDER_ORIENTATION_HORIZONTAL
                    class: c_euv_slider_range()
                    style: range_fill
                }
                div {
                    "data-slot": "slider-thumb"
                    "data-orientation": SLIDER_ORIENTATION_HORIZONTAL
                    style: thumb_offset
                    class: c_euv_slider_thumb()
                }
                input {
                    id: id
                    name: name
                    type: "range"
                    "data-orientation": SLIDER_ORIENTATION_HORIZONTAL
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
}
