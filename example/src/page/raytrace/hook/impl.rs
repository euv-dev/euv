use super::*;

/// Default constructor for `RayTraceCameraAngles`: a slight downward
/// look (pitch 0.25) so the ground AABB is visible in the first frame.
impl RayTraceCameraAngles {
    /// Creates a default `RayTraceCameraAngles` with sensible starting
    /// values: a slight downward look (pitch 0.25) so the ground AABB
    /// is visible in the first frame.
    ///
    /// # Returns
    ///
    /// - `RayTraceCameraAngles` - The new camera angles.
    pub(crate) fn default() -> RayTraceCameraAngles {
        RayTraceCameraAngles {
            yaw: Rc::new(Cell::new(0.6)),
            pitch: Rc::new(Cell::new(0.25)),
        }
    }
}
