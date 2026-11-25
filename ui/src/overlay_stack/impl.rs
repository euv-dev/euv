use super::*;

/// Unified overlay stack management operations for `EuvOverlayStack`.
impl EuvOverlayStack {
    /// Registers an open overlay onto the unified dismissal stack and returns its unique entry ID.
    ///
    /// # Arguments
    ///
    /// - `&str` - Human-readable overlay label.
    /// - `i32` - Layer stacking z-index (`Z_INDEX_MODAL`, `Z_INDEX_DRAWER`, etc.).
    /// - `F` - Closure closing the overlay when dismissed by system back.
    ///
    /// # Returns
    ///
    /// - `u64` - Unique registration ID used to remove the entry upon manual dismissal.
    pub fn push<F>(label: &str, z_index: i32, on_dismiss: F) -> u64
    where
        F: FnMut() + 'static,
    {
        let id: u64 = OVERLAY_STACK_NEXT_ID.with(|seq: &Cell<u64>| {
            let cur: u64 = seq.get();
            seq.set(cur.wrapping_add(1).max(1));
            cur
        });
        let entry: OverlayStackEntry = OverlayStackEntry {
            id,
            label: label.to_string(),
            z_index,
            on_dismiss: Rc::new(RefCell::new(on_dismiss)),
        };
        OVERLAY_STACK_ENTRIES.with(|stack: &RefCell<Vec<OverlayStackEntry>>| {
            if let Ok(mut list) = stack.try_borrow_mut() {
                list.push(entry);
            }
        });
        id
    }

    /// Removes an overlay entry by `id` when closed outside the back-stack path (e.g. close button or scrim tap).
    ///
    /// # Arguments
    ///
    /// - `u64` - Registration ID returned by `push`.
    pub fn remove(id: u64) {
        if id == 0 {
            return;
        }
        OVERLAY_STACK_ENTRIES.with(|stack: &RefCell<Vec<OverlayStackEntry>>| {
            if let Ok(mut list) = stack.try_borrow_mut() {
                list.retain(|entry: &OverlayStackEntry| entry.get_id() != id);
            }
        });
    }

    /// Dismisses ONLY the highest-priority (`z_index`, then most recent `id`) active overlay in LIFO order.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if an active overlay was dismissed; `false` if the overlay stack is empty.
    pub fn dismiss_top() -> bool {
        let top_cb: Option<OverlayDismissCallback> =
            OVERLAY_STACK_ENTRIES.with(|stack: &RefCell<Vec<OverlayStackEntry>>| {
                let Ok(mut list) = stack.try_borrow_mut() else {
                    return None;
                };
                if list.is_empty() {
                    return None;
                }
                let mut best_idx: usize = 0;
                for i in 1..list.len() {
                    let a: &OverlayStackEntry = &list[best_idx];
                    let b: &OverlayStackEntry = &list[i];
                    if b.get_z_index() > a.get_z_index()
                        || (b.get_z_index() == a.get_z_index() && b.get_id() > a.get_id())
                    {
                        best_idx = i;
                    }
                }
                let removed: OverlayStackEntry = list.remove(best_idx);
                let OverlayStackEntry { on_dismiss, .. } = removed;
                Some(on_dismiss)
            });
        if let Some(cb) = top_cb {
            if let Ok(mut func) = cb.try_borrow_mut() {
                func();
            }
            return true;
        }
        false
    }

    /// Dismisses every active overlay currently open in top-to-bottom LIFO order.
    ///
    /// # Returns
    ///
    /// - `usize` - Number of overlays dismissed.
    pub fn dismiss_all() -> usize {
        let mut closed: usize = 0;
        while Self::dismiss_top() {
            closed += 1;
        }
        closed
    }

    /// Returns the number of currently open overlays registered in the stack.
    ///
    /// # Returns
    ///
    /// - `usize` - Active overlay depth.
    #[must_use]
    pub fn active_count() -> usize {
        OVERLAY_STACK_ENTRIES.with(|stack: &RefCell<Vec<OverlayStackEntry>>| {
            stack
                .try_borrow()
                .map(|list: Ref<'_, Vec<OverlayStackEntry>>| list.len())
                .unwrap_or(0)
        })
    }

    /// Returns the ordered labels of all currently open overlays (bottom to top).
    ///
    /// # Returns
    ///
    /// - `Vec<String>` - Active overlay labels.
    #[must_use]
    pub fn active_labels() -> Vec<String> {
        OVERLAY_STACK_ENTRIES.with(|stack: &RefCell<Vec<OverlayStackEntry>>| {
            stack
                .try_borrow()
                .map(|list: Ref<'_, Vec<OverlayStackEntry>>| {
                    list.iter()
                        .map(|entry: &OverlayStackEntry| entry.get_label().clone())
                        .collect()
                })
                .unwrap_or_default()
        })
    }

    /// Clears all registered entries without invoking callbacks (useful for test reset).
    pub fn clear() {
        OVERLAY_STACK_ENTRIES.with(|stack: &RefCell<Vec<OverlayStackEntry>>| {
            if let Ok(mut list) = stack.try_borrow_mut() {
                list.clear();
            }
        });
    }

    /// Binds a boolean open signal (`EuvModal`, `EuvDrawer`, `EuvSheet`, `EuvAlertDialog`, etc.) to the unified overlay stack.
    ///
    /// # Arguments
    ///
    /// - `Signal<bool>` - Controlled overlay visibility signal.
    /// - `&str` - Diagnostic overlay label.
    /// - `i32` - Layer z-index priority.
    ///
    /// # Returns
    ///
    /// - `Effect` - Reactive effect keeping the stack synchronized with `open`.
    pub fn bind_signal(open: Signal<bool>, label: &str, z_index: i32) -> Effect {
        let entry_id: Rc<Cell<u64>> = Rc::new(Cell::new(0));
        let label_owned: String = label.to_string();
        Effect::new(move || {
            let is_open: bool = open.get();
            let cur_id: u64 = entry_id.get();
            if is_open && cur_id == 0 {
                let id_holder: Rc<Cell<u64>> = Rc::clone(&entry_id);
                let new_id: u64 = Self::push(&label_owned, z_index, move || {
                    id_holder.set(0);
                    let _: bool = open.set_if_changed(false);
                });
                entry_id.set(new_id);
            } else if !is_open && cur_id != 0 {
                entry_id.set(0);
                Self::remove(cur_id);
            }
        })
    }
}
