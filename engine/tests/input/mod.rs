mod r#fn;

use std::{cell::Cell, rc::Rc};
pub struct CountingHandler {
    pub updates: Rc<Cell<u32>>,
    pub renders: Rc<Cell<u32>>,
}

impl Default for CountingHandler {
    fn default() -> Self {
        Self {
            updates: Rc::new(Cell::new(0)),
            renders: Rc::new(Cell::new(0)),
        }
    }
}

impl CountingHandler {
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
    fn on_update(&mut self, delta_time: f64) {
        let _ = delta_time;
        self.updates.set(self.updates.get() + 1);
    }

    fn on_render(&mut self, interpolation: f64) {
        let _ = interpolation;
        self.renders.set(self.renders.get() + 1);
    }
}

use super::*;
