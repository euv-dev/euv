/// The default maximum number of bounces evaluated by
/// [`RayTraceScene::trace`] before returning. Higher values yield more
/// accurate reflections at increased computational cost.
pub(crate) const RAYTRACE_DEFAULT_MAX_BOUNCES: u32 = 4;

/// The minimum ray parameter `t` accepted as a valid intersection. Hits
/// closer than this are treated as self-intersection artifacts and skipped.
pub(crate) const RAYTRACE_DEFAULT_T_MIN: f64 = 0.001;

/// The maximum ray parameter `t` evaluated before a ray is considered to
/// have escaped the scene. Pairs with `RAYTRACE_DEFAULT_T_MIN`.
pub(crate) const RAYTRACE_DEFAULT_T_MAX: f64 = 1000.0;
