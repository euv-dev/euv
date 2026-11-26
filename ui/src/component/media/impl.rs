use super::*;

/// Constructors and fluent builders for `EuvImage`.
impl EuvImage {
    /// Creates a new `EuvImage` with sensible defaults (`MediaFit::Contain`, `160px` height).
    ///
    /// # Arguments
    ///
    /// - `ImageAsset` - Decoded image asset.
    /// - `ThemeTokens` - Active theme tokens.
    ///
    /// # Returns
    ///
    /// - `Self` - Configured `EuvImage`.
    #[must_use]
    pub fn new(asset: ImageAsset, tokens: ThemeTokens) -> Self {
        let cap: String = asset.get_alt_text().clone();
        Self {
            asset,
            fit: MediaFit::Contain,
            height_px: 160.0,
            caption: cap,
            tokens,
        }
    }

    /// Overrides the viewport height in logical pixels.
    ///
    /// # Arguments
    ///
    /// - `Self` - Builder value.
    /// - `f32` - Desired height in pixels.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated `EuvImage`.
    #[must_use]
    pub fn height(mut self, px: f32) -> Self {
        self.set_height_px(px.max(48.0));
        self
    }

    /// Sets the `MediaFit` scaling strategy.
    ///
    /// # Arguments
    ///
    /// - `Self` - Builder value.
    /// - `MediaFit` - Desired box fit mode.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated `EuvImage`.
    #[must_use]
    pub fn with_fit(mut self, fit: MediaFit) -> Self {
        self.set_fit(fit);
        self
    }
}

/// Declarative `#[component]` implementation for `EuvImage`.
#[component]
/// Inherent implementation of [`EuvImage`].
impl EuvImage {
    /// Renders `EuvImage` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let h_px: f32 = self.get_height_px();
        let format_lbl: String = self.get_asset().get_metadata().get_format_label().clone();
        let img_buf: ImageBuffer = self.get_asset().get_buffer().clone();
        let caption: String = self.get_caption().clone();
        let aspect: f32 =
            (img_buf.get_width().max(1) as f32) / (img_buf.get_height().max(1) as f32);
        let fit_w: f32 = (h_px * aspect).round().max(48.0);
        view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_SM
                padding: Edges::all(SPACE_MD)
                background: p.get_card()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_lg()
                row {
                    flex_wrap: FlexWrap::Wrap
                    width: Length::Percent(100.0)
                    justify: JustifyContent::SpaceBetween
                    align: AlignItems::Center
                    gap: SPACE_SM
                    text {
                        font_size: g.get_font_body()
                        font_weight: FontWeight::SemiBold
                        color: p.get_card_foreground()
                        { caption }
                    }
                    EuvBadge(format_lbl, EuvTone::Primary, tokens)
                }
                row {
                    width: Length::Percent(100.0)
                    height: Length::Px(h_px)
                    justify: JustifyContent::Center
                    align: AlignItems::Center
                    background: p.get_media_surface()
                    radius: g.get_radius_md()
                    border: (g.get_border_width(), p.get_border())
                    overflow: Overflow::Hidden
                    image {
                        source: img_buf
                        width: Length::Px(fit_w)
                        max_width: Length::Percent(100.0)
                        height: Length::Px(h_px)
                    }
                }
            }
        }
    }
}

/// Constructors for `EuvSvgView`.
impl EuvSvgView {
    /// Creates a native `EuvSvgView` for `document`.
    ///
    /// # Arguments
    ///
    /// - `SvgDocument` - Parsed and rasterized SVG vector asset.
    /// - `String` - Display label.
    /// - `ThemeTokens` - Active theme tokens.
    ///
    /// # Returns
    ///
    /// - `Self` - Configured `EuvSvgView`.
    #[must_use]
    pub fn new(document: SvgDocument, label: String, tokens: ThemeTokens) -> Self {
        Self {
            document,
            height_px: 160.0,
            label,
            tokens,
        }
    }
}

