use super::*;

/// Returns the thread-local reactive generation signal notified whenever the ambient theme provider changes.
///
/// # Returns
///
/// - `Signal<u64>` - Live generation counter signal in the active reactive runtime.
fn theme_provider_signal() -> Signal<u64> {
    ACTIVE_THEME_GEN.with(|slot: &RefCell<Option<Signal<u64>>>| {
        if let Ok(guard) = slot.try_borrow()
            && let Some(sig) = *guard
            && untracked(|| sig.try_get().is_some())
        {
            return sig;
        }
        let fresh: Signal<u64> = Signal::new(0);
        if let Ok(mut guard) = slot.try_borrow_mut() {
            *guard = Some(fresh);
        }
        fresh
    })
}

/// Registers the active application `UiThemeState` for reactive in-place theme updates across components.
///
/// # Arguments
///
/// - `UiThemeState` - Reactive theme controller to install as the ambient theme.
pub fn provide_theme(theme: UiThemeState) {
    ACTIVE_THEME_STATE.with(|slot: &RefCell<Option<UiThemeState>>| {
        if let Ok(mut guard) = slot.try_borrow_mut() {
            *guard = Some(theme);
        }
    });
    let gen_sig: Signal<u64> = theme_provider_signal();
    let next_gen: u64 = gen_sig.get_untracked().wrapping_add(1);
    let _: bool = gen_sig.set(next_gen);
}

/// Clears any ambient `UiThemeState` registered on the current thread.
pub fn clear_provided_theme() {
    ACTIVE_THEME_STATE.with(|slot: &RefCell<Option<UiThemeState>>| {
        if let Ok(mut guard) = slot.try_borrow_mut() {
            *guard = None;
        }
    });
    ACTIVE_THEME_GEN.with(|slot: &RefCell<Option<Signal<u64>>>| {
        if let Ok(guard) = slot.try_borrow()
            && let Some(sig) = *guard
            && untracked(|| sig.try_get().is_some())
        {
            let next_gen: u64 = sig.get_untracked().wrapping_add(1);
            let _: bool = sig.set(next_gen);
        }
    });
}

