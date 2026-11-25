use super::*;

/// Individual spatialized audio playback command queued by game scenes.
#[derive(Clone, Copy, Data, Debug, New, PartialEq)]
pub struct AudioCommand {
    /// Sound cue identifier.
    #[get(type(copy))]
    pub(crate) cue: SoundCue,
    /// Linear gain in `[0.0, 1.0]`.
    #[get(type(copy))]
    pub(crate) volume: f32,
    /// Stereo pan in `[-1.0, 1.0]` (`0.0` = center).
    #[get(type(copy))]
    pub(crate) pan: f32,
    /// Pitch multiplier (`1.0` = nominal playback speed).
    #[get(type(copy))]
    pub(crate) pitch: f32,
}

/// Cross-platform deterministic game audio mixer queue (bridged to WebAudio / CoreAudio / AAudio / WASAPI).
#[derive(Clone, Data, Debug)]
pub struct AudioBus {
    /// Master gain in `[0.0, 1.0]`.
    #[get(type(copy))]
    pub(crate) master_volume: f32,
    /// Global mute toggle.
    #[get(type(copy))]
    pub(crate) muted: bool,
    /// Pending audio commands emitted during the current frame.
    pub(crate) queue: Vec<AudioCommand>,
    /// Cumulative sound commands triggered over the lifetime of the bus.
    #[get(type(copy))]
    pub(crate) total_played: u64,
}
