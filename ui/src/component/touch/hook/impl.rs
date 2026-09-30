use super::*;

/// Implementation of touch point extraction from DOM touch events.
impl NativeTouchPoint {
    /// Extracts all active touch points from a `TouchEvent`.
    ///
    /// Iterates over the `touches` list of the given `TouchEvent` and
    /// builds a `Vec<NativeTouchPoint>` with each touch point's
    /// identifier, viewport coordinates, screen coordinates, page
    /// coordinates, and offset coordinates relative to the target element.
    ///
    /// The offset coordinates (`offset_x`, `offset_y`) are computed by
    /// subtracting the target element's bounding rect from the touch's
    /// client coordinates, since the browser `Touch` object does not
    /// provide `offsetX`/`offsetY` directly.
    ///
    /// Uses web-sys typed getters (`TouchEvent::touches()`,
    /// `TouchList::get`, `Touch::client_x()`) instead of
    /// `Reflect::get(event, "clientX")`. The Reflect path allocates a
    /// `JsValue::from_str` per field per touch (7 fields × N touches per
    /// event) on every `touchmove` (60-120Hz); the typed getters skip
    /// the string lookup and the per-field JS string allocation.
    ///
    /// # Arguments
    ///
    /// - `&Event` - The native DOM touch event.
    ///
    /// # Returns
    ///
    /// - `Vec<NativeTouchPoint>` - All currently active touch points.
    pub fn extract_all(event: &Event) -> Vec<NativeTouchPoint> {
        let touch_event: &TouchEvent = event.unchecked_ref::<TouchEvent>();
        let touches: TouchList = touch_event.touches();
        let target: JsValue = event
            .target()
            .map_or(JsValue::NULL, |event_target: EventTarget| {
                event_target.into()
            });
        let element: Element = target.unchecked_into();
        let rect: DomRect = element.get_bounding_client_rect();
        let rect_left: f64 = rect.left();
        let rect_top: f64 = rect.top();
        let length: u32 = touches.length();
        (0..length)
            .filter_map(|index: u32| touches.get(index))
            .map(|touch: Touch| {
                let identifier: i32 = touch.identifier();
                let client_x: i32 = touch.client_x();
                let client_y: i32 = touch.client_y();
                let screen_x: i32 = touch.screen_x();
                let screen_y: i32 = touch.screen_y();
                let page_x: i32 = touch.page_x();
                let page_y: i32 = touch.page_y();
                let offset_x: i32 = (client_x as f64 - rect_left).round() as i32;
                let offset_y: i32 = (client_y as f64 - rect_top).round() as i32;
                NativeTouchPoint {
                    identifier,
                    client_x,
                    client_y,
                    screen_x,
                    screen_y,
                    offset_x,
                    offset_y,
                    page_x,
                    page_y,
                }
            })
            .collect()
    }

    /// Extracts the changed touch points from a `TouchEvent`.
    ///
    /// The `changedTouches` list contains touch points that have changed
    /// since the last touch event:
    /// - For `touchstart` - newly added touch points.
    /// - For `touchmove` - touch points that have moved.
    /// - For `touchend` / `touchcancel` - removed touch points.
    ///
    /// This is useful for determining which specific fingers were lifted
    /// in a `touchend` event, since the `touches` list no longer contains
    /// them.
    ///
    /// Uses web-sys typed getters (`TouchEvent::changed_touches()`,
    /// `TouchList::get`, `Touch::client_x()`) to avoid the per-field
    /// `Reflect::get` + `JsValue::from_str` allocation cost on the hot
    /// `touchmove` path.
    ///
    /// # Arguments
    ///
    /// - `&Event` - The native DOM touch event.
    ///
    /// # Returns
    ///
    /// - `Vec<NativeTouchPoint>` - The touch points that changed in this event.
    pub fn extract_changed(event: &Event) -> Vec<NativeTouchPoint> {
        let touch_event: &TouchEvent = event.unchecked_ref::<TouchEvent>();
        let touches: TouchList = touch_event.changed_touches();
        let target: JsValue = event
            .target()
            .map_or(JsValue::NULL, |event_target: EventTarget| {
                event_target.into()
            });
        let element: Element = target.unchecked_into();
        let rect: DomRect = element.get_bounding_client_rect();
        let rect_left: f64 = rect.left();
        let rect_top: f64 = rect.top();
        let length: u32 = touches.length();
        (0..length)
            .filter_map(|index: u32| touches.get(index))
            .map(|touch: Touch| {
                let identifier: i32 = touch.identifier();
                let client_x: i32 = touch.client_x();
                let client_y: i32 = touch.client_y();
                let screen_x: i32 = touch.screen_x();
                let screen_y: i32 = touch.screen_y();
                let page_x: i32 = touch.page_x();
                let page_y: i32 = touch.page_y();
                let offset_x: i32 = (client_x as f64 - rect_left).round() as i32;
                let offset_y: i32 = (client_y as f64 - rect_top).round() as i32;
                NativeTouchPoint {
                    identifier,
                    client_x,
                    client_y,
                    screen_x,
                    screen_y,
                    offset_x,
                    offset_y,
                    page_x,
                    page_y,
                }
            })
            .collect()
    }
}

