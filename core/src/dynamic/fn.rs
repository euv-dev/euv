use super::*;

/// Returns the active ambient `DynamicUpdateHub` attached to the current `BuildContext`.
///
/// # Returns
///
/// - `DynamicUpdateHub` - Shared dynamic update hub handle.
#[must_use]
pub fn current_dynamic_hub() -> DynamicUpdateHub {
    current_build_context().get_dynamic_hub().clone()
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

/// Feeds a byte slice into a running 64-bit FNV-1a hash accumulator.
///
/// # Arguments
///
/// - `&[u8]` - Input byte slice.
/// - `&mut u64` - Running hash state.
fn fnv_mix_bytes(bytes: &[u8], hash: &mut u64) {
    for byte in bytes.iter().copied() {
        *hash ^= byte as u64;
        *hash = hash.wrapping_mul(FNV1A_PRIME);
    }
}

/// Feeds an optional `f32` value deterministically into `hash`.
///
/// # Arguments
///
/// - `Option<f32>` - Optional floating-point scalar.
/// - `&mut u64` - Running hash state.
fn fnv_mix_opt_f32(val: Option<f32>, hash: &mut u64) {
    match val {
        Some(v) => {
            fnv_mix_bytes(&[1], hash);
            fnv_mix_bytes(&v.to_bits().to_le_bytes(), hash);
        }
        None => fnv_mix_bytes(&[0], hash),
    }
}

/// Feeds an optional `u32` value deterministically into `hash`.
///
/// # Arguments
///
/// - `Option<u32>` - Optional unsigned 32-bit value.
/// - `&mut u64` - Running hash state.
fn fnv_mix_opt_u32(val: Option<u32>, hash: &mut u64) {
    match val {
        Some(v) => {
            fnv_mix_bytes(&[1], hash);
            fnv_mix_bytes(&v.to_le_bytes(), hash);
        }
        None => fnv_mix_bytes(&[0], hash),
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
    fnv_mix_bytes(&[kind_tag], hash);
    fnv_mix_bytes(spec.get_text().as_bytes(), hash);
    fnv_mix_bytes(&[0xFF], hash);
    if let Some(action) = spec.action_name() {
        fnv_mix_bytes(&[1], hash);
        fnv_mix_bytes(action.as_bytes(), hash);
    } else {
        fnv_mix_bytes(&[0], hash);
    }
    fnv_mix_bytes(spec.get_payload().as_bytes(), hash);
    let st: DynamicStyleSpec = spec.get_style();
    fnv_mix_opt_f32(st.width_px(), hash);
    fnv_mix_opt_f32(st.height_px(), hash);
    fnv_mix_bytes(&[u8::from(st.get_full_width())], hash);
    fnv_mix_opt_f32(st.padding_px(), hash);
    fnv_mix_opt_f32(st.gap_px(), hash);
    fnv_mix_opt_f32(st.radius_px(), hash);
    fnv_mix_opt_f32(st.font_size_px(), hash);
    fnv_mix_bytes(&[u8::from(st.get_bold())], hash);
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

/// Compiles a declarative `DynamicNodeSpec` tree directly into live native `SceneTree` nodes.
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
    let mut ctx: BuildContext = current_build_context();
    let st: DynamicStyleSpec = spec.get_style();
    let node_id: NodeId = match spec.get_kind() {
        DynamicNodeKind::Column => {
            let mut style: Style = st.apply_to(Style::empty().column());
            if st.gap_px().is_none() {
                style = style.gap(DYNAMIC_DEFAULT_CARD_GAP);
            }
            ctx.view(style)
        }
        DynamicNodeKind::Row => {
            let mut style: Style = st.apply_to(Style::empty().row().align(AlignItems::Center));
            if st.gap_px().is_none() {
                style = style.gap(DYNAMIC_DEFAULT_CARD_GAP);
            }
            ctx.view(style)
        }
        DynamicNodeKind::Grid => {
            let cols: u16 = if st.get_columns() > 0 {
                st.get_columns()
            } else {
                DYNAMIC_DEFAULT_GRID_COLUMNS
            };
            let mut style: Style =
                st.apply_to(Style::empty().grid(cols).width(Length::Percent(100.0)));
            if st.gap_px().is_none() {
                style = style.gap(DYNAMIC_DEFAULT_CARD_GAP);
            }
            ctx.view(style)
        }
        DynamicNodeKind::View => ctx.view(st.apply_to(Style::empty())),
        DynamicNodeKind::Card => {
            let base: Style = Style::empty()
                .column()
                .width(Length::Percent(100.0))
                .padding(Edges::all(DYNAMIC_DEFAULT_CARD_PADDING))
                .gap(DYNAMIC_DEFAULT_CARD_GAP)
                .background(Color::hex(DYNAMIC_CARD_BG_HEX))
                .border(1.0, Color::hex(DYNAMIC_CARD_BORDER_HEX))
                .radius(DYNAMIC_DEFAULT_CARD_RADIUS);
            let card: NodeId = ctx.view(st.apply_to(base));
            if !spec.get_text().is_empty() {
                let title_color: Color =
                    Color::hex(st.color_hex_value().unwrap_or(DYNAMIC_TEXT_FG_HEX));
                let title_node: NodeId = ctx.text(
                    spec.get_text().clone(),
                    Style::empty()
                        .font_size(16.0)
                        .font_weight(FontWeight::Bold)
                        .color(title_color),
                );
                ctx.append(card, title_node);
            }
            card
        }
        DynamicNodeKind::Text => {
            let base: Style = Style::empty()
                .font_size(DYNAMIC_DEFAULT_FONT_SIZE)
                .color(Color::hex(DYNAMIC_TEXT_FG_HEX));
            ctx.text(spec.get_text().clone(), st.apply_to(base))
        }
        DynamicNodeKind::Button => {
            let base: Style = Style::empty()
                .row()
                .justify(JustifyContent::Center)
                .align(AlignItems::Center)
                .height(Length::Px(DYNAMIC_DEFAULT_BUTTON_HEIGHT))
                .padding(Edges::symmetric(0.0, 14.0))
                .background(Color::hex(DYNAMIC_BUTTON_BG_HEX))
                .color(Color::hex(DYNAMIC_BUTTON_FG_HEX))
                .radius(DYNAMIC_DEFAULT_BUTTON_RADIUS)
                .font_size(13.0)
                .font_weight(FontWeight::Medium)
                .cursor(CursorIcon::Pointer);
            let resolved: Style = st.apply_to(base);
            let btn: NodeId = ctx.view(resolved);
            let label_style: Style = Style::empty()
                .color(resolved.get_text().get_color())
                .font_size(resolved.get_text().get_font_size())
                .font_weight(resolved.get_text().get_weight())
                .text_align(TextAlign::Center);
            let lbl: NodeId = ctx.text(spec.get_text().clone(), label_style);
            ctx.append(btn, lbl);
            ctx.set_a11y(
                btn,
                A11yProps::new(A11yRole::Button, spec.get_text().clone()),
            );
            btn
        }
        DynamicNodeKind::Badge => {
            let base: Style = Style::empty()
                .row()
                .align(AlignItems::Center)
                .padding(Edges::symmetric(4.0, 10.0))
                .background(Color::hex(DYNAMIC_BADGE_BG_HEX))
                .color(Color::hex(DYNAMIC_BADGE_FG_HEX))
                .radius(DYNAMIC_DEFAULT_BADGE_RADIUS)
                .font_size(12.0)
                .font_weight(FontWeight::SemiBold);
            let resolved: Style = st.apply_to(base);
            let pill: NodeId = ctx.view(resolved);
            let lbl: NodeId = ctx.text(
                spec.get_text().clone(),
                Style::empty()
                    .color(resolved.get_text().get_color())
                    .font_size(resolved.get_text().get_font_size())
                    .font_weight(resolved.get_text().get_weight()),
            );
            ctx.append(pill, lbl);
            pill
        }
        DynamicNodeKind::Divider => {
            let base: Style = Style::empty()
                .width(Length::Percent(100.0))
                .height(Length::Px(1.0))
                .background(Color::hex(DYNAMIC_DIVIDER_HEX));
            ctx.view(st.apply_to(base))
        }
    };
    if let Some(action_name) = spec.action_name() {
        let hub_c: DynamicUpdateHub = hub.clone();
        let src: String = source_id.to_string();
        let act: String = action_name.to_string();
        let pay: String = spec.get_payload().clone();
        ctx.on_click(node_id, move |_: &UiEvent| {
            hub_c.dispatch_action(DynamicActionEvent::new(
                src.clone(),
                act.clone(),
                pay.clone(),
            ));
        });
    }
    for child in spec.get_children().iter() {
        let child_id: NodeId = mount_dynamic_spec(child, source_id, hub);
        ctx.append(node_id, child_id);
    }
    node_id
}
