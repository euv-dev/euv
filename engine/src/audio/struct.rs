use super::*;

/// A wrapper around the Web Audio API `AudioContext` providing
/// game engine audio playback and volume management.
#[derive(Clone, Data, New)]
pub struct GameAudioContext {
    /// The underlying Web Audio API context.
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) context: AudioContext,
    /// The master gain node controlling overall volume.
    #[set(pub(crate))]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) master_gain: GainNode,
    /// The master volume level in the range 0.0 to 1.0.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) master_volume: f64,
}

/// An audio clip wrapping a decoded `AudioBuffer` with playback control.
#[derive(Clone, Data, New)]
pub struct AudioClip {
    /// The decoded audio data buffer.
    #[set(pub(crate))]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) buffer: AudioBuffer,
    /// The name identifying this clip.
    #[set(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) name: String,
    /// The current playback state.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    pub(crate) state: AudioPlayState,
    /// Whether the clip should loop when it reaches the end.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) looping: bool,
    /// The volume level in the range 0.0 to 1.0.
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) volume: f64,
    /// The playback rate multiplier (1.0 = normal speed).
    #[get(type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) playback_rate: f64,
    /// The currently active source node, if the clip is playing.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    #[new(skip)]
    #[get(pub(crate))]
    pub(crate) source_node: Option<AudioBufferSourceNode>,
}
