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

    /// Spatializes a 3D sound emitter relative to `listener` with inverse-distance attenuation, stereo ear panning, and Doppler shift.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable audio bus.
    /// - `&AudioListener3D` - Active 3D listener pose.
    /// - `&SpatialAudioSource3D` - World-space sound source parameters.
    pub fn play_spatial_3d(&mut self, listener: &AudioListener3D, source: &SpatialAudioSource3D) {
        let delta: Vec3 = source.get_position().sub(listener.get_position());
        let dist: f32 = delta.length();
        let max_d: f32 = source.get_max_distance().max(1.0);
        if dist >= max_d {
            return;
        }
        let min_d: f32 = source.get_min_distance().clamp(0.25, max_d);
        let atten: f32 = if dist <= min_d {
            1.0
        } else {
            let norm: f32 = ((dist - min_d) / (max_d - min_d).max(0.01)).clamp(0.0, 1.0);
            (1.0 - norm) * (1.0 - norm)
        };
        let to_src: Vec3 = if dist > 1e-4 {
            delta.scale(1.0 / dist)
        } else {
            Vec3::zero()
        };
        let pan: f32 = to_src.dot(listener.get_right()).clamp(-1.0, 1.0);
        let rel_vel: f32 = source
            .get_velocity()
            .sub(listener.get_velocity())
            .dot(to_src);
        let speed_of_sound: f32 = 343.0;
        let doppler: f32 =
            (speed_of_sound / (speed_of_sound + rel_vel).clamp(120.0, 680.0)).clamp(0.5, 2.0);
        self.play(
            source.get_cue(),
            source.get_volume() * atten,
            pan,
            source.get_pitch() * doppler,
        );
    }
}

/// Camera-based listener builder for `AudioListener3D`.
impl AudioListener3D {
    /// Builds an `AudioListener3D` from a `Camera3D` and optional linear velocity.
    ///
    /// # Arguments
    ///
    /// - `&Camera3D` - Active 3D perspective camera.
    /// - `Vec3` - Listener linear velocity in meters/sec.
    ///
    /// # Returns
    ///
    /// - `Self` - Positioned and oriented 3D audio listener.
    #[must_use]
    pub fn from_camera(camera: &Camera3D, velocity: Vec3) -> Self {
        Self::new(camera.get_position(), velocity, camera.right())
    }
}

/// Fluent constructor for `SpatialAudioSource3D`.
impl SpatialAudioSource3D {
    /// Creates a `SpatialAudioSource3D` at `position` with default `[2.0m, 80.0m]` attenuation radius.
    ///
    /// # Arguments
    ///
    /// - `SoundCue` - Sound cue identifier.
    /// - `Vec3` - World-space position in meters.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready 3D spatial audio emitter.
    #[must_use]
    pub fn new(cue: SoundCue, position: Vec3) -> Self {
        Self {
            cue,
            position,
            velocity: Vec3::zero(),
            volume: 1.0,
            pitch: 1.0,
            min_distance: 2.0,
            max_distance: 80.0,
        }
    }

    /// Sets linear velocity for Doppler pitch calculation.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source emitter.
    /// - `Vec3` - Velocity in meters/sec.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated spatial audio source.
    #[must_use]
    pub fn with_velocity(mut self, velocity: Vec3) -> Self {
        self.set_velocity(velocity);
        self
    }

    /// Sets nominal volume and audible `[min_distance, max_distance]` range.
    ///
    /// # Arguments
    ///
    /// - `Self` - Source emitter.
    /// - `f32` - Nominal volume `[0.0, 1.0]`.
    /// - `f32` - Reference full-volume distance in meters.
    /// - `f32` - Maximum audible radius in meters.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated spatial audio source.
    #[must_use]
    pub fn with_range(mut self, volume: f32, min_distance: f32, max_distance: f32) -> Self {
        self.set_volume(volume.clamp(0.0, 1.0));
        self.set_min_distance(min_distance.max(0.25));
        self.set_max_distance(max_distance.max(min_distance + 0.5));
        self
    }
}