/// Implementation of high-precision touch point extraction from DOM touch events.
impl NativeTouchPointF64 {
    /// Extracts all active touch points with high-precision `f64` offset coordinates
    /// from a `TouchEvent`.
    ///
    /// Similar to `NativeTouchPoint::extract_all`, but returns `f64` precision for
    /// offset/client coordinates, which is essential for canvas drawing
    /// and other pixel-precise interactions.
    ///
    /// Uses web-sys typed getters (`TouchEvent::touches()`,
    /// `TouchList::get`, `Touch::client_x()`) instead of
    /// `Reflect::get(event, "clientX")`. web-sys `Touch` exposes
    /// `client_x`/`page_x`/etc. as `i32`; we widen to `f64` to preserve
    /// the `NativeTouchPointF64` high-precision contract without losing
    /// sub-pixel information on the offset computation.
    ///
    /// # Arguments
    ///
    /// - `&Event` - The native DOM touch event.
    ///
    /// # Returns
    ///
    /// - `Vec<NativeTouchPointF64>` - All currently active touch points with `f64` coordinates.
    pub fn extract_all(event: &Event) -> Vec<NativeTouchPointF64> {
        let touch_event: &TouchEvent = event.unchecked_ref::<TouchEvent>();
        let touches: TouchList = touch_event.touches();
        let target: JsValue = event
            .target()
            .map_or(JsValue::NULL, |event_target: EventTarget| {
                event_target.into()
            });
        let element: Element = target.unchecked_into();
        let rect: DomRect = element.get_bounding_client_rect();
        let rect_left: f64 = rect.left();
        let rect_top: f64 = rect.top();
        let length: u32 = touches.length();
        (0..length)
            .filter_map(|index: u32| touches.get(index))
            .map(|touch: Touch| {
                let identifier: i32 = touch.identifier();
                let client_x: f64 = touch.client_x() as f64;
                let client_y: f64 = touch.client_y() as f64;
                let screen_x: f64 = touch.screen_x() as f64;
                let screen_y: f64 = touch.screen_y() as f64;
                let page_x: f64 = touch.page_x() as f64;
                let page_y: f64 = touch.page_y() as f64;
                let offset_x: f64 = client_x - rect_left;
                let offset_y: f64 = client_y - rect_top;
                NativeTouchPointF64 {
                    identifier,
                    client_x,
                    client_y,
                    screen_x,
                    screen_y,
                    offset_x,
                    offset_y,
                    page_x,
                    page_y,
                }
            })
            .collect()
    }
}

/// Implementation of gesture recognition over raw touch points.
///
/// The recognizer is a pure state machine: it is fed touch point lists and
/// timestamps and reports what it decides. It never touches the DOM itself,
/// which keeps the classification rules testable without a browser.
impl EuvGestureRecognizer {
    /// Creates a recognizer with the default thresholds.
    ///
    /// Equivalent to [`EuvGestureRecognizer::default`]; the inherent form
    /// exists so callers read as `EuvGestureRecognizer::new()` rather than
    /// reaching for the trait.
    ///
    /// # Returns
    ///
    /// - `EuvGestureRecognizer` - A recognizer using
    ///   [`EuvGestureConfig::default`].
    pub fn new() -> EuvGestureRecognizer {
        Self::default()
    }

    /// Creates a recognizer with caller-supplied thresholds.
    ///
    /// # Arguments
    ///
    /// - `EuvGestureConfig` - The thresholds to classify against.
    ///
    /// # Returns
    ///
    /// - `EuvGestureRecognizer` - A recognizer using the given thresholds.
    pub fn with_config(config: EuvGestureConfig) -> EuvGestureRecognizer {
        EuvGestureRecognizer { config }
    }

