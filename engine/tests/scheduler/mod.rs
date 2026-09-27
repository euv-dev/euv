mod r#fn;

pub use std::{cell::Cell, cell::RefCell, rc::Rc};

use euv_engine::*;

/// A `TickHandler` that records how many update and render callbacks it received.
pub struct CountingHandler {
    /// Number of `on_update` callbacks received.
    pub updates: Rc<Cell<u32>>,
    /// Number of `on_render` callbacks received.
    pub renders: Rc<Cell<u32>>,
}

impl Default for CountingHandler {
    /// Builds a handler with both counters at zero.
    ///
    /// # Returns
    ///
    /// - `CountingHandler` - A handler whose counters start at zero.
    fn default() -> Self {
        Self {
            updates: Rc::new(Cell::new(0)),
            renders: Rc::new(Cell::new(0)),
        }
    }
}

impl CountingHandler {
    /// Builds a boxed handler plus the two counter cells, so tests can read
    /// the counters after the value has been erased behind `dyn TickHandler`.
    ///
    /// # Returns
    ///
    /// - `(TickHandlerRc, Rc<Cell<u32>>, Rc<Cell<u32>>)` - The boxed handler,
    ///   its update counter cell, and its render counter cell.
    pub fn spawn() -> (TickHandlerRc, Rc<Cell<u32>>, Rc<Cell<u32>>) {
        let updates: Rc<Cell<u32>> = Rc::new(Cell::new(0));
        let renders: Rc<Cell<u32>> = Rc::new(Cell::new(0));
        let handler: CountingHandler = Self {
            updates: updates.clone(),
            renders: renders.clone(),
        };
        (Rc::new(EngineCell::new(handler)), updates, renders)
    }
}

impl TickHandler for CountingHandler {
    /// Increments the update counter; the delta is not otherwise used.
    ///
    /// # Arguments
    ///
    /// - `f64` - The fixed delta time in seconds.
    fn on_update(&mut self, delta_time: f64) {
        let _ = delta_time;
        self.updates.set(self.updates.get() + 1);
    }

    /// Increments the render counter; the interpolation is not otherwise used.
    ///
    /// # Arguments
    ///
    /// - `f64` - The interpolation factor.
    fn on_render(&mut self, interpolation: f64) {
        let _ = interpolation;
        self.renders.set(self.renders.get() + 1);
    }
}

/// An `Updatable` that accumulates the total delta time it was driven with.
pub struct DeltaRecorder {
    /// Total delta time accumulated across every `update` call.
    pub total: Rc<Cell<f64>>,
    /// Number of `update` calls received.
    pub calls: Rc<Cell<u32>>,
}

impl DeltaRecorder {
    /// Builds a recorder plus its two observation cells.
    ///
    /// # Arguments
    ///
    /// - `Rc<Cell<f64>>` - Shared cell receiving the accumulated delta time.
    /// - `Rc<Cell<u32>>` - Shared cell receiving the update call count.
    ///
    /// # Returns
    ///
    /// - `DeltaRecorder` - A recorder writing into the supplied cells.
    pub fn new(total: Rc<Cell<f64>>, calls: Rc<Cell<u32>>) -> DeltaRecorder {
        DeltaRecorder { total, calls }
    }
}

impl Updatable for DeltaRecorder {
    /// Accumulates the delta time and increments the call counter.
    ///
    /// # Arguments
    ///
    /// - `f64` - The fixed delta time in seconds.
    fn update(&mut self, delta_time: f64) {
        self.total.set(self.total.get() + delta_time);
        self.calls.set(self.calls.get() + 1);
    }
}

/// An `Updatable` that appends a label to a shared log on every update, so
/// tests can assert the exact traversal order of the task list.
pub struct LoggingTask {
    /// The label written into the shared order log.
    pub label: u8,
    /// The shared order log.
    pub log: Rc<RefCell<Vec<u8>>>,
}

impl Updatable for LoggingTask {
    /// Appends this task's label to the shared order log.
    ///
    /// # Arguments
    ///
    /// - `f64` - The fixed delta time in seconds, unused.
    fn update(&mut self, delta_time: f64) {
        let _ = delta_time;
        self.log.borrow_mut().push(self.label);
    }
}

/// An `Updatable` wrapping a real one-shot [`Timer`], counting how many times
/// the timer actually fired. `Timer` is `Copy`, so a test cannot observe a
/// timer after registering it by value; this adapter keeps both the timer and
/// the observation cell together and reports the count out-of-band.
pub struct FireCounter {
    /// The driven one-shot timer.
    pub timer: Timer,
    /// The shared fire counter.
    pub fires: Rc<Cell<u32>>,
}

impl FireCounter {
    /// Builds a counter driving a one-shot [`Timer`] of `duration` seconds.
    ///
    /// # Arguments
    ///
    /// - `f64` - The timer duration in seconds.
    /// - `Rc<Cell<u32>>` - Shared cell receiving the fire count.
    ///
    /// # Returns
    ///
    /// - `FireCounter` - A counter wrapping the requested timer.
    pub fn new(duration: f64, fires: Rc<Cell<u32>>) -> FireCounter {
        FireCounter {
            timer: Timer::create(duration),
            fires,
        }
    }
}

impl Updatable for FireCounter {
    /// Advances the wrapped timer and counts any fire it reports.
    ///
    /// # Arguments
    ///
    /// - `f64` - The fixed delta time in seconds.
    fn update(&mut self, delta_time: f64) {
        if self.timer.update(delta_time) > 0 {
            self.fires.set(self.fires.get() + 1);
        }
    }
}