/// Declarative `#[component]` implementation for `EuvSvgView`.
#[component]
/// Inherent implementation of [`EuvSvgView`].
impl EuvSvgView {
    /// Renders `EuvSvgView` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let tokens: ThemeTokens = self.get_tokens();
        let p: ThemePalette = tokens.get_palette();
        let g: ThemeGeometry = tokens.get_geometry();
        let badge_txt: String = format!(
            "SVG · {}×{} · {} prims",
            self.get_document().get_view_box_width() as u32,
            self.get_document().get_view_box_height() as u32,
            self.get_document().get_primitive_count()
        );
        let label: String = self.get_label().clone();
        let svg_buf: ImageBuffer = self.get_document().get_rasterized().clone();
        let svg_h: f32 = self.get_height_px();
        let svg_aspect: f32 =
            (svg_buf.get_width().max(1) as f32) / (svg_buf.get_height().max(1) as f32);
        let svg_fit_w: f32 = (svg_h * svg_aspect).round().max(48.0);
        view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_SM
                padding: Edges::all(SPACE_MD)
                background: p.get_card()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_lg()
                row {
                    flex_wrap: FlexWrap::Wrap
                    width: Length::Percent(100.0)
                    justify: JustifyContent::SpaceBetween
                    align: AlignItems::Center
                    gap: SPACE_SM
                    text {
                        font_size: g.get_font_body()
                        font_weight: FontWeight::SemiBold
                        color: p.get_card_foreground()
                        { label }
                    }
                    EuvBadge(badge_txt, EuvTone::Primary, tokens)
                }
                row {
                    width: Length::Percent(100.0)
                    height: Length::Px(svg_h)
                    justify: JustifyContent::Center
                    align: AlignItems::Center
                    background: p.get_media_surface()
                    radius: g.get_radius_md()
                    border: (g.get_border_width(), p.get_border())
                    overflow: Overflow::Hidden
                    image {
                        source: svg_buf
                        width: Length::Px(svg_fit_w)
                        max_width: Length::Percent(100.0)
                        height: Length::Px(svg_h)
                    }
                }
            }
        }
    }
}

/// Constructors for `EuvGifPlayer`.
impl EuvGifPlayer {
    /// Creates an interactive animated GIF player bound to `state`.
    ///
    /// # Arguments
    ///
    /// - `Signal<GifAnimation>` - Reactive GIF animation signal.
    /// - `ThemeTokens` - Active theme tokens.
    ///
    /// # Returns
    ///
    /// - `Self` - Configured `EuvGifPlayer`.
    #[must_use]
    pub fn new(state: Signal<GifAnimation>, tokens: ThemeTokens) -> Self {
        Self {
            state,
            height_px: 150.0,
            tokens,
        }
    }
}

/// Declarative `#[component]` implementation for `EuvGifPlayer`.
#[component]
/// Inherent implementation of [`EuvGifPlayer`].
impl EuvGifPlayer {
    /// Renders `EuvGifPlayer` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let sig: Signal<GifAnimation> = self.get_state();
        let h_px: f32 = self.get_height_px();
        let tokens: ThemeTokens = self.get_tokens();
        view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_SM
                padding: Edges::all(SPACE_MD)
                background: p.get_card()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_lg()
                bind_children: move || {
                    let anim: GifAnimation = sig.get();
                    let total_f: usize = anim.get_frames().len().max(1);
                    let cur_f: usize = (anim.get_current_frame() % total_f) + 1;
                    let is_playing: bool = anim.get_playing();
                    let gif_buf: ImageBuffer = anim.active_image();
                    let gif_aspect: f32 = (gif_buf.get_width().max(1) as f32)
                    / (gif_buf.get_height().max(1) as f32);
                    let gif_fit_w: f32 = (h_px * gif_aspect).round().max(48.0);
                    vec![
                        view! {
                            row {
                                flex_wrap: FlexWrap::Wrap
                                width: Length::Percent(100.0)
                                justify: JustifyContent::SpaceBetween
                                align: AlignItems::Center
                                gap: SPACE_SM
                                text {
                                    font_size: g.get_font_body()
                                    font_weight: FontWeight::SemiBold
                                    color: p.get_card_foreground()
                                    { anim.get_metadata().get_title().clone() }
                                }
                                EuvBadge(
                                    format!("GIF · Frame {cur_f}/{total_f}"),
                                    if is_playing { EuvTone::Success } else { EuvTone::Neutral },
                                    tokens,
                                )
                            }
                        },
                        view! {
                            row {
                                width: Length::Percent(100.0)
                                height: Length::Px(h_px)
                                justify: JustifyContent::Center
                                align: AlignItems::Center
                                background: p.get_media_surface()
                                radius: g.get_radius_md()
                                border: (g.get_border_width(), p.get_border())
                                overflow: Overflow::Hidden
                                image {
                                    source: gif_buf
                                    width: Length::Px(gif_fit_w)
                                    max_width: Length::Percent(100.0)
                                    height: Length::Px(h_px)
                                }
                            }
                        },
                        view! {
                            EuvActionRow() {
                                EuvButton(
                                    if is_playing {
                                        LABEL_PAUSE_GIF.to_string()
                                    } else {
                                        LABEL_PLAY_GIF.to_string()
                                    },
                                    tokens,
                                ) {
                                    sm()
                                    on_click: move |_: &UiEvent| {
                                        let mut next: GifAnimation = sig.get_untracked();
                                        let cur_p: bool = next.get_playing();
                                        next.set_playing(!cur_p);
                                        let _: bool = sig.set(next);
                                    }
                                }
                                EuvButton(LABEL_NEXT_FRAME.to_string(), tokens) {
                                    secondary()
                                    sm()
                                    on_click: move |_: &UiEvent| {
                                        let mut next: GifAnimation = sig.get_untracked();
                                        next.next_frame();
                                        let _: bool = sig.set(next);
                                    }
                                }
                            }
                        },
                    ]
                }
            }
        }
    }
}

