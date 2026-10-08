use super::*;

type OptionalRcHandler = Option<Rc<dyn Fn(Event)>>;

fn handler() -> NativeEventHandler {
    NativeEventHandler::create("click", |_: Event| {})
}

fn is_event(value: &AttributeValue) -> bool {
    matches!(value, AttributeValue::Event(_))
}

fn is_empty_text(value: &AttributeValue) -> bool {
    matches!(value, AttributeValue::Text(text) if text.is_empty())
}

#[test]
fn a_bare_handler_becomes_an_event_attribute() {
    let adapter: EventAdapter<NativeEventHandler> = EventAdapter::new(handler());
    let observed: AttributeValue = adapter.into_attribute("click");
    assert!(
        is_event(&observed),
        "a present handler becomes an Event attribute"
    );
}

#[test]
fn the_event_name_is_overridden_by_the_attribute_position() {
    let adapter: EventAdapter<NativeEventHandler> = EventAdapter::new(handler());
    let observed: AttributeValue = adapter.into_attribute("dblclick");
    assert!(
        is_event(&observed),
        "the attribute position wins over the handler's own name"
    );
}

#[test]
fn a_present_optional_handler_becomes_an_event_attribute() {
    let adapter: EventAdapter<Option<NativeEventHandler>> = EventAdapter::new(Some(handler()));
    let observed: AttributeValue = adapter.into_attribute("click");
    assert!(is_event(&observed), "Some becomes an Event attribute");
}

#[test]
fn an_absent_optional_handler_becomes_empty_text() {
    let adapter: EventAdapter<Option<NativeEventHandler>> = EventAdapter::new(None);
    let observed: AttributeValue = adapter.into_attribute("click");
    assert!(
        is_empty_text(&observed),
        "None cannot be an Event attribute, so it degrades to empty text"
    );
}

#[test]
fn an_absent_closure_handler_becomes_empty_text() {
    let adapter: EventAdapter<OptionalRcHandler> = EventAdapter::new(None);
    let observed: AttributeValue = adapter.into_attribute("click");
    assert!(
        is_empty_text(&observed),
        "a missing callback degrades the same way a missing handler does"
    );
}

#[test]
fn a_present_closure_handler_becomes_an_event_attribute() {
    let callback: Rc<dyn Fn(Event)> = Rc::new(|_: Event| {});
    let adapter: EventAdapter<OptionalRcHandler> = EventAdapter::new(Some(callback));
    let observed: AttributeValue = adapter.into_attribute("click");
    assert!(
        is_event(&observed),
        "Some wraps the closure into an Event attribute"
    );
}

#[test]
fn a_named_handler_adapter_becomes_an_event_attribute() {
    let adapter: EventNamedAdapter<NativeEventHandler> = EventNamedAdapter::new(handler(), "click");
    let observed: AttributeValue = adapter.into();
    assert!(
        is_event(&observed),
        "the named adapter produces an Event attribute"
    );
}

#[test]
fn a_named_closure_adapter_becomes_an_event_attribute() {
    let callback: Rc<dyn Fn(Event)> = Rc::new(|_: Event| {});
    let adapter: EventNamedAdapter<OptionalRcHandler> =
        EventNamedAdapter::new(Some(callback), "click");
    let observed: AttributeValue = adapter.into();
    assert!(
        is_event(&observed),
        "the named closure adapter produces an Event attribute"
    );
}

#[test]
fn a_named_closure_adapter_without_a_callback_becomes_empty_text() {
    let adapter: EventNamedAdapter<OptionalRcHandler> = EventNamedAdapter::new(None, "click");
    let observed: AttributeValue = adapter.into();
    assert!(is_empty_text(&observed), "no callback means empty text");
}

#[test]
fn a_handler_attr_value_adapter_becomes_an_event_attribute() {
    let adapter: AttrValueAdapter<NativeEventHandler> = AttrValueAdapter::new(handler());
    let observed: AttributeValue = adapter.into_callback_named("click");
    assert!(
        is_event(&observed),
        "the callback path produces an Event attribute"
    );
}

#[test]
fn a_present_optional_handler_attr_adapter_becomes_an_event_attribute() {
    let adapter: AttrValueAdapter<Option<NativeEventHandler>> =
        AttrValueAdapter::new(Some(handler()));
    let observed: AttributeValue = adapter.into_callback();
    assert!(is_event(&observed), "Some becomes an Event attribute");
}