/// Returns the ambient `UiThemeState` if one is active and its reactive signals are still alive.
///
/// # Returns
///
/// - `Option<UiThemeState>` - Active reactive theme controller, or `None` when none is provided.
#[must_use]
pub fn active_theme_state() -> Option<UiThemeState> {
    ACTIVE_THEME_STATE.with(|slot: &RefCell<Option<UiThemeState>>| {
        let candidate: Option<UiThemeState> = slot
            .try_borrow()
            .ok()
            .and_then(|guard: Ref<'_, Option<UiThemeState>>| guard.clone());
        candidate
            .filter(|theme: &UiThemeState| untracked(|| theme.get_tokens().try_get().is_some()))
    })
}

/// Resolves live `ThemeTokens` reactively from the ambient `UiThemeState`, falling back to `fallback`.
///
/// # Arguments
///
/// - `ThemeTokens` - Static fallback tokens used when no ambient theme is provided.
///
/// # Returns
///
/// - `ThemeTokens` - Current theme tokens (subscribing the active reactive observer when ambient theme exists).
#[must_use]
pub fn use_live_tokens(fallback: ThemeTokens) -> ThemeTokens {
    let _: u64 = theme_provider_signal().get();
    match active_theme_state() {
        Some(theme) => theme.get_tokens().try_get().unwrap_or(fallback),
        None => fallback,
    }
}

/// Resolves live `ThemeTokens` without subscribing reactive observers (suitable for canvas painters and handlers).
///
/// # Arguments
///
/// - `ThemeTokens` - Static fallback tokens used when no ambient theme is provided.
///
/// # Returns
///
/// - `ThemeTokens` - Current theme tokens read without reactive tracking.
#[must_use]
pub fn live_tokens_untracked(fallback: ThemeTokens) -> ThemeTokens {
    match active_theme_state() {
        Some(theme) => untracked(|| theme.get_tokens().try_get()).unwrap_or(fallback),
        None => fallback,
    }
}

/// Remaps one RGB-matched color entry from `pairs` while preserving `color`'s alpha channel.
///
/// # Arguments
///
/// - `Color` - Existing color value.
/// - `&[(Color, Color)]` - Ordered `(old_color, new_color)` semantic lookup pairs.
///
/// # Returns
///
/// - `Option<Color>` - Remapped color preserving original alpha if matched.
fn remap_color_from_pairs(color: Color, pairs: &[(Color, Color)]) -> Option<Color> {
    if color.get_a() == 0 {
        return None;
    }
    let r: u8 = color.get_r();
    let g: u8 = color.get_g();
    let b: u8 = color.get_b();
    let a: u8 = color.get_a();
    for &(old_c, new_c) in pairs.iter() {
        if old_c.get_r() == r && old_c.get_g() == g && old_c.get_b() == b {
            let next_a: u8 = if old_c.get_a() == a && old_c.get_a() != 255 {
                new_c.get_a()
            } else {
                a
            };
            return Some(Color::rgba(
                new_c.get_r(),
                new_c.get_g(),
                new_c.get_b(),
                next_a,
            ));
        }
    }
    None
}

/// Remaps a single scalar corner radius from `old_g` to `new_g` when matching a design-token step.
///
/// # Arguments
///
/// - `f32` - Existing corner radius in logical pixels.
/// - `ThemeGeometry` - Previous theme geometry tokens.
/// - `ThemeGeometry` - Next theme geometry tokens.
///
/// # Returns
///
/// - `f32` - Updated corner radius.
fn remap_scalar_radius(value: f32, old_g: ThemeGeometry, new_g: ThemeGeometry) -> f32 {
    if (value - old_g.get_radius_xl()).abs() < 0.25 {
        new_g.get_radius_xl()
    } else if (value - old_g.get_radius_lg()).abs() < 0.25 {
        new_g.get_radius_lg()
    } else if (value - old_g.get_radius_md()).abs() < 0.25 {
        new_g.get_radius_md()
    } else if (value - old_g.get_radius_sm()).abs() < 0.25 {
        new_g.get_radius_sm()
    } else {
        value
    }
}

/// Remaps a `Shadow` matching one of `old_g`'s elevation levels onto `new_g`.
///
/// # Arguments
///
/// - `Shadow` - Existing shadow specification.
/// - `ThemeGeometry` - Previous theme geometry tokens.
/// - `ThemeGeometry` - Next theme geometry tokens.
///
/// # Returns
///
/// - `Shadow` - Updated shadow specification.
fn remap_theme_shadow(shadow: Shadow, old_g: ThemeGeometry, new_g: ThemeGeometry) -> Shadow {
    if shadow == old_g.get_shadow_lg() {
        new_g.get_shadow_lg()
    } else if shadow == old_g.get_shadow_md() {
        new_g.get_shadow_md()
    } else if shadow == old_g.get_shadow_sm() {
        new_g.get_shadow_sm()
    } else if shadow == old_g.get_shadow_xs() {
        new_g.get_shadow_xs()
    } else {
        shadow
    }
}

/// Remaps colors, borders, radii, and shadows in-place across the scene subtree rooted at `root` without tearing down nodes or resetting scroll positions.
///
/// # Arguments
///
/// - `&mut SceneTree` - Active retained scene tree.
/// - `NodeId` - Root node handle of the subtree to restyle in-place.
/// - `ThemeTokens` - Previously applied theme tokens.
/// - `ThemeTokens` - Newly active theme tokens.
pub fn remap_subtree_theme_in_place(
    tree: &mut SceneTree,
    root: NodeId,
    old_tokens: ThemeTokens,
    new_tokens: ThemeTokens,
) {
    remap_subtree_theme_excluding(tree, root, None, old_tokens, new_tokens);
}

/// Remaps colors, borders, radii, and shadows in-place across the scene subtree rooted at `root`, optionally skipping an independently managed child subtree `exclude`.
///
/// # Arguments
///
/// - `&mut SceneTree` - Active retained scene tree.
/// - `NodeId` - Root node handle of the subtree to restyle in-place.
/// - `Option<NodeId>` - Optional descendant root handle excluded from traversal.
/// - `ThemeTokens` - Previously applied theme tokens.
/// - `ThemeTokens` - Newly active theme tokens.
pub fn remap_subtree_theme_excluding(
    tree: &mut SceneTree,
    root: NodeId,
    exclude: Option<NodeId>,
    old_tokens: ThemeTokens,
    new_tokens: ThemeTokens,
) {
    let excluded: &[NodeId] = exclude.as_slice();
    remap_subtree_theme_excluding_nodes(tree, root, excluded, old_tokens, new_tokens);
}

/// Remaps colors, borders, radii, and shadows in-place across the scene subtree rooted at `root`, skipping every descendant root listed in `excluded`.
///
/// # Arguments
///
/// - `&mut SceneTree` - Active retained scene tree.
/// - `NodeId` - Root node handle of the subtree to restyle in-place.
/// - `&[NodeId]` - Descendant root handles excluded from traversal.
/// - `ThemeTokens` - Previously applied theme tokens.
/// - `ThemeTokens` - Newly active theme tokens.
pub(crate) fn remap_subtree_theme_excluding_nodes(
    tree: &mut SceneTree,
    root: NodeId,
    excluded: &[NodeId],
    old_tokens: ThemeTokens,
    new_tokens: ThemeTokens,
) {
    if old_tokens == new_tokens {
        return;
    }
    let op: ThemePalette = old_tokens.get_palette();
    let np: ThemePalette = new_tokens.get_palette();
    let og: ThemeGeometry = old_tokens.get_geometry();
    let ng: ThemeGeometry = new_tokens.get_geometry();
    let oe: CodeThemePalette = CodeThemePalette::resolve(old_tokens.get_mode());
    let ne: CodeThemePalette = CodeThemePalette::resolve(new_tokens.get_mode());
    let surface_pairs: [(Color, Color); 25] = [
        (op.get_card(), np.get_card()),
        (op.get_background(), np.get_background()),
        (op.get_muted(), np.get_muted()),
        (op.get_secondary(), np.get_secondary()),
        (op.get_accent(), np.get_accent()),
        (op.get_primary(), np.get_primary()),
        (op.get_destructive(), np.get_destructive()),
        (op.get_success(), np.get_success()),
        (op.get_warning(), np.get_warning()),
        (op.get_overlay(), np.get_overlay()),
        (op.get_switch_track_off(), np.get_switch_track_off()),
        (op.get_switch_thumb(), np.get_switch_thumb()),
        (op.get_media_surface(), np.get_media_surface()),
        (op.get_border(), np.get_border()),
        (op.get_foreground(), np.get_foreground()),
        (oe.get_background(), ne.get_background()),
        (oe.get_header_bg(), ne.get_header_bg()),
        (oe.get_line_highlight(), ne.get_line_highlight()),
        (oe.get_inline_code_bg(), ne.get_inline_code_bg()),
        (oe.get_quote_bg(), ne.get_quote_bg()),
        (oe.get_border(), ne.get_border()),
        (oe.get_quote_border(), ne.get_quote_border()),
        (oe.get_keyword(), ne.get_keyword()),
        (oe.get_link(), ne.get_link()),
        (oe.get_diff_add(), ne.get_diff_add()),
    ];
    let border_pairs: [(Color, Color); 15] = [
        (op.get_border(), np.get_border()),
        (oe.get_border(), ne.get_border()),
        (op.get_primary(), np.get_primary()),
        (op.get_ring(), np.get_ring()),
        (oe.get_quote_border(), ne.get_quote_border()),
        (op.get_destructive(), np.get_destructive()),
        (op.get_success(), np.get_success()),
        (op.get_warning(), np.get_warning()),
        (oe.get_link(), ne.get_link()),
        (oe.get_diff_add(), ne.get_diff_add()),
        (oe.get_diff_del(), ne.get_diff_del()),
        (oe.get_keyword(), ne.get_keyword()),
        (op.get_accent(), np.get_accent()),
        (op.get_muted(), np.get_muted()),
        (op.get_foreground(), np.get_foreground()),
    ];
    let text_pairs: [(Color, Color); 31] = [
        (op.get_foreground(), np.get_foreground()),
        (op.get_card_foreground(), np.get_card_foreground()),
        (op.get_muted_foreground(), np.get_muted_foreground()),
        (op.get_primary_foreground(), np.get_primary_foreground()),
        (op.get_primary(), np.get_primary()),
        (op.get_secondary_foreground(), np.get_secondary_foreground()),
        (
            op.get_destructive_foreground(),
            np.get_destructive_foreground(),
        ),
        (op.get_destructive(), np.get_destructive()),
        (op.get_success_foreground(), np.get_success_foreground()),
        (op.get_success(), np.get_success()),
        (op.get_warning_foreground(), np.get_warning_foreground()),
        (op.get_warning(), np.get_warning()),
        (op.get_background(), np.get_background()),
        (oe.get_foreground(), ne.get_foreground()),
        (oe.get_line_number(), ne.get_line_number()),
        (oe.get_comment(), ne.get_comment()),
        (oe.get_keyword(), ne.get_keyword()),
        (oe.get_control(), ne.get_control()),
        (oe.get_operator(), ne.get_operator()),
        (oe.get_string(), ne.get_string()),
        (oe.get_number(), ne.get_number()),
        (oe.get_constant(), ne.get_constant()),
        (oe.get_function(), ne.get_function()),
        (oe.get_ty(), ne.get_ty()),
        (oe.get_primitive(), ne.get_primitive()),
        (oe.get_property(), ne.get_property()),
        (oe.get_punctuation(), ne.get_punctuation()),
        (oe.get_inline_code_fg(), ne.get_inline_code_fg()),
        (oe.get_link(), ne.get_link()),
        (oe.get_diff_add(), ne.get_diff_add()),
        (oe.get_diff_del(), ne.get_diff_del()),
    ];
    let mut stack: Vec<NodeId> = vec![root];
    while let Some(nid) = stack.pop() {
        if excluded.contains(&nid) {
            continue;
        }
        let children: Vec<NodeId> = match tree.get_node(nid) {
            Some(node) if node.get_alive() => node.get_children().clone(),
            _ => continue,
        };
        stack.extend(children);
        let mut paint_dirty: bool = false;
        if let Some(node) = tree.get_node_mut(nid) {
            let mut st: Style = node.get_style();
            let mut changed: bool = false;
            if let Some(bg) = st.get_paint().background_color()
                && let Some(next_bg) = remap_color_from_pairs(bg, &surface_pairs)
                && next_bg != bg
            {
                st.get_mut_paint().set_background(Some(next_bg));
                changed = true;
            }
            let border: Border = st.get_paint().get_border();
            if border.get_width() > 0.0
                && let Some(next_bc) = remap_color_from_pairs(border.get_color(), &border_pairs)
                && next_bc != border.get_color()
            {
                st.get_mut_paint()
                    .set_border(Border::new(border.get_width(), next_bc));
                changed = true;
            }
            let rad: CornerRadius = st.get_paint().get_radius();
            let next_rad: CornerRadius = CornerRadius::new(
                remap_scalar_radius(rad.get_top_left(), og, ng),
                remap_scalar_radius(rad.get_top_right(), og, ng),
                remap_scalar_radius(rad.get_bottom_right(), og, ng),
                remap_scalar_radius(rad.get_bottom_left(), og, ng),
            );
            if next_rad != rad {
                st.get_mut_paint().set_radius(next_rad);
                changed = true;
            }
            if let Some(sh) = st.get_paint().box_shadow() {
                let next_sh: Shadow = remap_theme_shadow(sh, og, ng);
                if next_sh != sh {
                    st.get_mut_paint().set_shadow(Some(next_sh));
                    changed = true;
                }
            }
            let tc: Color = st.get_text().get_color();
            if let Some(next_tc) = remap_color_from_pairs(tc, &text_pairs)
                && next_tc != tc
            {
                st.get_mut_text().set_color(next_tc);
                changed = true;
            }
            if matches!(node.get_kind(), NodeKind::Canvas) {
                paint_dirty = true;
            }
            if changed {
                node.set_style(st);
                paint_dirty = true;
            }
        }
        if paint_dirty {
            tree.mark_dirty(nid, DIRTY_PAINT);
        }
    }
    tree.mark_dirty(root, DIRTY_PAINT);
}