/// Constructors for `EuvLivePhoto`.
impl EuvLivePhoto {
    /// Creates an interactive LIVE Photo component bound to `state`.
    ///
    /// # Arguments
    ///
    /// - `Signal<LivePhotoAsset>` - Reactive LIVE Photo signal.
    /// - `ThemeTokens` - Active theme tokens.
    ///
    /// # Returns
    ///
    /// - `Self` - Configured `EuvLivePhoto`.
    #[must_use]
    pub fn new(state: Signal<LivePhotoAsset>, tokens: ThemeTokens) -> Self {
        Self {
            state,
            height_px: 150.0,
            tokens,
        }
    }
}

/// Declarative `#[component]` implementation for `EuvLivePhoto`.
#[component]
/// Inherent implementation of [`EuvLivePhoto`].
impl EuvLivePhoto {
    /// Renders `EuvLivePhoto` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let ctx: Context = current_context();
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let sig: Signal<LivePhotoAsset> = self.get_state();
        let h_px: f32 = self.get_height_px();
        let tokens: ThemeTokens = self.get_tokens();
        let gesture_sig: Signal<Option<GestureEvent>> = ctx.get_gesture();
        let host: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_SM
                padding: Edges::all(SPACE_MD)
                background: p.get_card()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_lg()
                bind_children: move || {
                    let live: LivePhotoAsset = sig.get();
                    let active: bool = live.get_active_motion();
                    let muted: bool = live.get_muted();
                    let live_buf: ImageBuffer = live.active_image();
                    let live_aspect: f32 = (live_buf.get_width().max(1) as f32)
                    / (live_buf.get_height().max(1) as f32);
                    let live_fit_w: f32 = (h_px * live_aspect).round().max(48.0);
                    vec![
                        view! {
                            row {
                                flex_wrap: FlexWrap::Wrap
                                width: Length::Percent(100.0)
                                justify: JustifyContent::SpaceBetween
                                align: AlignItems::Center
                                gap: SPACE_SM
                                text {
                                    font_size: g.get_font_body()
                                    font_weight: FontWeight::SemiBold
                                    color: p.get_card_foreground()
                                    { live.get_metadata().get_title().clone() }
                                }
                                EuvBadge(
                                    if active {
                                        LABEL_LIVE_PLAYING.to_string()
                                    } else {
                                        LABEL_LIVE_PHOTO.to_string()
                                    },
                                    if active { EuvTone::Warning } else { EuvTone::Primary },
                                    tokens,
                                )
                            }
                        },
                        view! {
                            row {
                                width: Length::Percent(100.0)
                                height: Length::Px(h_px)
                                justify: JustifyContent::Center
                                align: AlignItems::Center
                                background: p.get_media_surface()
                                radius: g.get_radius_md()
                                border: (
                                    if active { 2.0 } else { g.get_border_width() },
                                    if active { p.get_warning() } else { p.get_border() },
                                )
                                cursor: CursorIcon::Pointer
                                overflow: Overflow::Hidden
                                on_pointer_down: move |_: &UiEvent| {
                                    let mut next: LivePhotoAsset = sig.get_untracked();
                                    next.set_motion_playing(true);
                                    next.step_ms(120);
                                    let _: bool = sig.set(next);
                                }
                                on_pointer_up: move |_: &UiEvent| {
                                    let mut next: LivePhotoAsset = sig.get_untracked();
                                    next.set_motion_playing(false);
                                    let _: bool = sig.set(next);
                                }
                                image {
                                    source: live_buf
                                    width: Length::Px(live_fit_w)
                                    max_width: Length::Percent(100.0)
                                    height: Length::Px(h_px)
                                }
                            }
                        },
                        view! {
                            EuvActionRow() {
                                EuvButton(
                                    if active {
                                        LABEL_STOP_LIVE.to_string()
                                    } else {
                                        LABEL_TRIGGER_LIVE.to_string()
                                    },
                                    tokens,
                                ) {
                                    sm()
                                    on_click: move |_: &UiEvent| {
                                        let mut next: LivePhotoAsset = sig.get_untracked();
                                        let next_active: bool = !next.get_active_motion();
                                        next.set_motion_playing(next_active);
                                        if next_active {
                                            next.step_ms(120);
                                        }
                                        let _: bool = sig.set(next);
                                    }
                                }
                                EuvButton(
                                    if muted {
                                        LABEL_UNMUTE.to_string()
                                    } else {
                                        LABEL_MUTE.to_string()
                                    },
                                    tokens,
                                ) {
                                    outline()
                                    sm()
                                    on_click: move |_: &UiEvent| {
                                        let mut next: LivePhotoAsset = sig.get_untracked();
                                        let cur_m: bool = next.get_muted();
                                        next.set_muted(!cur_m);
                                        let _: bool = sig.set(next);
                                    }
                                }
                            }
                        },
                    ]
                }
            }
        };
        let (g_scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            let last_seq: Rc<Cell<u64>> = Rc::new(Cell::new(
                gesture_sig
                    .get_untracked()
                    .map(|ev: GestureEvent| ev.get_sequence())
                    .unwrap_or(0),
            ));
            Effect::new(move || {
                if let Some(ev) = gesture_sig.get()
                    && ev.get_sequence() != last_seq.replace(ev.get_sequence())
                    && ev.get_kind() == GestureEventKind::LongPress
                {
                    let mut next: LivePhotoAsset = sig.get_untracked();
                    next.set_motion_playing(true);
                    next.step_ms(120);
                    let _: bool = sig.set(next);
                }
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(host, g_scope);
            },
            (),
        );
        host
    }
}

