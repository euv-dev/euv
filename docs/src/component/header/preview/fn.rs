use super::*;

/// Renders the search controller's current preview snapshot.
///
/// # Arguments
///
/// - `Signal<String>` - Current route.
/// - `Signal<String>` - Search query.
/// - `Signal<HeaderSearchState>` - Interaction state.
/// - `String` - Stable unique input id.
/// - `NodeRef<Element>` - Click-outside boundary.
///
/// # Returns
///
/// - `VirtualNode` - Accessible input and bounded result list.
pub(crate) fn header_search_preview(
    route_signal: Signal<String>,
    query: Signal<String>,
    state: Signal<HeaderSearchState>,
    input_id: String,
    target: NodeRef<Element>,
) -> VirtualNode {
    let results: Vec<DocsPage> = header_search_matches(crate::generated::SITE.pages, &query.get());
    let snapshot: HeaderSearchState = state.get();
    let open: bool = snapshot.get_open();
    let list_id: String = format!("{input_id}-listbox");
    let active_id: String = if open {
        snapshot
            .get_active()
            .filter(|index: &usize| *index < results.len())
            .map(|index: usize| format!("{input_id}-option-{index}"))
            .unwrap_or_default()
    } else {
        String::new()
    };
    let status: String = if !open {
        String::new()
    } else if query.get().trim().is_empty() {
        HEADER_SEARCH_EMPTY.to_string()
    } else if results.is_empty() {
        HEADER_SEARCH_NONE.to_string()
    } else {
        format!("{} matching pages", results.len())
    };
    let rows: Vec<VirtualNode> = results
        .iter()
        .enumerate()
        .map(|(index, page): (usize, &DocsPage)| {
            let route: &'static str = page.route;
            let title: &'static str = page.title;
            let active: bool = open && snapshot.get_active() == Some(index);
            html! {
                div {
                    id: format!("{input_id}-option-{index}")
                    role: HEADER_SEARCH_OPTION
                    "aria-selected": if active {
                        HEADER_SEARCH_TRUE
                    } else {
                        HEADER_SEARCH_FALSE
                    }
                    class: if active {
                        c_docs_header_search_option_active()
                    } else {
                        c_docs_header_search_option()
                    }
                    onmousedown: move |event: Event| {
                        event.prevent_default();
                    }
                    onclick: move |_event: Event| {
                        header_search_select(route_signal, query, state, route);
                    }
                    span {
                        title
                    }
                    span {
                        class: c_docs_header_search_route() route
                    }
                }
            }
        })
        .collect();
    html! {
        div {
            ref: target
            class: CLASS_DOCS_HEADER_SEARCH
            input {
                id: input_id
                class: CLASS_DOCS_HEADER_SEARCH_INPUT
                type: HEADER_SEARCH_TYPE
                role: HEADER_SEARCH_COMBOBOX
                "aria-label": HEADER_SEARCH_LABEL
                "aria-autocomplete": HEADER_SEARCH_LIST
                "aria-haspopup": HEADER_SEARCH_LISTBOX
                "aria-expanded": if open {
                    HEADER_SEARCH_TRUE
                } else {
                    HEADER_SEARCH_FALSE
                }
                "aria-controls": list_id.clone()
                "aria-activedescendant": active_id
                autocomplete: HEADER_SEARCH_OFF
                placeholder: HEADER_SEARCH_PLACEHOLDER
                value: query
                onfocus: move |_event: Event| {
                    header_search_focus(state);
                }
                onblur: move |_event: Event| {
                    state.set(HeaderSearchState::default());
                }
                oninput: move |event: Event| {
                    if let Some(input) = event.target().and_then(|target: EventTarget| target.dyn_into::<HtmlInputElement>().ok()) {
                        query.set(input.value());
                        header_search_focus(state);
                    }
                }
                onkeydown: header_search_keydown(route_signal, query, state)
            }
            div {
                class: c_docs_header_search_panel()
                class: if open {
                    Css::default()
                } else {
                    c_docs_header_search_panel_closed().clone()
                }
                "aria-hidden": if open {
                    HEADER_SEARCH_FALSE
                } else {
                    HEADER_SEARCH_TRUE
                }
                div {
                    id: list_id
                    role: HEADER_SEARCH_LISTBOX
                    "aria-label": HEADER_SEARCH_LABEL
                    rows
                }
                div {
                    role: HEADER_SEARCH_STATUS
                    "aria-live": HEADER_SEARCH_POLITE
                    "aria-atomic": HEADER_SEARCH_TRUE
                    class: c_docs_header_search_status()
                    status
                }
            }
        }
    }
}

/// Adapts native keys to controller intents, preserving ordinary input editing.
///
/// # Arguments
///
/// - `Signal<String>` - Current route.
/// - `Signal<String>` - Current query.
/// - `Signal<HeaderSearchState>` - Interaction state.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - Composition-safe keyboard handler.
fn header_search_keydown(
    route_signal: Signal<String>,
    query: Signal<String>,
    state: Signal<HeaderSearchState>,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        let Some(keyboard) = event.dyn_ref::<KeyboardEvent>() else {
            return;
        };
        if keyboard.ctrl_key() || keyboard.alt_key() || keyboard.meta_key() || keyboard.shift_key()
        {
            return;
        }
        let results: Vec<DocsPage> =
            header_search_matches(crate::generated::SITE.pages, &query.get());
        let mut snapshot: HeaderSearchState = state.get();
        let (consumed, route): (bool, Option<&'static str>) = snapshot.key(
            &keyboard.key(),
            keyboard.is_composing() || keyboard.key_code() == 229,
            &results,
        );
        state.set(snapshot);
        if consumed {
            event.prevent_default();
        }
        if let Some(route) = route {
            header_search_select(route_signal, query, state, route);
        }
    }))
}
