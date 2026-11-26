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

/// 3D spatial audio listener pose derived from a `Camera3D` or player avatar.
#[derive(Clone, Copy, Data, Debug, New, PartialEq)]
pub struct AudioListener3D {
    /// Listener world position in meters.
    #[get(type(copy))]
    pub(crate) position: Vec3,
    /// Listener linear velocity in meters/sec for Doppler shift.
    #[get(type(copy))]
    pub(crate) velocity: Vec3,
    /// Unit listener right ear axis for stereo HRTF panning.
    #[get(type(copy))]
    pub(crate) right: Vec3,
}

/// 3D world-space sound emitter with distance rolloff and Doppler velocity.
#[derive(Clone, Copy, Data, Debug, PartialEq)]
pub struct SpatialAudioSource3D {
    /// Sound cue identifier.
    #[get(type(copy))]
    pub(crate) cue: SoundCue,
    /// Emitter world position in meters.
    #[get(type(copy))]
    pub(crate) position: Vec3,
    /// Emitter linear velocity in meters/sec for Doppler shift.
    #[get(type(copy))]
    pub(crate) velocity: Vec3,
    /// Nominal unattenuated gain `[0.0, 1.0]`.
    #[get(type(copy))]
    pub(crate) volume: f32,
    /// Nominal playback pitch multiplier (`1.0` = unshifted).
    #[get(type(copy))]
    pub(crate) pitch: f32,
    /// Reference distance in meters inside which gain stays at maximum.
    #[get(type(copy))]
    pub(crate) min_distance: f32,
    /// Maximum audible radius in meters beyond which sound is culled.
    #[get(type(copy))]
    pub(crate) max_distance: f32,
}