/// Constructors for `EuvVideoPlayer`.
impl EuvVideoPlayer {
    /// Creates an interactive cross-platform video player bound to `state`.
    ///
    /// # Arguments
    ///
    /// - `Signal<VideoTrack>` - Reactive video track signal.
    /// - `ThemeTokens` - Active theme tokens.
    ///
    /// # Returns
    ///
    /// - `Self` - Configured `EuvVideoPlayer`.
    #[must_use]
    pub fn new(state: Signal<VideoTrack>, tokens: ThemeTokens) -> Self {
        Self {
            state,
            height_px: 184.0,
            tokens,
        }
    }
}

/// Declarative `#[component]` implementation for `EuvVideoPlayer`.
#[component]
/// Inherent implementation of [`EuvVideoPlayer`].
impl EuvVideoPlayer {
    /// Renders `EuvVideoPlayer` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let sig: Signal<VideoTrack> = self.get_state();
        let h_px: f32 = self.get_height_px();
        let tokens: ThemeTokens = self.get_tokens();
        view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_SM
                padding: Edges::all(SPACE_MD)
                background: p.get_card()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_lg()
                bind_children: move || {
                    let track: VideoTrack = sig.get();
                    let playing: bool = track.get_state() == PlaybackState::Playing;
                    let dur: f32 = track.get_metadata().get_duration_secs().max(0.1);
                    let cur: f32 = track.get_current_time_secs();
                    let rate: f32 = track.get_playback_rate();
                    let muted: bool = track.get_muted();
                    let vid_buf: ImageBuffer = track.active_frame();
                    let vid_aspect: f32 = (vid_buf.get_width().max(1) as f32)
                    / (vid_buf.get_height().max(1) as f32);
                    let vid_fit_w: f32 = (h_px * vid_aspect).round().max(48.0);
                    let caption_text: String = format!(
                        "CC: {}  [{:02}:{:02} / {:02}:{:02}]",
                        track.active_caption(),
                        (cur as u32) / 60,
                        (cur as u32) % 60,
                        (dur as u32) / 60,
                        (dur as u32) % 60
                    );
                    vec![
                        view! {
                            row {
                                flex_wrap: FlexWrap::Wrap
                                width: Length::Percent(100.0)
                                justify: JustifyContent::SpaceBetween
                                align: AlignItems::Center
                                gap: SPACE_SM
                                text {
                                    font_size: g.get_font_body()
                                    font_weight: FontWeight::SemiBold
                                    color: p.get_card_foreground()
                                    { track.get_metadata().get_title().clone() }
                                }
                                EuvBadge(
                                    track.get_metadata().get_format_label().clone(),
                                    if playing { EuvTone::Success } else { EuvTone::Primary },
                                    tokens,
                                )
                            }
                        },
                        view! {
                            row {
                                width: Length::Percent(100.0)
                                height: Length::Px(h_px)
                                justify: JustifyContent::Center
                                align: AlignItems::Center
                                background: p.get_media_surface()
                                radius: g.get_radius_md()
                                border: (g.get_border_width(), p.get_border())
                                overflow: Overflow::Hidden
                                image {
                                    source: vid_buf
                                    width: Length::Px(vid_fit_w)
                                    max_width: Length::Percent(100.0)
                                    height: Length::Px(h_px)
                                }
                            }
                        },
                        view! {
                            text {
                                font_size: 12.0
                                color: p.get_muted_foreground()
                                { caption_text }
                            }
                        },
                        view! {
                            EuvProgress((cur / dur).clamp(0.0, 1.0), tokens)
                        },
                        view! {
                            EuvActionRow() {
                                EuvButton(
                                    if playing {
                                        LABEL_PAUSE_VIDEO.to_string()
                                    } else {
                                        LABEL_PLAY_VIDEO.to_string()
                                    },
                                    tokens,
                                ) {
                                    sm()
                                    on_click: move |_: &UiEvent| {
                                        let mut next: VideoTrack = sig.get_untracked();
                                        next.toggle_play();
                                        if next.get_state() == PlaybackState::Playing {
                                            next.step(1.0);
                                        }
                                        let _: bool = sig.set(next);
                                    }
                                }
                                EuvButton(LABEL_SEEK_BACK_3S.to_string(), tokens) {
                                    secondary()
                                    sm()
                                    on_click: move |_: &UiEvent| {
                                        let mut next: VideoTrack = sig.get_untracked();
                                        let t: f32 = next.get_current_time_secs();
                                        next.seek(t - 3.0);
                                        let _: bool = sig.set(next);
                                    }
                                }
                                EuvButton(LABEL_SEEK_FWD_3S.to_string(), tokens) {
                                    secondary()
                                    sm()
                                    on_click: move |_: &UiEvent| {
                                        let mut next: VideoTrack = sig.get_untracked();
                                        let t: f32 = next.get_current_time_secs();
                                        next.seek(t + 3.0);
                                        let _: bool = sig.set(next);
                                    }
                                }
                                EuvButton(format!("{rate:.1}x"), tokens) {
                                    outline()
                                    sm()
                                    on_click: move |_: &UiEvent| {
                                        let mut next: VideoTrack = sig.get_untracked();
                                        let cur_r: f32 = next.get_playback_rate();
                                        let next_r: f32 = if cur_r < 1.25 {
                                            1.5
                                        } else if cur_r < 1.75 {
                                            2.0
                                        } else {
                                            1.0
                                        };
                                        next.set_playback_rate(next_r);
                                        let _: bool = sig.set(next);
                                    }
                                }
                                EuvButton(
                                    if muted {
                                        LABEL_UNMUTE.to_string()
                                    } else {
                                        LABEL_MUTE.to_string()
                                    },
                                    tokens,
                                ) {
                                    ghost()
                                    sm()
                                    on_click: move |_: &UiEvent| {
                                        let mut next: VideoTrack = sig.get_untracked();
                                        let cur_m: bool = next.get_muted();
                                        next.set_muted(!cur_m);
                                        let _: bool = sig.set(next);
                                    }
                                }
                            }
                        },
                    ]
                }
            }
        }
    }
}

