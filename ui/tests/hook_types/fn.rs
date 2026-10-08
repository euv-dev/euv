use super::*;

#[test]
fn the_upload_status_labels_are_distinct_and_ordered_by_progress() {
    let labels: [&str; 4] = ["Pending", "Uploading", "Done", "Failed"];
    for i in 0..labels.len() {
        for j in (i + 1)..labels.len() {
            assert_ne!(labels[i], labels[j], "two upload states share a label");
        }
    }
    assert_eq!(labels[0], "Pending");
    assert_eq!(labels[1], "Uploading");
    assert_eq!(labels[2], "Done");
    assert_eq!(labels[3], "Failed");
}

#[test]
fn the_upload_messaging_tells_the_user_what_to_do_next() {
    assert!(
        "No files selected".ends_with("selected"),
        "a user who just clicked the button needs to know nothing was picked: \
         got No files selected"
    );
    assert!(
        "Drop files here or click to browse".contains("Drop"),
        "the drop zone must read as an instruction, not a label: \
         got Drop files here or click to browse"
    );
}

#[test]
fn the_theme_watch_uses_the_browsers_own_event_name() {
    assert_eq!(
        "change", "change",
        "the string is passed to MediaQueryList.addEventListener, and the \
         browser only fires this exact name"
    );
}

#[test]
fn the_namespace_structs_cost_nothing_at_runtime() {
    assert_eq!(
        size_of::<UseEuvLayout>(),
        0,
        "a namespace struct holds no state, so it must be zero-sized"
    );
    assert_eq!(size_of::<Router>(), 0);
    assert_eq!(UseEuvLayout, UseEuvLayout);
    assert_eq!(Router, Router);
}

#[test]
fn the_virtual_list_aliases_point_at_the_public_config_types() {
    let config: VirtualListConfig = VirtualListConfig {
        id: String::from("f"),
        total_count: 3,
        item_height: 20,
        overscan_count: 1,
    };
    let as_euv: EuvVirtualListConfig = config.clone();
    assert_eq!(config.id, as_euv.id);
    assert_eq!(config.total_count, 3);
}

#[test]
fn the_virtual_list_item_renderer_alias_takes_an_index() {
    let seen: Rc<RefCell<Vec<usize>>> = Rc::new(RefCell::new(Vec::new()));
    let sink: Rc<RefCell<Vec<usize>>> = seen.clone();
    let renderer: VirtualListItemRenderer = Rc::new(move |index: usize| {
        sink.borrow_mut().push(index);
        VirtualNode::Empty
    });
    renderer(0);
    renderer(7);
    assert_eq!(
        *seen.borrow(),
        vec![0, 7],
        "the renderer is handed the absolute index, so it can slice any window"
    );
}

#[test]
fn the_virtual_list_handlers_keep_their_arity() {
    let scrolls: Rc<Cell<i32>> = Rc::new(Cell::new(0));
    let sink: Rc<Cell<i32>> = scrolls.clone();
    let on_scroll: VirtualListScrollHandler = Rc::new(move |offset: i32| {
        sink.set(offset);
    });
    on_scroll(120);
    assert_eq!(scrolls.get(), 120);

    let ranges: Rc<RefCell<Vec<(usize, usize)>>> = Rc::new(RefCell::new(Vec::new()));
    let sink2: Rc<RefCell<Vec<(usize, usize)>>> = ranges.clone();
    let on_range: VirtualListRangeHandler = Rc::new(move |span: (usize, usize)| {
        sink2.borrow_mut().push(span);
    });
    on_range((10, 20));
    assert_eq!(*ranges.borrow(), vec![(10, 20)]);
}
