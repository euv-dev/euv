use super::*;

/// Returns the active ambient `DynamicUpdateHub` attached to the current `Context`.
///
/// # Returns
///
/// - `DynamicUpdateHub` - Shared dynamic update hub handle.
#[must_use]
pub fn current_dynamic_hub() -> DynamicUpdateHub {
    current_context().get_dynamic_hub().clone()
}

/// Normalizes a page route path so it always starts with `'/'` and trims surrounding whitespace.
///
/// # Arguments
///
/// - `&str` - Raw route string.
///
/// # Returns
///
/// - `String` - Normalized route path, or an empty string if input was blank.
#[must_use]
pub fn normalize_dynamic_route(route: &str) -> String {
    let trimmed: &str = route.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    if trimmed.starts_with('/') {
        trimmed.to_string()
    } else {
        format!("/{trimmed}")
    }
}

/// Computes a deterministic 64-bit FNV-1a checksum over a `DynamicNodeSpec` tree across all platforms.
///
/// # Arguments
///
/// - `&DynamicNodeSpec` - Declarative dynamic node tree to digest.
///
/// # Returns
///
/// - `u64` - Deterministic non-zero checksum digest.
#[must_use]
pub fn compute_spec_checksum(spec: &DynamicNodeSpec) -> u64 {
    let mut hash: u64 = FNV1A_OFFSET_BASIS;
    hash_node_spec(spec, &mut hash);
    if hash == 0 { 1 } else { hash }
}

/// Feeds a single byte into a running 64-bit FNV-1a hash accumulator.
///
/// # Arguments
///
/// - `u8` - Input byte.
/// - `&mut u64` - Running hash state.
#[inline(always)]
fn fnv_mix_byte(byte: u8, hash: &mut u64) {
    *hash = (*hash ^ (byte as u64)).wrapping_mul(FNV1A_PRIME);
}

/// Feeds a byte slice into a running 64-bit FNV-1a hash accumulator using 8-way unrolling.
///
/// # Arguments
///
/// - `&[u8]` - Input byte slice.
/// - `&mut u64` - Running hash state.
#[inline]
fn fnv_mix_bytes(bytes: &[u8], hash: &mut u64) {
    let mut acc: u64 = *hash;
    let mut slice: &[u8] = bytes;
    while let Some((chunk, rest)) = slice.split_first_chunk::<8>() {
        acc = (acc ^ (chunk[0] as u64)).wrapping_mul(FNV1A_PRIME);
        acc = (acc ^ (chunk[1] as u64)).wrapping_mul(FNV1A_PRIME);
        acc = (acc ^ (chunk[2] as u64)).wrapping_mul(FNV1A_PRIME);
        acc = (acc ^ (chunk[3] as u64)).wrapping_mul(FNV1A_PRIME);
        acc = (acc ^ (chunk[4] as u64)).wrapping_mul(FNV1A_PRIME);
        acc = (acc ^ (chunk[5] as u64)).wrapping_mul(FNV1A_PRIME);
        acc = (acc ^ (chunk[6] as u64)).wrapping_mul(FNV1A_PRIME);
        acc = (acc ^ (chunk[7] as u64)).wrapping_mul(FNV1A_PRIME);
        slice = rest;
    }
    for byte in slice.iter().copied() {
        acc = (acc ^ (byte as u64)).wrapping_mul(FNV1A_PRIME);
    }
    *hash = acc;
}

/// Feeds an optional `f32` value deterministically into `hash`.
///
/// # Arguments
///
/// - `Option<f32>` - Optional floating-point scalar.
/// - `&mut u64` - Running hash state.
#[inline(always)]
fn fnv_mix_opt_f32(val: Option<f32>, hash: &mut u64) {
    match val {
        Some(v) => {
            fnv_mix_byte(1, hash);
            fnv_mix_bytes(&v.to_bits().to_le_bytes(), hash);
        }
        None => fnv_mix_byte(0, hash),
    }
}

/// Feeds an optional `u32` value deterministically into `hash`.
///
/// # Arguments
///
/// - `Option<u32>` - Optional unsigned 32-bit value.
/// - `&mut u64` - Running hash state.
#[inline(always)]
fn fnv_mix_opt_u32(val: Option<u32>, hash: &mut u64) {
    match val {
        Some(v) => {
            fnv_mix_byte(1, hash);
            fnv_mix_bytes(&v.to_le_bytes(), hash);
        }
        None => fnv_mix_byte(0, hash),
    }
}