    /// Mounts the reactive signals and returns the handlers that drive them.
    ///
    /// Wire the returned handlers to a single element in `html!`:
    ///
    /// ```ignore
    /// let state: EuvGestureState = EuvGestureRecognizer::new().use_gesture();
    /// div {
    ///     ontouchstart: state.on_start
    ///     ontouchmove: state.on_move
    ///     ontouchend: state.on_end
    ///     ontouchcancel: state.on_cancel
    /// }
    /// ```
    ///
    /// Per-move bookkeeping lives in a non-reactive cell so a 120Hz
    /// `touchmove` does not allocate; only `last_gesture` is written when a
    /// gesture completes.
    ///
    /// A long press is resolved on `touchend` from the measured elapsed time
    /// rather than by a timer, so a press that is released after the
    /// threshold still reports `LongPress` and a cancelled press reports
    /// nothing.
    ///
    /// # Returns
    ///
    /// - `EuvGestureState` - The reactive signals plus the four handlers.
    pub fn use_gesture(self) -> EuvGestureState {
        let last_gesture: Signal<Option<EuvGesture>> = App::use_signal(|| None);
        let drag: Signal<Option<EuvDrag>> = App::use_signal(|| None);
        let pinch: Signal<Option<EuvPinch>> = App::use_signal(|| None);
        let progress: Rc<RefCell<GestureProgress>> =
            Rc::new(RefCell::new(GestureProgress::default()));
        let recognizer: EuvGestureRecognizer = self;

        let start_progress: Rc<RefCell<GestureProgress>> = Rc::clone(&progress);
        let start_gesture: Signal<Option<EuvGesture>> = last_gesture;
        let start_drag: Signal<Option<EuvDrag>> = drag;
        let start_pinch: Signal<Option<EuvPinch>> = pinch;
        let on_start: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |event: Event| {
            Self::suppress_page_scroll(&event);
            let points: Vec<NativeTouchPoint> = NativeTouchPoint::extract_all(&event);
            if points.is_empty() {
                return;
            }
            // All four handlers share this one cell, and the browser dispatches
            // touch events independently of one another, so a `touchstart` can
            // land while a `touchmove` frame still holds the guard. A contended
            // begin is skipped: the next `touchstart` re-seeds every field, so
            // the only loss is the current sequence's origin.
            if let Ok(mut slot) = start_progress.try_borrow_mut() {
                slot.begin(&points);
            }
            start_gesture.set(None);
            start_drag.set(None);
            if let Some(reading) = recognizer.pinch_from(&points, recognizer.config.swipe_threshold)
            {
                start_pinch.set(Some(reading));
            }
        }));

        let move_progress: Rc<RefCell<GestureProgress>> = Rc::clone(&progress);
        let move_drag: Signal<Option<EuvDrag>> = drag;
        let move_pinch: Signal<Option<EuvPinch>> = pinch;
        let move_recognizer: EuvGestureRecognizer = self;
        let on_move: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |event: Event| {
            Self::suppress_page_scroll(&event);
            let points: Vec<NativeTouchPoint> = NativeTouchPoint::extract_all(&event);
            if points.is_empty() {
                return;
            }
            // Read everything out of the cell, drop the guard, and only then
            // touch the signals: a `Signal::set` runs listeners synchronously and
            // a listener may re-enter this handler, and borrowing across it would
            // abort the WASM instance with an already-borrowed panic. A contended
            // cell drops this frame's reading; the next `touchmove` at 60-120Hz
            // recomputes it, so the drag and pinch signals are one frame stale
            // rather than the app being aborted.
            let Ok(mut slot) = move_progress.try_borrow_mut() else {
                return;
            };
            slot.advance(&points);
            let readings: (Option<EuvDrag>, Option<EuvPinch>) = {
                let drag_reading: Option<EuvDrag> = slot.primary().map(|point: GesturePoint| {
                    move_recognizer.drag_from(
                        &point,
                        slot.get_start_x(),
                        slot.get_start_y(),
                        *slot.get_travel(),
                    )
                });
                let pinch_reading: Option<EuvPinch> = move_recognizer
                    .pinch_from(&points, slot.get_pinch_start_distance())
                    .filter(|candidate: &EuvPinch| {
                        move_recognizer.pinch_is_significant(
                            candidate.distance,
                            slot.get_pinch_start_distance(),
                        )
                    });
                (drag_reading, pinch_reading)
            };
            drop(slot);
            if let Some(reading) = readings.0 {
                move_drag.set(Some(reading));
            }
            if let Some(reading) = readings.1 {
                move_pinch.set(Some(reading));
            }
        }));

        let end_progress: Rc<RefCell<GestureProgress>> = Rc::clone(&progress);
        let end_gesture: Signal<Option<EuvGesture>> = last_gesture;
        let end_drag: Signal<Option<EuvDrag>> = drag;
        let end_pinch: Signal<Option<EuvPinch>> = pinch;
        let end_recognizer: EuvGestureRecognizer = self;
        let on_end: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |_: Event| {
            // `finish` reads the clock through `now_millis`, so the guard must
            // not outlive this block. A contended cell reports "no gesture" and
            // the next `touchstart` re-seeds the cell.
            let outcome: Option<EuvGesture> = match end_progress.try_borrow_mut() {
                Ok(mut slot) => {
                    let result: Option<EuvGesture> = slot.finish(&end_recognizer.config);
                    slot.reset();
                    result
                }
                Err(_) => None,
            };
            end_drag.set(None);
            end_pinch.set(None);
            if let Some(gesture) = outcome {
                end_gesture.set(Some(gesture));
            }
        }));

        let cancel_progress: Rc<RefCell<GestureProgress>> = Rc::clone(&progress);
        let cancel_drag: Signal<Option<EuvDrag>> = drag;
        let cancel_pinch: Signal<Option<EuvPinch>> = pinch;
        let on_cancel: Option<Rc<dyn Fn(Event)>> = Some(Rc::new(move |_: Event| {
            // Best-effort reset. A contended cell means a `touchmove` frame is
            // mid-flight and will read the same cell on its way out; the reset
            // is recomputed by the next `touchstart`, so skipping it costs one
            // stale gesture. `borrow_mut` would abort the instance.
            if let Ok(mut slot) = cancel_progress.try_borrow_mut() {
                slot.reset();
            }
            cancel_drag.set(None);
            cancel_pinch.set(None);
        }));

        EuvGestureState {
            last_gesture,
            drag,
            pinch,
            on_start,
            on_move,
            on_end,
            on_cancel,
        }
    }

    /// Calls `prevent_default` on a touch event so the browser does not also
    /// scroll or zoom the page under a gesture.
    ///
    /// Guarded on `cancelable` because a passive or already-dispatched event
    /// rejects the call.
    ///
    /// # Arguments
    ///
    /// - `&Event` - The touch event to cancel default handling on.
    fn suppress_page_scroll(event: &Event) {
        if event.cancelable() {
            event.prevent_default();
        }
    }

    /// Decides which single-finger gesture, if any, a finished touch
    /// represents.
    ///
    /// A touch is a tap or long press only when the straight-line distance
    /// from start to end is within `tap_slop`; that is deliberately separate
    /// from the path length a drag accumulates, so a finger that wanders in
    /// a small loop is not mistaken for a tap. Otherwise the dominant axis
    /// of the displacement wins, and the gesture only fires past
    /// `swipe_threshold`.
    ///
    /// # Arguments
    ///
    /// - `f64` - X displacement from start to end, in CSS pixels.
    /// - `f64` - Y displacement from start to end, in CSS pixels.
    /// - `f64` - Elapsed time from `touchstart` to `touchend`, in
    ///   milliseconds.
    /// - `f64` - Path length travelled, in CSS pixels. A finger that loops
    ///   back to its origin has near-zero displacement but a long path, and
    ///   must not be classified as a tap.
    ///
    /// # Returns
    ///
    /// - `Option<EuvGesture>` - The recognized gesture, or `None` when the
    ///   movement is too small to be either a tap or a swipe.
    pub fn classify(
        &self,
        dx: f64,
        dy: f64,
        elapsed_millis: f64,
        travel: f64,
    ) -> Option<EuvGesture> {
        let distance: f64 = (dx * dx + dy * dy).sqrt();
        if distance <= self.get_config().get_tap_slop()
            && travel <= self.get_config().get_tap_slop()
        {
            if elapsed_millis >= self.get_config().get_long_press_millis() {
                return Some(EuvGesture::LongPress);
            }
            return Some(EuvGesture::Tap);
        }
        if distance < self.get_config().get_swipe_threshold() {
            return None;
        }
        if dx.abs() >= dy.abs() {
            Some(if dx > 0.0 {
                EuvGesture::Right
            } else {
                EuvGesture::Left
            })
        } else {
            Some(if dy > 0.0 {
                EuvGesture::Down
            } else {
                EuvGesture::Up
            })
        }
    }

    /// Builds the two-finger pinch description for the current touch points.
    ///
    /// Returns `None` unless exactly two points are supplied, so a
    /// one-finger drag is never mistaken for a degenerate pinch.
    ///
    /// # Arguments
    ///
    /// - `&[NativeTouchPoint]` - The active touch points, which must hold
    ///   exactly two entries.
    /// - `f64` - The distance recorded when the pinch began, used as the
    ///   baseline for `delta`.
    ///
    /// # Returns
    ///
    /// - `Option<EuvPinch>` - The pinch description, or `None` when the point
    ///   count is not two.
    pub fn pinch_from(&self, points: &[NativeTouchPoint], start_distance: f64) -> Option<EuvPinch> {
        if points.len() != 2 {
            return None;
        }
        let first: &NativeTouchPoint = &points[0];
        let second: &NativeTouchPoint = &points[1];
        let dx: f64 = f64::from(second.client_x) - f64::from(first.client_x);
        let dy: f64 = f64::from(second.client_y) - f64::from(first.client_y);
        let distance: f64 = (dx * dx + dy * dy).sqrt();
        Some(EuvPinch {
            distance,
            start_distance,
            center_x: f64::from(first.client_x + second.client_x) / 2.0,
            center_y: f64::from(first.client_y + second.client_y) / 2.0,
            delta: distance - start_distance,
        })
    }

    /// Decides whether a pinch has grown or shrunk far enough to be worth
    /// reporting.
    ///
    /// Comparing the current distance against the pinch baseline as a ratio
    /// keeps the threshold meaningful at any zoom level, where an absolute
    /// pixel delta would be noise when zoomed out and too small when zoomed
    /// in.
    ///
    /// # Arguments
    ///
    /// - `f64` - Distance between the two fingers now.
    /// - `f64` - Distance between the two fingers when the pinch began.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when the relative change exceeds
    ///   `pinch_threshold`.
    pub fn pinch_is_significant(&self, distance: f64, start_distance: f64) -> bool {
        if start_distance <= 0.0 {
            return false;
        }
        (distance - start_distance).abs() / start_distance
            >= self.get_config().get_pinch_threshold()
    }

    /// Builds the single-finger drag description for the current point.
    ///
    /// # Arguments
    ///
    /// - `&GesturePoint` - The finger that is moving.
    /// - `f64` - X of the point where this drag began.
    /// - `f64` - Y of the point where this drag began.
    /// - `f64` - Path length accumulated so far, in CSS pixels.
    ///
    /// # Returns
    ///
    /// - `EuvDrag` - The drag description for this move.
    pub fn drag_from(
        &self,
        point: &GesturePoint,
        start_x: f64,
        start_y: f64,
        travel: f64,
    ) -> EuvDrag {
        let x: f64 = point.client_x;
        let y: f64 = point.client_y;
        EuvDrag {
            x,
            y,
            delta_x: x - start_x,
            delta_y: y - start_y,
            travel,
        }
    }
}

