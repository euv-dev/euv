use super::*;

impl Lifecycle for CountingComponent {
    fn on_update(&mut self, delta_time: f64) {
        let _: f64 = delta_time;
        let mut count: RefMut<'_, u32> = self.updates.borrow_mut();
        *count += 1;
    }
}

impl Component for CountingComponent {
    fn on_start(&mut self) {
        let mut count: RefMut<'_, u32> = self.started.borrow_mut();
        *count += 1;
    }

    fn on_render(&self, draw_list: &mut DrawList, transform: &Transform2D) {
        let _: &Transform2D = transform;
        let position: Vector2D = transform.get_position();
        draw_list.fill_circle(position, 1.0, Color::white());
    }

    fn on_destroy(&mut self) {
        let mut count: RefMut<'_, u32> = self.destroyed.borrow_mut();
        *count += 1;
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Recorder {
    pub fn new(
        starts: Rc<Cell<u32>>,
        updates: Rc<Cell<u32>>,
        renders: Rc<Cell<u32>>,
        destroys: Rc<Cell<u32>>,
        name: String,
    ) -> Self {
        Self {
            starts,
            updates,
            renders,
            destroys,
            name,
        }
    }
}

impl Lifecycle for Recorder {
    fn on_update(&mut self, delta_time: f64) {
        let _: f64 = delta_time;
        self.updates.set(self.updates.get() + 1);
    }
}

impl Component for Recorder {
    fn on_start(&mut self) {
        self.starts.set(self.starts.get() + 1);
    }

    fn on_render(&self, draw_list: &mut DrawList, transform: &Transform2D) {
        let _: &Transform2D = transform;
        let _: &mut DrawList = draw_list;
        self.renders.set(self.renders.get() + 1);
    }

    fn on_destroy(&mut self) {
        self.destroys.set(self.destroys.get() + 1);
    }

    fn name(&self) -> &str {
        &self.name
    }
}
