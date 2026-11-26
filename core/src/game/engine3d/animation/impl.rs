use super::*;

/// Parent accessor for `SkeletonJoint3D`.
impl SkeletonJoint3D {
    /// Creates a new `SkeletonJoint3D`.
    ///
    /// # Arguments
    ///
    /// - `&str` - Joint label.
    /// - `Option<usize>` - Optional parent joint index.
    /// - `Transform3D` - Bind-pose local transform.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed joint record.
    #[must_use]
    pub fn new(name: &str, parent: Option<usize>, rest_transform: Transform3D) -> Self {
        Self {
            name: name.to_string(),
            parent,
            rest_transform,
        }
    }

    /// Returns the optional parent joint index.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Joint reference.
    ///
    /// # Returns
    ///
    /// - `Option<usize>` - Parent joint index if non-root.
    #[must_use]
    pub fn parent_index(&self) -> Option<usize> {
        let Self { parent, .. } = self;
        *parent
    }
}

/// Keyframe sampling helpers for `JointTrack3D`.
impl JointTrack3D {
    /// Creates a new `JointTrack3D` for `joint_index` sorting keyframes by timestamp.
    ///
    /// # Arguments
    ///
    /// - `usize` - Target joint index.
    /// - `Vec<Keyframe3D>` - Unsorted or sorted keyframes.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready animation track.
    #[must_use]
    pub fn new(joint_index: usize, mut keyframes: Vec<Keyframe3D>) -> Self {
        keyframes.sort_by(|a: &Keyframe3D, b: &Keyframe3D| {
            a.get_time_secs().total_cmp(&b.get_time_secs())
        });
        Self {
            joint_index,
            keyframes,
        }
    }

    /// Samples this track at `time_secs`, linearly interpolating between neighboring keyframes.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Track reference.
    /// - `f32` - Evaluation time in seconds.
    /// - `Transform3D` - Fallback bind-pose transform when track is empty.
    ///
    /// # Returns
    ///
    /// - `Transform3D` - Interpolated local transform.
    #[must_use]
    pub fn sample(&self, time_secs: f32, fallback: Transform3D) -> Transform3D {
        let keys: &[Keyframe3D] = self.get_keyframes().as_slice();
        if keys.is_empty() {
            return fallback;
        }
        if keys.len() == 1 || time_secs <= keys[0].get_time_secs() {
            return keys[0].get_transform();
        }
        let last: Keyframe3D = keys[keys.len() - 1];
        if time_secs >= last.get_time_secs() {
            return last.get_transform();
        }
        for i in 0..(keys.len() - 1) {
            let k0: Keyframe3D = keys[i];
            let k1: Keyframe3D = keys[i + 1];
            if time_secs <= k1.get_time_secs() {
                let span: f32 = (k1.get_time_secs() - k0.get_time_secs()).max(1e-5);
                let alpha: f32 = ((time_secs - k0.get_time_secs()) / span).clamp(0.0, 1.0);
                return k0.get_transform().lerp(k1.get_transform(), alpha);
            }
        }
        last.get_transform()
    }
}

/// Clip construction and time normalization for `AnimationClip3D`.
impl AnimationClip3D {
    /// Creates a new `AnimationClip3D`.
    ///
    /// # Arguments
    ///
    /// - `&str` - Clip name.
    /// - `f32` - Total duration in seconds.
    /// - `bool` - Looping flag.
    ///
    /// # Returns
    ///
    /// - `Self` - Empty animation clip ready for tracks.
    #[must_use]
    pub fn new(name: &str, duration_secs: f32, looping: bool) -> Self {
        Self {
            name: name.to_string(),
            duration_secs: duration_secs.max(1e-4),
            looping,
            tracks: Vec::new(),
        }
    }

    /// Appends a `JointTrack3D` to this clip and returns `self` for fluent chaining.
    ///
    /// # Arguments
    ///
    /// - `Self` - Base animation clip.
    /// - `JointTrack3D` - Joint animation track.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated animation clip.
    #[must_use]
    pub fn with_track(mut self, track: JointTrack3D) -> Self {
        self.get_mut_tracks().push(track);
        self
    }

    /// Normalizes raw playback time into `[0.0, duration_secs]` respecting `looping`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Clip reference.
    /// - `f32` - Unclamped playback time in seconds.
    ///
    /// # Returns
    ///
    /// - `f32` - Normalized local evaluation timestamp.
    #[must_use]
    pub fn normalize_time(&self, time_secs: f32) -> f32 {
        let dur: f32 = self.get_duration_secs().max(1e-4);
        if self.get_looping() {
            time_secs.rem_euclid(dur)
        } else {
            time_secs.clamp(0.0, dur)
        }
    }
}

/// Hierarchical forward-kinematics solver for `SkeletonRig3D`.
impl SkeletonRig3D {
    /// Appends a joint to the skeleton hierarchy and returns its assigned index.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable skeleton rig.
    /// - `&str` - Joint label.
    /// - `Option<usize>` - Optional parent joint index.
    /// - `Transform3D` - Bind-pose local transform.
    ///
    /// # Returns
    ///
    /// - `usize` - Assigned joint index.
    pub fn add_joint(
        &mut self,
        name: &str,
        parent: Option<usize>,
        rest_transform: Transform3D,
    ) -> usize {
        let idx: usize = self.get_joints().len();
        self.get_mut_joints()
            .push(SkeletonJoint3D::new(name, parent, rest_transform));
        idx
    }

    /// Evaluates hierarchical world-space `Mat4` joint palettes at `time_secs` for `clip`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Skeleton rig reference.
    /// - `&AnimationClip3D` - Active animation clip.
    /// - `f32` - Playback timestamp in seconds.
    /// - `Transform3D` - Root world transform of the animated actor.
    ///
    /// # Returns
    ///
    /// - `Vec<Mat4>` - World-space bone matrices indexed by joint id.
    #[must_use]
    pub fn sample_world_matrices(
        &self,
        clip: &AnimationClip3D,
        time_secs: f32,
        root_world: Transform3D,
    ) -> Vec<Mat4> {
        let t: f32 = clip.normalize_time(time_secs);
        let joints: &[SkeletonJoint3D] = self.get_joints().as_slice();
        let mut locals: Vec<Transform3D> = joints
            .iter()
            .map(SkeletonJoint3D::get_rest_transform)
            .collect();
        for track in clip.get_tracks().iter() {
            let j: usize = track.get_joint_index();
            if j < locals.len() {
                locals[j] = track.sample(t, locals[j]);
            }
        }
        let root_m: Mat4 = root_world.to_mat4();
        let mut world: Vec<Mat4> = Vec::with_capacity(joints.len());
        for (idx, joint) in joints.iter().enumerate() {
            let local_m: Mat4 = locals[idx].to_mat4();
            let parent_m: Mat4 = match joint.parent_index() {
                Some(p) if p < world.len() => world[p],
                _ => root_m,
            };
            world.push(parent_m.mul(local_m));
        }
        world
    }
}