/// Recursively folds a `DynamicNodeSpec` into `hash`.
///
/// # Arguments
///
/// - `&DynamicNodeSpec` - Node specification to hash.
/// - `&mut u64` - Running hash state.
fn hash_node_spec(spec: &DynamicNodeSpec, hash: &mut u64) {
    let kind_tag: u8 = match spec.get_kind() {
        DynamicNodeKind::Column => 1,
        DynamicNodeKind::Row => 2,
        DynamicNodeKind::Grid => 3,
        DynamicNodeKind::View => 4,
        DynamicNodeKind::Card => 5,
        DynamicNodeKind::Text => 6,
        DynamicNodeKind::Button => 7,
        DynamicNodeKind::Badge => 8,
        DynamicNodeKind::Divider => 9,
    };
    fnv_mix_byte(kind_tag, hash);
    fnv_mix_bytes(spec.get_text().as_bytes(), hash);
    fnv_mix_byte(0xFF, hash);
    if let Some(action) = spec.action_name() {
        fnv_mix_byte(1, hash);
        fnv_mix_bytes(action.as_bytes(), hash);
    } else {
        fnv_mix_byte(0, hash);
    }
    fnv_mix_bytes(spec.get_payload().as_bytes(), hash);
    let st: DynamicStyleSpec = spec.get_style();
    fnv_mix_opt_f32(st.width_px(), hash);
    fnv_mix_opt_f32(st.height_px(), hash);
    fnv_mix_byte(u8::from(st.get_full_width()), hash);
    fnv_mix_opt_f32(st.padding_px(), hash);
    fnv_mix_opt_f32(st.gap_px(), hash);
    fnv_mix_opt_f32(st.radius_px(), hash);
    fnv_mix_opt_f32(st.font_size_px(), hash);
    fnv_mix_byte(u8::from(st.get_bold()), hash);
    fnv_mix_opt_u32(st.background_hex_value(), hash);
    fnv_mix_opt_u32(st.color_hex_value(), hash);
    fnv_mix_opt_f32(st.border_width_px(), hash);
    fnv_mix_opt_u32(st.border_hex_value(), hash);
    fnv_mix_bytes(&st.get_columns().to_le_bytes(), hash);
    fnv_mix_bytes(&st.get_flex_grow().to_bits().to_le_bytes(), hash);
    let child_len: u32 = spec.get_children().len() as u32;
    fnv_mix_bytes(&child_len.to_le_bytes(), hash);
    for child in spec.get_children().iter() {
        hash_node_spec(child, hash);
    }
}

