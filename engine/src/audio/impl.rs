use super::*;

/// Implements audio context management for `GameAudioContext`.
impl GameAudioContext {
    /// Creates a new audio context with default master volume.
    ///
    /// # Returns
    ///
    /// - `Option<GameAudioContext>` - The new audio context, or `None` when
    ///   the browser does not expose a WebAudio `AudioContext`.
    pub fn create() -> Option<GameAudioContext> {
        let Ok(context) = AudioContext::new() else {
            return None;
        };
        let Ok(master_gain) = context.create_gain() else {
            return None;
        };
        let _: Result<AudioNode, JsValue> =
            master_gain.connect_with_audio_node(&context.destination());
        master_gain.gain().set_value(AUDIO_DEFAULT_VOLUME as f32);
        Some(GameAudioContext::new(context, master_gain))
    }

    /// Sets the master volume for all audio output.
    ///
    /// # Arguments
    ///
    /// - `f64` - The volume level in the range 0.0 to 1.0.
    pub fn apply_master_volume(&self, volume: f64) {
        let clamped: f64 = Numeric::clamp(volume, 0.0, 1.0);
        self.get_master_gain().gain().set_value(clamped as f32);
    }

    /// Resumes the audio context if it was suspended.
    pub fn resume(&self) {
        let _: Result<Promise, JsValue> = self.get_context().resume();
    }

    /// Suspends the audio context, temporarily halting all audio processing.
    pub fn suspend(&self) {
        let _: Result<Promise, JsValue> = self.get_context().suspend();
    }

    /// Closes the audio context and releases all resources.
    pub fn close(&self) {
        let _: Result<Promise, JsValue> = self.get_context().close();
    }

    /// Returns the current sample rate of the audio context in Hz.
    ///
    /// # Returns
    ///
    /// - `f64` - The sample rate.
    pub fn sample_rate(&self) -> f64 {
        self.get_context().sample_rate() as f64
    }

    /// Returns the current playback time in seconds.
    ///
    /// # Returns
    ///
    /// - `f64` - The current time.
    pub fn current_time(&self) -> f64 {
        self.get_context().current_time()
    }
}

/// Implements playback control for `AudioClip`.
impl AudioClip {
    /// Creates a new audio clip from a decoded buffer.
    ///
    /// A clip carries no name: playback is driven by `play` / `stop`, which
    /// address a clip by identity rather than by label, so a name field would
    /// only ever be stored and never read.
    ///
    /// # Arguments
    ///
    /// - `AudioBuffer` - The decoded audio data.
    ///
    /// # Returns
    ///
    /// - `AudioClip` - The new clip.
    pub fn create(buffer: AudioBuffer) -> AudioClip {
        AudioClip::new(
            buffer,
            AudioPlayState::Stopped,
            AUDIO_DEFAULT_LOOP,
            AUDIO_DEFAULT_VOLUME,
            AUDIO_DEFAULT_PLAYBACK_RATE,
        )
    }

    /// Plays this clip through the given audio context.
    ///
    /// Creates a new `AudioBufferSourceNode`, connects it through a gain node
    /// to the master gain, and starts playback. If already playing, does nothing.
    ///
    /// # Arguments
    ///
    /// - `&GameAudioContext` - The audio context to play through.
    pub fn play(&mut self, audio_context: &GameAudioContext) {
        if self.get_state() == AudioPlayState::Playing {
            return;
        }
        let Ok(source) = audio_context.get_context().create_buffer_source() else {
            return;
        };
        source.set_buffer(Some(self.get_buffer()));
        source.set_loop(self.get_looping());
        source
            .playback_rate()
            .set_value(self.get_playback_rate() as f32);
        let Ok(gain) = audio_context.get_context().create_gain() else {
            return;
        };
        gain.gain().set_value(self.get_volume() as f32);
        let _: Result<AudioNode, JsValue> = source.connect_with_audio_node(&gain);
        let _: Result<AudioNode, JsValue> =
            gain.connect_with_audio_node(audio_context.get_master_gain());
        let _: Result<(), JsValue> = source.start();
        self.set_source_node(Some(source));
        self.set_state(AudioPlayState::Playing);
    }

    /// Stops playback of this clip and releases the source node.
    pub fn stop(&mut self) {
        if let Some(source) = self.get_mut_source_node().take() {
            let scheduled: &AudioScheduledSourceNode = source.as_ref();
            let _: Result<(), JsValue> = scheduled.stop_with_when(0.0);
            let _: Result<(), JsValue> = source.disconnect();
        }
        self.set_state(AudioPlayState::Stopped);
    }

    /// Sets whether the clip should loop.
    ///
    /// # Arguments
    ///
    /// - `bool` - True to enable looping.
    pub fn update_looping(&mut self, looping: bool) {
        self.set_looping(looping);
        if let Some(source) = self.get_mut_source_node() {
            source.set_loop(looping);
        }
    }

    /// Sets the volume of this clip.
    ///
    /// # Arguments
    ///
    /// - `f64` - The volume in the range 0.0 to 1.0.
    pub fn update_volume(&mut self, volume: f64) {
        self.set_volume(Numeric::clamp(volume, 0.0, 1.0));
    }

    /// Sets the playback rate multiplier.
    ///
    /// # Arguments
    ///
    /// - `f64` - The playback rate (1.0 = normal speed).
    pub fn update_playback_rate(&mut self, rate: f64) {
        self.set_playback_rate(rate);
        if let Some(source) = self.get_mut_source_node() {
            source.playback_rate().set_value(rate as f32);
        }
    }

    /// Returns the duration of the audio buffer in seconds.
    ///
    /// # Returns
    ///
    /// - `f64` - The duration.
    pub fn duration(&self) -> f64 {
        self.get_buffer().duration()
    }

    /// Returns the number of channels in the audio buffer.
    ///
    /// # Returns
    ///
    /// - `u32` - The channel count.
    pub fn channel_count(&self) -> u32 {
        self.get_buffer().number_of_channels()
    }
}