impl EuvGesture {
    /// Returns the stable lowercase token for this gesture.
    ///
    /// Used for telemetry and `data-gesture` attributes, so the spelling is
    /// part of the public contract and must not change between releases.
    ///
    /// # Returns
    ///
    /// - `&'static str` - The wire name: one of `left`, `right`, `up`,
    ///   `down`, `tap`, or `long-press`.
    pub fn name(self) -> &'static str {
        let index: usize = match self {
            Self::Left => 0,
            Self::Right => 1,
            Self::Up => 2,
            Self::Down => 3,
            Self::Tap => 4,
            Self::LongPress => 5,
        };
        GESTURE_NAMES[index]
    }
}
impl Default for EuvGestureConfig {
    /// Returns the default thresholds, tuned for a finger on glass.
    ///
    /// 48px is roughly the width of an adult fingertip contact patch, so a
    /// smaller travel is treated as jitter rather than intent. 500ms is the
    /// conventional long-press boundary used by both iOS and Android.
    ///
    /// # Returns
    ///
    /// - `EuvGestureConfig` - The default threshold set.
    fn default() -> Self {
        Self {
            swipe_threshold: 48.0,
            tap_slop: 10.0,
            long_press_millis: 500.0,
            pinch_threshold: 0.01,
        }
    }
}

