use super::*;

/// Default unmuted audio bus at `0.85` master volume.
impl Default for AudioBus {
    /// Creates a default `AudioBus`.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready audio bus.
    fn default() -> Self {
        Self {
            master_volume: 0.85,
            muted: false,
            queue: Vec::with_capacity(32),
            total_played: 0,
        }
    }
}

/// Sound cue scheduling and frame draining for `AudioBus`.
impl AudioBus {
    /// Queues a sound cue with normalized volume, pan, and pitch.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable audio bus.
    /// - `SoundCue` - Sound cue to play.
    /// - `f32` - Linear volume `[0.0, 1.0]`.
    /// - `f32` - Stereo pan `[-1.0, 1.0]`.
    /// - `f32` - Pitch multiplier `[0.25, 4.0]`.
    pub fn play(&mut self, cue: SoundCue, volume: f32, pan: f32, pitch: f32) {
        if self.get_muted() {
            return;
        }
        let eff_vol: f32 = (volume.clamp(0.0, 1.0) * self.get_master_volume()).clamp(0.0, 1.0);
        if eff_vol <= 0.001 {
            return;
        }
        self.get_mut_queue().push(AudioCommand::new(
            cue,
            eff_vol,
            pan.clamp(-1.0, 1.0),
            pitch.clamp(0.25, 4.0),
        ));
        let next_total: u64 = self.get_total_played().saturating_add(1);
        self.set_total_played(next_total);
    }

    /// Drains all queued audio commands for platform audio playback.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable audio bus.
    ///
    /// # Returns
    ///
    /// - `Vec<AudioCommand>` - Ordered commands emitted since the previous drain.
    pub fn drain(&mut self) -> Vec<AudioCommand> {
        take(self.get_mut_queue())
    }
}
