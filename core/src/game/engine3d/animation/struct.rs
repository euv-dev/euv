use super::*;

/// Single timestamped 3D transform keyframe inside an `AnimationClip3D`.
#[derive(Clone, Copy, Data, Debug, New, PartialEq)]
pub struct Keyframe3D {
    /// Keyframe time offset in seconds.
    #[get(type(copy))]
    pub(crate) time_secs: f32,
    /// Local joint or node transform at `time_secs`.
    #[get(type(copy))]
    pub(crate) transform: Transform3D,
}

/// Named bone joint inside a hierarchical `SkeletonRig3D`.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct SkeletonJoint3D {
    /// Human-readable joint identifier (e.g., `"root"`, `"spine"`, `"turret"`).
    pub(crate) name: String,
    /// Optional parent joint index (`None` for root bone).
    #[get(skip)]
    pub(crate) parent: Option<usize>,
    /// Bind-pose local transform relative to parent.
    #[get(type(copy))]
    pub(crate) rest_transform: Transform3D,
}

/// Per-joint keyframe track animating one joint index over time.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct JointTrack3D {
    /// Target joint index inside `SkeletonRig3D::joints`.
    #[get(type(copy))]
    pub(crate) joint_index: usize,
    /// Chronologically sorted keyframes.
    pub(crate) keyframes: Vec<Keyframe3D>,
}

/// Reusable skeletal/transform animation clip with duration and looping control.
#[derive(Clone, Data, Debug, PartialEq)]
pub struct AnimationClip3D {
    /// Clip name.
    pub(crate) name: String,
    /// Total clip duration in seconds.
    #[get(type(copy))]
    pub(crate) duration_secs: f32,
    /// Whether playback wraps seamlessly at `duration_secs`.
    #[get(type(copy))]
    pub(crate) looping: bool,
    /// Per-joint keyframe tracks.
    pub(crate) tracks: Vec<JointTrack3D>,
}

/// Hierarchical 3D skeletal rig evaluating forward kinematics across bone chains.
#[derive(Clone, Data, Debug, Default, PartialEq)]
pub struct SkeletonRig3D {
    /// Ordered joint hierarchy (parents precede children).
    pub(crate) joints: Vec<SkeletonJoint3D>,
}
