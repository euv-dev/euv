use super::*;

/// A wrapper around the Web Audio API `AudioContext` providing
/// game engine audio playback and volume management.
#[derive(Clone, Data, New)]
pub struct GameAudioContext {
    /// The underlying Web Audio API context.
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    #[get(pub(crate))]
    pub(crate) context: AudioContext,
    /// The master gain node controlling overall volume.
    ///
    /// The gain node's own `AudioParam` is the single source of truth for the
    /// master level. There is deliberately no `master_volume` field beside it:
    /// a second copy of the level could drift from the node it mirrors, and
    /// nothing in the engine ever read it. Volume changes go through
    /// [`GameAudioContext::apply_master_volume`].
    #[set(pub(crate))]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) master_gain: GainNode,
}

/// An audio clip wrapping a decoded `AudioBuffer` with playback control.
#[derive(Clone, Data, New)]
pub struct AudioClip {
    /// The decoded audio data buffer.
    #[set(pub(crate))]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) buffer: AudioBuffer,
    /// The current playback state.
    ///
    /// `get_state` is public because tests/audio_clip/fn.rs reads this field from
    /// outside this crate. `get_mut` / `set` stay crate-internal.
    #[get_mut(pub(crate))]
    #[get(type(copy))]
    #[set(pub(crate))]
    pub state: AudioPlayState,
    /// Whether the clip should loop when it reaches the end.
    ///
    /// `get_looping` is public because tests/audio_clip/fn.rs reads this field from
    /// outside this crate. `get_mut` / `set` stay crate-internal.
    #[get_mut(pub(crate))]
    #[get(type(copy))]
    #[set(pub(crate))]
    pub looping: bool,
    /// The volume level in the range 0.0 to 1.0.
    ///
    /// `get_volume` is public because tests/audio_clip/fn.rs reads this field from
    /// outside this crate. `get_mut` / `set` stay crate-internal.
    #[get_mut(pub(crate))]
    #[get(type(copy))]
    #[set(pub(crate))]
    pub volume: f64,
    /// The playback rate multiplier (1.0 = normal speed).
    ///
    /// `get_playback_rate` is public because tests/audio_clip/fn.rs reads this field from
    /// outside this crate. `get_mut` / `set` stay crate-internal.
    #[get_mut(pub(crate))]
    #[get(type(copy))]
    #[set(pub(crate))]
    pub playback_rate: f64,
    /// The currently active source node, if the clip is playing.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    #[get(pub(crate))]
    pub(crate) source_node: Option<AudioBufferSourceNode>,
}
