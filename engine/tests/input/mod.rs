mod r#fn;

pub use std::{cell::Cell, rc::Rc};

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

use super::*;