/// Recursively compiles a borrowed `&DynamicNodeSpec` directly into an already-locked `SceneTree` with batched child attachment.
///
/// # Arguments
///
/// - `&mut SceneTree` - Active scene tree write borrow.
/// - `&DynamicNodeSpec` - Borrowed declarative node specification.
/// - `&str` - Originating component slot ID or page route for action attribution.
/// - `&DynamicUpdateHub` - Hub receiving any interactive button `DynamicActionEvent`s.
/// - `&mut Vec<NodeId>` - Reusable child-ID scratch stack sliced per recursion frame.
///
/// # Returns
///
/// - `NodeId` - Mounted native scene node handle.
fn mount_dynamic_spec_in_tree(
    tree: &mut SceneTree,
    spec: &DynamicNodeSpec,
    source_id: &str,
    hub: &DynamicUpdateHub,
    child_stack: &mut Vec<NodeId>,
) -> NodeId {
    let st: DynamicStyleSpec = spec.get_style();
    let mark: usize = child_stack.len();
    let node_id: NodeId = match spec.get_kind() {
        DynamicNodeKind::Column => {
            let mut style: Style = Style::default();
            style.get_mut_layout().set_display(Display::Flex);
            style
                .get_mut_layout()
                .set_flex_direction(FlexDirection::Column);
            st.apply_in_place(&mut style);
            if st.gap_px().is_none() {
                let layout: &mut LayoutStyle = style.get_mut_layout();
                layout.set_row_gap(DYNAMIC_DEFAULT_CARD_GAP);
                layout.set_column_gap(DYNAMIC_DEFAULT_CARD_GAP);
            }
            tree.create_view(style)
        }
        DynamicNodeKind::Row => {
            let mut style: Style = Style::default();
            let layout: &mut LayoutStyle = style.get_mut_layout();
            layout.set_display(Display::Flex);
            layout.set_flex_direction(FlexDirection::Row);
            layout.set_flex_wrap(FlexWrap::Wrap);
            layout.set_width(Length::Percent(100.0));
            layout.set_align_items(AlignItems::Center);
            st.apply_in_place(&mut style);
            if st.gap_px().is_none() {
                let layout_mut: &mut LayoutStyle = style.get_mut_layout();
                layout_mut.set_row_gap(DYNAMIC_DEFAULT_CARD_GAP);
                layout_mut.set_column_gap(DYNAMIC_DEFAULT_CARD_GAP);
            }
            tree.create_view(style)
        }
        DynamicNodeKind::Grid => {
            let cols: u16 = if st.get_columns() > 0 {
                st.get_columns()
            } else {
                DYNAMIC_DEFAULT_GRID_COLUMNS
            };
            let mut style: Style = Style::default();
            let layout: &mut LayoutStyle = style.get_mut_layout();
            layout.set_display(Display::Grid);
            layout.set_grid_columns(cols.max(1));
            layout.set_width(Length::Percent(100.0));
            st.apply_in_place(&mut style);
            if st.gap_px().is_none() {
                let layout_mut: &mut LayoutStyle = style.get_mut_layout();
                layout_mut.set_row_gap(DYNAMIC_DEFAULT_CARD_GAP);
                layout_mut.set_column_gap(DYNAMIC_DEFAULT_CARD_GAP);
            }
            tree.create_view(style)
        }
        DynamicNodeKind::View => {
            let mut style: Style = Style::default();
            st.apply_in_place(&mut style);
            tree.create_view(style)
        }
        DynamicNodeKind::Card => {
            let mut style: Style = Style::default();
            let layout: &mut LayoutStyle = style.get_mut_layout();
            layout.set_display(Display::Flex);
            layout.set_flex_direction(FlexDirection::Column);
            layout.set_width(Length::Percent(100.0));
            layout.set_padding(Edges::all(DYNAMIC_DEFAULT_CARD_PADDING));
            layout.set_row_gap(DYNAMIC_DEFAULT_CARD_GAP);
            layout.set_column_gap(DYNAMIC_DEFAULT_CARD_GAP);
            let paint: &mut PaintStyle = style.get_mut_paint();
            paint.set_background(Some(Color::hex(DYNAMIC_CARD_BG_HEX)));
            paint.set_border(Border::new(1.0, Color::hex(DYNAMIC_CARD_BORDER_HEX)));
            paint.set_radius(CornerRadius::all(DYNAMIC_DEFAULT_CARD_RADIUS));
            st.apply_in_place(&mut style);
            let card: NodeId = tree.create_view(style);
            if !spec.get_text().is_empty() {
                let title_color: Color =
                    Color::hex(st.color_hex_value().unwrap_or(DYNAMIC_TEXT_FG_HEX));
                let mut title_style: Style = Style::default();
                let text_st: &mut TextStyle = title_style.get_mut_text();
                text_st.set_font_size(16.0);
                text_st.set_weight(FontWeight::Bold);
                text_st.set_color(title_color);
                let title_node: NodeId = tree.create_text(spec.get_text().clone(), title_style);
                child_stack.push(title_node);
            }
            card
        }
        DynamicNodeKind::Text => {
            let mut style: Style = Style::default();
            style.get_mut_layout().set_max_width(Length::Percent(100.0));
            let text_st: &mut TextStyle = style.get_mut_text();
            text_st.set_font_size(DYNAMIC_DEFAULT_FONT_SIZE);
            text_st.set_color(Color::hex(DYNAMIC_TEXT_FG_HEX));
            st.apply_in_place(&mut style);
            tree.create_text(spec.get_text().clone(), style)
        }
        DynamicNodeKind::Button => {
            let mut style: Style = Style::default();
            let layout: &mut LayoutStyle = style.get_mut_layout();
            layout.set_display(Display::Flex);
            layout.set_flex_direction(FlexDirection::Row);
            layout.set_max_width(Length::Percent(100.0));
            layout.set_overflow(Overflow::Hidden);
            layout.set_justify_content(JustifyContent::Center);
            layout.set_align_items(AlignItems::Center);
            layout.set_height(Length::Px(DYNAMIC_DEFAULT_BUTTON_HEIGHT));
            layout.set_padding(Edges::symmetric(0.0, 14.0));
            let paint: &mut PaintStyle = style.get_mut_paint();
            paint.set_background(Some(Color::hex(DYNAMIC_BUTTON_BG_HEX)));
            paint.set_radius(CornerRadius::all(DYNAMIC_DEFAULT_BUTTON_RADIUS));
            paint.set_cursor(CursorIcon::Pointer);
            let text_st: &mut TextStyle = style.get_mut_text();
            text_st.set_color(Color::hex(DYNAMIC_BUTTON_FG_HEX));
            text_st.set_font_size(13.0);
            text_st.set_weight(FontWeight::Medium);
            st.apply_in_place(&mut style);
            let label_color: Color = style.text_ref().get_color();
            let label_font_size: f32 = style.text_ref().get_font_size();
            let label_weight: FontWeight = style.text_ref().get_weight();
            let btn: NodeId = tree.create_view(style);
            let mut label_style: Style = Style::default();
            label_style
                .get_mut_layout()
                .set_max_width(Length::Percent(100.0));
            let lbl_text: &mut TextStyle = label_style.get_mut_text();
            lbl_text.set_color(label_color);
            lbl_text.set_font_size(label_font_size);
            lbl_text.set_weight(label_weight);
            lbl_text.set_align(TextAlign::Center);
            lbl_text.set_overflow(TextOverflow::Ellipsis);
            let lbl: NodeId = tree.create_text(spec.get_text().clone(), label_style);
            child_stack.push(lbl);
            if let Some(node) = tree.get_node_mut(btn) {
                node.set_a11y(Some(A11yProps::new(
                    A11yRole::Button,
                    spec.get_text().clone(),
                )));
            }
            btn
        }
        DynamicNodeKind::Badge => {
            let mut style: Style = Style::default();
            let layout: &mut LayoutStyle = style.get_mut_layout();
            layout.set_display(Display::Flex);
            layout.set_flex_direction(FlexDirection::Row);
            layout.set_align_items(AlignItems::Center);
            layout.set_padding(Edges::symmetric(4.0, 10.0));
            let paint: &mut PaintStyle = style.get_mut_paint();
            paint.set_background(Some(Color::hex(DYNAMIC_BADGE_BG_HEX)));
            paint.set_radius(CornerRadius::all(DYNAMIC_DEFAULT_BADGE_RADIUS));
            let text_st: &mut TextStyle = style.get_mut_text();
            text_st.set_color(Color::hex(DYNAMIC_BADGE_FG_HEX));
            text_st.set_font_size(12.0);
            text_st.set_weight(FontWeight::SemiBold);
            st.apply_in_place(&mut style);
            let label_color: Color = style.text_ref().get_color();
            let label_font_size: f32 = style.text_ref().get_font_size();
            let label_weight: FontWeight = style.text_ref().get_weight();
            let pill: NodeId = tree.create_view(style);
            let mut label_style: Style = Style::default();
            let lbl_text: &mut TextStyle = label_style.get_mut_text();
            lbl_text.set_color(label_color);
            lbl_text.set_font_size(label_font_size);
            lbl_text.set_weight(label_weight);
            let lbl: NodeId = tree.create_text(spec.get_text().clone(), label_style);
            child_stack.push(lbl);
            pill
        }
        DynamicNodeKind::Divider => {
            let mut style: Style = Style::default();
            let layout: &mut LayoutStyle = style.get_mut_layout();
            layout.set_width(Length::Percent(100.0));
            layout.set_height(Length::Px(1.0));
            style
                .get_mut_paint()
                .set_background(Some(Color::hex(DYNAMIC_DIVIDER_HEX)));
            st.apply_in_place(&mut style);
            tree.create_view(style)
        }
    };
    if let Some(action_name) = spec.action_name() {
        let hub_c: DynamicUpdateHub = hub.clone();
        let src: String = source_id.to_string();
        let act: String = action_name.to_string();
        let pay: String = spec.get_payload().clone();
        let handler: UiEventHandler = Rc::new(RefCell::new(move |_: &UiEvent| {
            hub_c.dispatch_action(DynamicActionEvent::new(
                src.clone(),
                act.clone(),
                pay.clone(),
            ));
        }));
        if let Some(node) = tree.get_node_mut(node_id) {
            node.get_mut_handlers().set_on_click(Some(handler));
            node.get_mut_interaction().set_focusable(true);
            let paint: &mut PaintStyle = node.get_mut_style().get_mut_paint();
            if paint.get_cursor() == CursorIcon::Default {
                paint.set_cursor(CursorIcon::Pointer);
            }
        }
    }
    let children: &[DynamicNodeSpec] = spec.get_children().as_slice();
    if !children.is_empty() {
        child_stack.reserve(children.len());
        for child in children.iter() {
            let child_id: NodeId =
                mount_dynamic_spec_in_tree(tree, child, source_id, hub, child_stack);
            child_stack.push(child_id);
        }
    }
    if child_stack.len() > mark {
        tree.append_children(node_id, &child_stack[mark..]);
        child_stack.truncate(mark);
    }
    node_id
}

/// Compiles a declarative `DynamicNodeSpec` tree directly into live native `SceneTree` nodes under a single scene write borrow.
///
/// # Arguments
///
/// - `&DynamicNodeSpec` - Declarative node specification.
/// - `&str` - Originating component slot ID or page route for action attribution.
/// - `&DynamicUpdateHub` - Hub receiving any interactive button `DynamicActionEvent`s.
///
/// # Returns
///
/// - `NodeId` - Mounted native scene node handle.
pub fn mount_dynamic_spec(
    spec: &DynamicNodeSpec,
    source_id: &str,
    hub: &DynamicUpdateHub,
) -> NodeId {
    let ctx: Context = current_context();
    let mut child_stack: Vec<NodeId> = Vec::with_capacity(spec.get_children().len().max(8));
    ctx.get_scene().with_write(
        |tree: &mut SceneTree| {
            mount_dynamic_spec_in_tree(tree, spec, source_id, hub, &mut child_stack)
        },
        NodeId::default(),
    )
}