impl Default for EuvGestureRecognizer {
    /// Returns a recognizer using the default thresholds.
    ///
    /// # Returns
    ///
    /// - `EuvGestureRecognizer` - A recognizer using
    ///   [`EuvGestureConfig::default`].
    fn default() -> Self {
        Self {
            config: EuvGestureConfig::default(),
        }
    }
}

impl GestureProgress {
    /// Records the origin of a new touch sequence.
    ///
    /// When two or more fingers land together, the inter-finger distance is
    /// captured as the pinch baseline, so a later spread is measured against
    /// the initial separation rather than against zero.
    ///
    /// # Arguments
    ///
    /// - `&[NativeTouchPoint]` - The touch points active at `touchstart`.
    pub fn begin(&mut self, points: &[NativeTouchPoint]) {
        let Some(first) = points.first() else {
            return;
        };
        self.set_start_x(f64::from(first.client_x));
        self.set_start_y(f64::from(first.client_y));
        self.set_last_x(self.get_start_x());
        self.set_last_y(self.get_start_y());
        self.set_travel(0.0);
        self.set_started_at(now_millis());
        self.set_active(true);
        let baseline: f64 = match points {
            [one, two, ..] => {
                let dx: f64 = f64::from(two.client_x - one.client_x);
                let dy: f64 = f64::from(two.client_y - one.client_y);
                (dx * dx + dy * dy).sqrt()
            }
            _ => 0.0,
        };
        self.set_pinch_start_distance(baseline);
    }