/// Constructors for `EuvAudioPlayer`.
impl EuvAudioPlayer {
    /// Creates an interactive studio audio player bound to `state`.
    ///
    /// # Arguments
    ///
    /// - `Signal<AudioTrack>` - Reactive audio track signal.
    /// - `ThemeTokens` - Active theme tokens.
    ///
    /// # Returns
    ///
    /// - `Self` - Configured `EuvAudioPlayer`.
    #[must_use]
    pub fn new(state: Signal<AudioTrack>, tokens: ThemeTokens) -> Self {
        Self { state, tokens }
    }
}

/// Declarative `#[component]` implementation for `EuvAudioPlayer`.
#[component]
/// Inherent implementation of [`EuvAudioPlayer`].
impl EuvAudioPlayer {
    /// Renders `EuvAudioPlayer` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let sig: Signal<AudioTrack> = self.get_state();
        let tokens: ThemeTokens = self.get_tokens();
        view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_SM
                padding: Edges::all(SPACE_MD)
                background: p.get_card()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_lg()
                bind_children: move || {
                    let track: AudioTrack = sig.get();
                    let playing: bool = track.get_state() == PlaybackState::Playing;
                    let dur: f32 = track.get_metadata().get_duration_secs().max(0.1);
                    let cur: f32 = track.get_current_time_secs();
                    let frac: f32 = (cur / dur).clamp(0.0, 1.0);
                    let muted: bool = track.get_muted();
                    let peaks: Vec<f32> = track.get_waveform_peaks().clone();
                    let wave_bg: Color = p.get_muted();
                    let wave_active: Color = p.get_primary();
                    let wave_idle: Color = p.get_border();
                    vec![
                        view! {
                            row {
                                flex_wrap: FlexWrap::Wrap
                                width: Length::Percent(100.0)
                                justify: JustifyContent::SpaceBetween
                                align: AlignItems::Center
                                gap: SPACE_SM
                                text {
                                    font_size: g.get_font_body()
                                    font_weight: FontWeight::SemiBold
                                    color: p.get_card_foreground()
                                    { track.get_metadata().get_title().clone() }
                                }
                                EuvBadge(
                                    track.get_metadata().get_format_label().clone(),
                                    EuvTone::Primary,
                                    tokens,
                                )
                            }
                        },
                        view! {
                            canvas {
                                width: Length::Percent(100.0)
                                height: Length::Px(56.0)
                                radius: g.get_radius_md()
                                border: (g.get_border_width(), p.get_border())
                                painter: move |bounds: Rect, list: &mut DisplayList| {
                                    list.push(DrawCommand::FillRoundedRect {
                                            rect: bounds,
                                            color: wave_bg,
                                            radius: CornerRadius::all(6.0),
                                            border: Border::default(),
                                    });
                                    let bar_count: usize = peaks.len().max(1);
                                    let pad_x: f32 = 8.0;
                                    let usable_w: f32 = (bounds.get_width() - pad_x * 2.0).max(16.0);
                                    let slot_w: f32 = usable_w / (bar_count as f32);
                                    let bar_w: f32 = (slot_w * 0.62).max(2.0);
                                    let max_h: f32 = (bounds.get_height() - 12.0).max(8.0);
                                    let played_idx: usize = ((bar_count as f32) * frac).round() as usize;
                                    for (idx, amp) in peaks.iter().copied().enumerate() {
                                        let bh: f32 = (amp.clamp(0.1, 1.0) * max_h).max(4.0);
                                        let bx: f32 = bounds.get_x() + pad_x + (idx as f32) * slot_w;
                                        let by: f32 = bounds.get_y() + (bounds.get_height() - bh) * 0.5;
                                        list.push(DrawCommand::FillRoundedRect {
                                                rect: Rect::new(bx, by, bar_w, bh),
                                                color: if idx <= played_idx {
                                                    wave_active
                                                } else {
                                                    wave_idle
                                                },
                                                radius: CornerRadius::all(2.0),
                                                border: Border::default(),
                                        });
                                    }
                                }
                            }
                        },
                        view! {
                            text {
                                font_size: 12.0
                                color: p.get_muted_foreground()
                                { format!(
                                        "{:02}:{:02} / {:02}:{:02} · {} samples",
                                        (cur as u32) / 60,
                                        (cur as u32) % 60,
                                        (dur as u32) / 60,
                                        (dur as u32) % 60,
                                        track.get_samples().len()
                                ) }
                            }
                        },
                        view! {
                            EuvActionRow() {
                                EuvButton(
                                    if playing {
                                        LABEL_PAUSE_AUDIO.to_string()
                                    } else {
                                        LABEL_PLAY_AUDIO.to_string()
                                    },
                                    tokens,
                                ) {
                                    sm()
                                    on_click: move |_: &UiEvent| {
                                        let mut next: AudioTrack = sig.get_untracked();
                                        next.toggle_play();
                                        if next.get_state() == PlaybackState::Playing {
                                            next.step(1.5);
                                        }
                                        let _: bool = sig.set(next);
                                    }
                                }
                                EuvButton(LABEL_SEEK_BACK_2S.to_string(), tokens) {
                                    secondary()
                                    sm()
                                    on_click: move |_: &UiEvent| {
                                        let mut next: AudioTrack = sig.get_untracked();
                                        let t: f32 = next.get_current_time_secs();
                                        next.seek(t - 2.0);
                                        let _: bool = sig.set(next);
                                    }
                                }
                                EuvButton(LABEL_SEEK_FWD_2S.to_string(), tokens) {
                                    secondary()
                                    sm()
                                    on_click: move |_: &UiEvent| {
                                        let mut next: AudioTrack = sig.get_untracked();
                                        let t: f32 = next.get_current_time_secs();
                                        next.seek(t + 2.0);
                                        let _: bool = sig.set(next);
                                    }
                                }
                                EuvButton(
                                    if muted {
                                        LABEL_UNMUTE_AUDIO.to_string()
                                    } else {
                                        LABEL_MUTE_AUDIO.to_string()
                                    },
                                    tokens,
                                ) {
                                    outline()
                                    sm()
                                    on_click: move |_: &UiEvent| {
                                        let mut next: AudioTrack = sig.get_untracked();
                                        let cur_m: bool = next.get_muted();
                                        next.set_muted(!cur_m);
                                        let _: bool = sig.set(next);
                                    }
                                }
                            }
                        },
                    ]
                }
            }
        }
    }
}

