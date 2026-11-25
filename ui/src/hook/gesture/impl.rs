use super::*;

/// Gesture tracking and classification methods for `GestureRecognizer`.
impl GestureRecognizer {
    /// Begins tracking a gesture at `point`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable recognizer.
    /// - `Point` - Contact start coordinates.
    pub fn begin(&mut self, point: Point) {
        self.set_active(true);
        self.set_start(point);
        self.set_last(point);
        self.set_total_path(0.0);
    }

    /// Records an intermediate motion sample at `point`, accumulating total path length.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable recognizer.
    /// - `Point` - Sampled motion coordinates.
    pub fn update(&mut self, point: Point) {
        if !self.get_active() {
            return;
        }
        let dx: f32 = point.get_x() - self.get_last().get_x();
        let dy: f32 = point.get_y() - self.get_last().get_y();
        let step: f32 = (dx * dx + dy * dy).sqrt();
        self.set_total_path(self.get_total_path() + step);
        self.set_last(point);
    }

    /// Ends the stroke at `point` and classifies the completed `GestureKind`.
    ///
    /// # Arguments
    ///
    /// - `&Self` - Mutable recognizer.
    /// - `Point` - Release coordinates.
    ///
    /// # Returns
    ///
    /// - `Option<GestureKind>` - Classified gesture if a stroke was active.
    pub fn end(&mut self, point: Point) -> Option<GestureKind> {
        if !self.get_active() {
            return None;
        }
        self.update(point);
        self.set_active(false);
        let dx: f32 = point.get_x() - self.get_start().get_x();
        let dy: f32 = point.get_y() - self.get_start().get_y();
        let net: f32 = (dx * dx + dy * dy).sqrt();
        let total: f32 = self.get_total_path();
        if net <= TAP_MAX_DISPLACEMENT_PX && total <= TAP_MAX_PATH_LENGTH_PX {
            return Some(GestureKind::Tap);
        }
        if net >= SWIPE_MIN_DISTANCE_PX {
            if dx.abs() >= dy.abs() {
                if dx > 0.0 {
                    return Some(GestureKind::SwipeRight);
                }
                return Some(GestureKind::SwipeLeft);
            }
            if dy > 0.0 {
                return Some(GestureKind::SwipeDown);
            }
            return Some(GestureKind::SwipeUp);
        }
        Some(GestureKind::Drag)
    }
}
