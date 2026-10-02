mod r#fn;

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

pub struct DeltaRecorder {
    pub total: Rc<Cell<f64>>,
    pub calls: Rc<Cell<u32>>,
}

impl DeltaRecorder {
    pub fn new(total: Rc<Cell<f64>>, calls: Rc<Cell<u32>>) -> DeltaRecorder {
        DeltaRecorder { total, calls }
    }
}

impl Updatable for DeltaRecorder {
    fn update(&mut self, delta_time: f64) {
        self.total.set(self.total.get() + delta_time);
        self.calls.set(self.calls.get() + 1);
    }
}

pub struct LoggingTask {
    pub label: u8,
    pub log: Rc<RefCell<Vec<u8>>>,
}

impl Updatable for LoggingTask {
    fn update(&mut self, delta_time: f64) {
        let _ = delta_time;
        self.log.borrow_mut().push(self.label);
    }
}

pub struct FireCounter {
    pub timer: Timer,
    pub fires: Rc<Cell<u32>>,
}

impl FireCounter {
    pub fn new(duration: f64, fires: Rc<Cell<u32>>) -> FireCounter {
        FireCounter {
            timer: Timer::create(duration),
            fires,
        }
    }
}

impl Updatable for FireCounter {
    fn update(&mut self, delta_time: f64) {
        if self.timer.update(delta_time) > 0 {
            self.fires.set(self.fires.get() + 1);
        }
    }
}

use super::*;
