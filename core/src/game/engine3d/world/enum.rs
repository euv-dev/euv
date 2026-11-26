/// Supported physically based 3D camera controller modes for `World3D`.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum CameraMode3D {
    /// Dynamic third-person chase camera trailing behind the player vehicle.
    #[default]
    Chase,
    /// Low hood/cockpit driving perspective mounted over the front axle.
    Hood,
    /// Continuous 360-degree orbital turntable camera around the active subject.
    Orbit,
    /// High-altitude aerial overview camera for open-world navigation.
    Aerial,
    /// Unrestricted 6-DOF free-fly camera controlled directly by user input.
    FreeFly,
    /// Smooth cinematic dolly camera gliding along a sinusoidal showcase arc.
    CinematicDolly,
}