#[test]
fn an_absent_optional_handler_attr_adapter_becomes_empty_text() {
    let adapter: AttrValueAdapter<Option<NativeEventHandler>> = AttrValueAdapter::new(None);
    let observed: AttributeValue = adapter.into_callback();
    assert!(is_empty_text(&observed), "None degrades to empty text");
}

#[test]
fn an_absent_optional_handler_attr_adapter_degrades_the_named_path_too() {
    let adapter: AttrValueAdapter<Option<NativeEventHandler>> = AttrValueAdapter::new(None);
    let observed: AttributeValue = adapter.into_callback_named("click");
    assert!(
        is_empty_text(&observed),
        "the named path degrades exactly like the unnamed one"
    );
}

#[test]
fn a_named_callback_adapter_becomes_an_event_attribute() {
    let adapter: CallbackNamedAdapter<Box<dyn FnMut(Event)>> =
        CallbackNamedAdapter::new(Box::new(|_: Event| {}), "click");
    let observed: AttributeValue = adapter.into();
    assert!(
        is_event(&observed),
        "the named callback adapter wraps its closure into an Event attribute"
    );
}

#[test]
fn a_named_callback_adapter_takes_its_name_from_the_adapter() {
    let adapter: CallbackNamedAdapter<Box<dyn FnMut(Event)>> =
        CallbackNamedAdapter::new(Box::new(|_: Event| {}), "dblclick");
    let observed: AttributeValue = adapter.into();
    assert!(
        is_event(&observed),
        "the adapter supplies the event name itself"
    );
}

#[test]
fn an_owned_string_inner_html_becomes_an_inner_html_attribute() {
    let adapter: InnerHtmlAdapter<String> = InnerHtmlAdapter::new("<b>hi</b>".to_string());
    let observed: AttributeValue = adapter.into();
    match observed {
        AttributeValue::InnerHtml(markup) => {
            assert_eq!(
                markup, "<b>hi</b>",
                "the markup is carried through verbatim"
            )
        }
        _ => unreachable!("an owned string must become InnerHtml"),
    }
}

#[test]
fn a_borrowed_str_inner_html_is_owned_on_the_way_in() {
    let adapter: InnerHtmlAdapter<&str> = InnerHtmlAdapter::new("<i>x</i>");
    let observed: AttributeValue = adapter.into();
    match observed {
        AttributeValue::InnerHtml(markup) => assert_eq!(
            markup, "<i>x</i>",
            "a borrowed slice is copied so the attribute can outlive it"
        ),
        _ => unreachable!("a borrowed slice must become InnerHtml"),
    }
}

#[test]
fn an_empty_inner_html_stays_empty() {
    let adapter: InnerHtmlAdapter<String> = InnerHtmlAdapter::new(String::new());
    let observed: AttributeValue = adapter.into();
    match observed {
        AttributeValue::InnerHtml(markup) => assert!(markup.is_empty(), "no markup, no content"),
        _ => unreachable!("an empty string must still be InnerHtml, not Text"),
    }
}

#[test]
fn a_signal_inner_html_becomes_a_signal_backed_attribute() {
    let adapter: InnerHtmlAdapter<Signal<String>> =
        InnerHtmlAdapter::new(Signal::create("<b>live</b>".to_string()));
    let observed: AttributeValue = adapter.into();
    assert!(
        matches!(observed, AttributeValue::InnerHtmlSignal(_)),
        "a signal inner-html is its own variant, not the plain one"
    );
}

#[test]
fn the_event_and_text_degradation_never_collide() {
    let present_adapter: EventAdapter<Option<NativeEventHandler>> =
        EventAdapter::new(Some(handler()));
    let absent_adapter: EventAdapter<Option<NativeEventHandler>> = EventAdapter::new(None);
    let present: AttributeValue = present_adapter.into_attribute("click");
    let absent: AttributeValue = absent_adapter.into_attribute("click");
    assert!(
        is_event(&present) && is_empty_text(&absent),
        "the two outcomes stay distinct"
    );
}

#[test]
fn a_bare_handler_converts_into_an_event_attribute_without_recursing() {
    let observed: AttributeValue = handler().into();
    assert!(
        is_event(&observed),
        "From<NativeEventHandler> must be the base case of the conversion cycle, \
         not a bounce through AttrValueAdapter back into itself"
    );
}

#[test]
fn a_bare_optional_handler_converts_without_recursing() {
    let present: AttributeValue = Some(handler()).into();
    let absent: AttributeValue = (None as Option<NativeEventHandler>).into();
    assert!(is_event(&present), "Some converts to an Event attribute");
    assert!(is_empty_text(&absent), "None converts to empty text");
}