/// Constructors for `EuvPdfViewer`.
impl EuvPdfViewer {
    /// Creates an interactive multi-page PDF viewer bound to `state`.
    ///
    /// # Arguments
    ///
    /// - `Signal<PdfDocument>` - Reactive PDF document signal.
    /// - `ThemeTokens` - Active theme tokens.
    ///
    /// # Returns
    ///
    /// - `Self` - Configured `EuvPdfViewer`.
    #[must_use]
    pub fn new(state: Signal<PdfDocument>, tokens: ThemeTokens) -> Self {
        Self { state, tokens }
    }
}

/// Declarative `#[component]` implementation for `EuvPdfViewer`.
#[component]
/// Inherent implementation of [`EuvPdfViewer`].
impl EuvPdfViewer {
    /// Renders `EuvPdfViewer` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let ctx: Context = current_context();
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let sig: Signal<PdfDocument> = self.get_state();
        let tokens: ThemeTokens = self.get_tokens();
        let gesture_sig: Signal<Option<GestureEvent>> = ctx.get_gesture();
        let host: NodeId = view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_SM
                padding: Edges::all(SPACE_MD)
                background: p.get_card()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_lg()
                bind_children: move || {
                    let doc: PdfDocument = sig.get();
                    let total_pages: usize = doc.get_pages().len().max(1);
                    let cur_page: usize = doc.get_current_page().min(total_pages - 1);
                    let zoom_pct: u32 = (doc.get_zoom() * 100.0).round() as u32;
                    let heading_size: f32 = (15.0 * doc.get_zoom()).clamp(13.0, 22.0);
                    let body_size: f32 = (13.0 * doc.get_zoom()).clamp(11.0, 18.0);
                    let page_blocks: Vec<String> = doc
                    .active_page()
                    .map(|pg: &PdfPage| pg.get_text_blocks().clone())
                    .unwrap_or_default();
                    vec![
                        view! {
                            row {
                                flex_wrap: FlexWrap::Wrap
                                width: Length::Percent(100.0)
                                justify: JustifyContent::SpaceBetween
                                align: AlignItems::Center
                                gap: SPACE_SM
                                text {
                                    font_size: g.get_font_body()
                                    font_weight: FontWeight::SemiBold
                                    color: p.get_card_foreground()
                                    { doc.get_title().clone() }
                                }
                                EuvBadge(
                                    format!(
                                        "PDF {} · Page {}/{} · {}%",
                                        doc.get_version(),
                                        cur_page + 1,
                                        total_pages,
                                        zoom_pct
                                    ),
                                    EuvTone::Destructive,
                                    tokens,
                                )
                            }
                        },
                        view! {
                            EuvActionRow() {
                                EuvButton(LABEL_PREV_PAGE.to_string(), tokens) {
                                    secondary()
                                    sm()
                                    on_click: move |_: &UiEvent| {
                                        let mut next: PdfDocument = sig.get_untracked();
                                        next.prev_page();
                                        let _: bool = sig.set(next);
                                    }
                                }
                                EuvButton(LABEL_NEXT_PAGE.to_string(), tokens) {
                                    secondary()
                                    sm()
                                    on_click: move |_: &UiEvent| {
                                        let mut next: PdfDocument = sig.get_untracked();
                                        next.next_page();
                                        let _: bool = sig.set(next);
                                    }
                                }
                                EuvButton(LABEL_ZOOM_OUT.to_string(), tokens) {
                                    outline()
                                    sm()
                                    on_click: move |_: &UiEvent| {
                                        let mut next: PdfDocument = sig.get_untracked();
                                        next.adjust_zoom(-0.25);
                                        let _: bool = sig.set(next);
                                    }
                                }
                                EuvButton(LABEL_ZOOM_IN.to_string(), tokens) {
                                    outline()
                                    sm()
                                    on_click: move |_: &UiEvent| {
                                        let mut next: PdfDocument = sig.get_untracked();
                                        next.adjust_zoom(0.25);
                                        let _: bool = sig.set(next);
                                    }
                                }
                            }
                        },
                        view! {
                            column {
                                width: Length::Percent(100.0)
                                padding: Edges::all(SPACE_LG)
                                gap: SPACE_SM
                                background: p.get_muted()
                                border: (g.get_border_width(), p.get_border())
                                radius: g.get_radius_md()
                                for (b_idx, block) in page_blocks.into_iter().enumerate() {
                                    text {
                                        font_size: if b_idx == 0 { heading_size } else { body_size }
                                        font_weight: if b_idx == 0 {
                                            FontWeight::Bold
                                        } else {
                                            FontWeight::Normal
                                        }
                                        color: if b_idx == 0 {
                                            p.get_foreground()
                                        } else {
                                            p.get_muted_foreground()
                                        }
                                        { block }
                                    }
                                }
                                text {
                                    font_size: 12.0
                                    color: p.get_muted_foreground()
                                    { format!("Author: {} · A4 (595×842 pt)", doc.get_author()) }
                                }
                            }
                        },
                    ]
                }
            }
        };
        let (pdf_scope, _): (Scope, Effect) = Scope::run(|_: Scope| {
            let last_seq: Rc<Cell<u64>> = Rc::new(Cell::new(
                gesture_sig
                    .get_untracked()
                    .map(|ev: GestureEvent| ev.get_sequence())
                    .unwrap_or(0),
            ));
            Effect::new(move || {
                if let Some(ev) = gesture_sig.get()
                    && ev.get_sequence() != last_seq.replace(ev.get_sequence())
                {
                    match ev.get_kind() {
                        GestureEventKind::SwipeLeft => {
                            let mut next: PdfDocument = sig.get_untracked();
                            next.next_page();
                            let _: bool = sig.set(next);
                        }
                        GestureEventKind::SwipeRight => {
                            let mut next: PdfDocument = sig.get_untracked();
                            next.prev_page();
                            let _: bool = sig.set(next);
                        }
                        GestureEventKind::Pinch => {
                            let mut next: PdfDocument = sig.get_untracked();
                            let delta: f32 = if ev.get_scale() > 1.0 { 0.25 } else { -0.25 };
                            next.adjust_zoom(delta);
                            let _: bool = sig.set(next);
                        }
                        _ => {}
                    }
                }
            })
        });
        ctx.get_scene().with_write(
            |tree: &mut SceneTree| {
                tree.attach_scope(host, pdf_scope);
            },
            (),
        );
        host
    }
}

