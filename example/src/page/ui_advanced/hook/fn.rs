use super::*;

/// Builds the reactive state the advanced demo page drives.
///
/// # Returns
///
/// - `AdvancedDemoState` - The signals the page drives.
pub(crate) fn use_advanced_demo_state() -> AdvancedDemoState {
    AdvancedDemoState {
        time_value: App::use_signal(String::new),
        transfer_selected: App::use_signal(Vec::new),
        tree_value: App::use_signal(String::new),
        cascader_value: App::use_signal(Vec::new),
        mention_value: App::use_signal(String::new),
        transfer_source: App::use_signal(|| {
            vec![
                EuvTransferItem {
                    value: "read",
                    label: "Read",
                    disabled: false,
                },
                EuvTransferItem {
                    value: "write",
                    label: "Write",
                    disabled: false,
                },
                EuvTransferItem {
                    value: "admin",
                    label: "Admin",
                    disabled: false,
                },
            ]
        }),
        transfer_target: App::use_signal(|| {
            vec![EuvTransferItem {
                value: "review",
                label: "Review",
                disabled: false,
            }]
        }),
    }
}

/// Builds the handler that reports the path the cascader chose.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(&'static str)>>` - The select handler of the cascader.
pub(crate) fn advanced_on_cascader_select() -> Option<Rc<dyn Fn(&'static str)>> {
    Some(Rc::new(move |picked: &'static str| {
        Console::log(ADVANCED_CASCADER_LOG.replace(ADVANCED_LOG_VALUE_PLACEHOLDER, picked));
    }))
}

/// Builds the handler that reports the mention that was picked.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(&'static str)>>` - The select handler of the mentions
///   field.
pub(crate) fn advanced_on_mentions_select() -> Option<Rc<dyn Fn(&'static str)>> {
    Some(Rc::new(move |picked: &'static str| {
        Console::log(ADVANCED_MENTIONS_LOG.replace(ADVANCED_LOG_VALUE_PLACEHOLDER, picked));
    }))
}

/// Builds the handler that moves a row into the target list.
///
/// The transfer does not own its rows: it reports the row that was moved
/// and the page moves it between the two lists.
///
/// # Arguments
///
/// - `Signal<Vec<EuvTransferItem>>` - The rows of the source list.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(&'static str)>>` - The move handler of the transfer.
pub(crate) fn advanced_on_move_to_target(
    source: Signal<Vec<EuvTransferItem>>,
    target: Signal<Vec<EuvTransferItem>>,
) -> Option<Rc<dyn Fn(&'static str)>> {
    Some(Rc::new(move |value: &'static str| {
        let mut source_rows: Vec<EuvTransferItem> = source.get();
        let mut target_rows: Vec<EuvTransferItem> = target.get();
        if let Some(index) = source_rows
            .iter()
            .position(|row: &EuvTransferItem| row.value == value)
        {
            let row: EuvTransferItem = source_rows.remove(index);
            target_rows.push(row);
        }
        source.set(source_rows);
        target.set(target_rows);
        Console::log(ADVANCED_TRANSFER_LOG.replace(ADVANCED_LOG_VALUE_PLACEHOLDER, value));
    }))
}

/// Builds the handler that moves a row back into the source list.
///
/// # Arguments
///
/// - `Signal<Vec<EuvTransferItem>>` - The rows of the source list.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(&'static str)>>` - The move handler of the transfer.
pub(crate) fn advanced_on_move_to_source(
    source: Signal<Vec<EuvTransferItem>>,
    target: Signal<Vec<EuvTransferItem>>,
) -> Option<Rc<dyn Fn(&'static str)>> {
    Some(Rc::new(move |value: &'static str| {
        let mut source_rows: Vec<EuvTransferItem> = source.get();
        let mut target_rows: Vec<EuvTransferItem> = target.get();
        if let Some(index) = target_rows
            .iter()
            .position(|row: &EuvTransferItem| row.value == value)
        {
            let row: EuvTransferItem = target_rows.remove(index);
            source_rows.push(row);
        }
        source.set(source_rows);
        target.set(target_rows);
        Console::log(ADVANCED_TRANSFER_LOG.replace(ADVANCED_LOG_VALUE_PLACEHOLDER, value));
    }))
}
