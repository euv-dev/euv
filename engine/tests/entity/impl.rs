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
