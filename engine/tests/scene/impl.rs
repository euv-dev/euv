use super::*;

impl Probe {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            enters: Rc::new(Cell::new(0)),
            exits: Rc::new(Cell::new(0)),
            updates: Rc::new(Cell::new(0)),
        }
    }

    pub fn enters(&self) -> Rc<Cell<u32>> {
        Rc::clone(&self.enters)
    }

    pub fn exits(&self) -> Rc<Cell<u32>> {
        Rc::clone(&self.exits)
    }

    pub fn updates(&self) -> Rc<Cell<u32>> {
        Rc::clone(&self.updates)
    }
}

impl Lifecycle for Probe {
    fn on_update(&mut self, delta_time: f64) {
        let _: f64 = delta_time;
        self.updates.set(self.updates.get() + 1);
    }
}

impl Scene for Probe {
    fn on_enter(&mut self) {
        self.enters.set(self.enters.get() + 1);
    }

    fn on_exit(&mut self) {
        self.exits.set(self.exits.get() + 1);
    }

    fn on_render(&self, draw_list: &mut DrawList) {
        let _: &mut DrawList = draw_list;
    }

    fn name(&self) -> &str {
        &self.name
    }
}
