/// The mathematical constant pi (~3.14159).
pub const PI: f64 = std::f64::consts::PI;

/// Two times pi, the full circle angle in radians (~6.28318).
pub const TWO_PI: f64 = std::f64::consts::TAU;

/// Half of pi, a quarter circle angle in radians (~1.57079).
pub const HALF_PI: f64 = std::f64::consts::FRAC_PI_2;

/// Conversion factor from degrees to radians.
pub const DEG_TO_RAD: f64 = PI / 180.0;

/// Conversion factor from radians to degrees.
pub const RAD_TO_DEG: f64 = 180.0 / PI;

/// A small epsilon value used for floating-point comparisons.
pub const EPSILON: f64 = 1e-6;

/// The default gravity acceleration in pixels per second squared.
pub const DEFAULT_GRAVITY: f64 = 980.0;

/// The default linear damping coefficient applied per second.
pub const DEFAULT_LINEAR_DAMPING: f64 = 0.0;

/// The default angular damping coefficient applied per second.
pub const DEFAULT_ANGULAR_DAMPING: f64 = 0.0;

/// The default restitution (bounciness) for rigid body collisions.
pub const DEFAULT_RESTITUTION: f64 = 0.3;

/// The default friction coefficient for rigid body surface contact.
pub const DEFAULT_FRICTION: f64 = 0.7;

/// The fixed timestep duration in seconds for the game loop (60 FPS).
pub const DEFAULT_FIXED_TIMESTEP: f64 = 1.0 / 60.0;

/// The maximum accumulated time in seconds before the game loop drops frames.
pub const DEFAULT_MAX_FRAME_TIME: f64 = 0.25;

/// The default 3D gravity acceleration in meters per second squared (pointing down on the y axis).
pub const DEFAULT_GRAVITY_3D: f64 = -9.81;

/// The default near clipping plane distance for a 3D perspective camera.
pub const DEFAULT_CAMERA_NEAR: f64 = 0.1;

/// The default far clipping plane distance for a 3D perspective camera.
pub const DEFAULT_CAMERA_FAR: f64 = 1000.0;

/// The default vertical field of view in radians for a 3D perspective camera (~60 degrees).
pub const DEFAULT_CAMERA_FOV: f64 = 1.0471975511965976;