/// Constructors and hyperlink callback builder for `EuvHtmlView`.
impl EuvHtmlView {
    /// Creates a native sanitized HTML5 viewer for `document`.
    ///
    /// # Arguments
    ///
    /// - `HtmlDocument` - Parsed HTML document.
    /// - `ThemeTokens` - Active theme tokens.
    ///
    /// # Returns
    ///
    /// - `Self` - Configured `EuvHtmlView`.
    #[must_use]
    pub fn new(document: HtmlDocument, tokens: ThemeTokens) -> Self {
        Self {
            document,
            on_link: None,
            tokens,
        }
    }

    /// Registers a native Rust callback invoked when a rendered HTML hyperlink is clicked.
    ///
    /// # Arguments
    ///
    /// - `Self` - Builder value.
    /// - `F` - Closure receiving the clicked hyperlink `href` string.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated `EuvHtmlView`.
    #[must_use]
    pub fn on_link<F>(mut self, callback: F) -> Self
    where
        F: FnMut(String) + 'static,
    {
        self.set_on_link(Some(Rc::new(RefCell::new(callback))));
        self
    }
}

/// Declarative `#[component]` implementation for `EuvHtmlView`.
#[component]
/// Inherent implementation of [`EuvHtmlView`].
impl EuvHtmlView {
    /// Renders `EuvHtmlView` into the ambient framework-managed `Context`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Component descriptor reference.
    ///
    /// # Returns
    ///
    /// - `NodeId` - Mounted root scene node handle.
    fn render(&self) -> NodeId {
        let p: ThemePalette = self.get_tokens().get_palette();
        let g: ThemeGeometry = self.get_tokens().get_geometry();
        let tokens: ThemeTokens = self.get_tokens();
        let Self {
            document, on_link, ..
        } = self.clone();
        view! {
            column {
                width: Length::Percent(100.0)
                gap: SPACE_SM
                padding: Edges::all(SPACE_MD)
                background: p.get_card()
                border: (g.get_border_width(), p.get_border())
                radius: g.get_radius_lg()
                row {
                    flex_wrap: FlexWrap::Wrap
                    width: Length::Percent(100.0)
                    justify: JustifyContent::SpaceBetween
                    align: AlignItems::Center
                    gap: SPACE_SM
                    text {
                        font_size: g.get_font_body()
                        font_weight: FontWeight::SemiBold
                        color: p.get_card_foreground()
                        { document.get_title().clone() }
                    }
                    EuvBadge(
                        format!(
                            "HTML5 · {} blocks · {} links",
                            document.get_blocks().len(),
                            document.get_links_count()
                        ),
                        EuvTone::Success,
                        tokens,
                    )
                }
                for block in document.get_blocks().iter() {
                    match block.get_kind() {
                        HtmlNodeKind::Heading(level) => text {
                            font_size: if level <= 1 {
                                18.0
                            } else if level == 2 {
                                16.0
                            } else {
                                14.5
                            }
                            font_weight: FontWeight::Bold
                            color: p.get_foreground()
                            { block.get_text().clone() }
                        },
                        HtmlNodeKind::Paragraph => text {
                            font_size: g.get_font_body()
                            color: p.get_muted_foreground()
                            { block.get_text().clone() }
                        },
                        HtmlNodeKind::Blockquote => column {
                            width: Length::Percent(100.0)
                            padding: Edges::all(SPACE_MD)
                            background: p.get_muted()
                            border: (g.get_border_width(), p.get_primary())
                            radius: g.get_radius_md()
                            text {
                                font_size: g.get_font_body()
                                color: p.get_foreground()
                                { block.get_text().clone() }
                            }
                        },
                        HtmlNodeKind::CodeBlock => column {
                            width: Length::Percent(100.0)
                            padding: Edges::all(SPACE_SM)
                            background: p.get_muted()
                            border: (g.get_border_width(), p.get_border())
                            radius: g.get_radius_sm()
                            text {
                                font_size: 12.5
                                color: p.get_primary()
                                { block.get_text().clone() }
                            }
                        },
                        HtmlNodeKind::ListItem => text {
                            font_size: g.get_font_body()
                            color: p.get_card_foreground()
                            { format!("• {}", block.get_text()) }
                        },
                        HtmlNodeKind::Link => EuvButton(block.get_text().clone(), tokens) {
                            outline()
                            sm()
                            on_click: {
                                let href_str: String =
                                block.href_url().unwrap_or_else(|| block.get_text().clone());
                                let cb_opt: Option<HtmlLinkCallback> = on_link.clone();
                                move |_: &UiEvent| {
                                    if let Some(cb) = cb_opt.as_ref()
                                    && let Ok(mut f) = cb.try_borrow_mut()
                                    {
                                        f(href_str.clone());
                                    }
                                }
                            }
                        },
                        HtmlNodeKind::Image => text {
                            font_size: 12.0
                            color: p.get_muted_foreground()
                            { format!("[Image: {}]", block.get_text()) }
                        },
                        HtmlNodeKind::HorizontalRule => EuvDivider(false, tokens),
                    }
                }
            }
        }
    }
}