    /// Folds a `touchmove` reading into the running totals.
    ///
    /// Travel is measured segment by segment rather than from the origin so
    /// that a finger drawing a closed loop accumulates a large distance,
    /// which is what separates a drag from a wandering tap.
    ///
    /// # Arguments
    ///
    /// - `&[NativeTouchPoint]` - The touch points active at `touchmove`.
    pub fn advance(&mut self, points: &[NativeTouchPoint]) {
        let Some(first) = points.first() else {
            return;
        };
        let x: f64 = f64::from(first.client_x);
        let y: f64 = f64::from(first.client_y);
        let step_x: f64 = x - self.get_last_x();
        let step_y: f64 = y - self.get_last_y();
        self.set_travel(self.get_travel() + (step_x * step_x + step_y * step_y).sqrt());
        self.set_last_x(x);
        self.set_last_y(y);
    }

    /// Returns the first tracked finger, or `None` when no sequence is
    /// active.
    ///
    ///
    /// # Returns
    ///
    /// - `Option<GesturePoint>` - The live position of the tracked finger,
    ///   or `None` when no sequence is in flight.
    pub fn primary(&self) -> Option<GesturePoint> {
        if !self.get_active() {
            return None;
        }
        Some(GesturePoint {
            client_x: self.get_last_x(),
            client_y: self.get_last_y(),
        })
    }

    /// Classifies the finished sequence and marks the cell inactive.
    ///
    /// A tap requires both a small straight-line displacement and a short
    /// accumulated path. Checking only the displacement would misread a
    /// finger that traces a closed loop and returns to its origin as a tap,
    /// even though it travelled hundreds of pixels; the path length is what
    /// separates the two cases.
    ///
    /// # Arguments
    ///
    /// - `&EuvGestureConfig` - The thresholds to classify against.
    ///
    /// # Returns
    ///
    /// - `Option<EuvGesture>` - The recognised gesture, or `None` when no
    ///   sequence was in flight.
    pub fn finish(&mut self, config: &EuvGestureConfig) -> Option<EuvGesture> {
        if !self.get_active() {
            return None;
        }
        self.set_active(false);
        let dx: f64 = self.get_last_x() - self.get_start_x();
        let dy: f64 = self.get_last_y() - self.get_start_y();
        let elapsed: f64 = now_millis() - self.get_started_at();
        let distance: f64 = (dx * dx + dy * dy).sqrt();
        if distance <= config.tap_slop && *self.get_travel() <= config.tap_slop {
            return Some(if elapsed >= config.long_press_millis {
                EuvGesture::LongPress
            } else {
                EuvGesture::Tap
            });
        }
        if distance < config.swipe_threshold {
            return None;
        }
        Some(if dx.abs() >= dy.abs() {
            if dx > 0.0 {
                EuvGesture::Right
            } else {
                EuvGesture::Left
            }
        } else if dy > 0.0 {
            EuvGesture::Down
        } else {
            EuvGesture::Up
        })
    }

    /// Clears all progress, used when a sequence is cancelled.
    ///
    /// # Arguments
    ///
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
