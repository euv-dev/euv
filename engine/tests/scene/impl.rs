use super::*;

impl Lifecycle for ProbeScene {
    fn on_update(&mut self, delta_time: f64) {
        let mut total: RefMut<'_, f64> = self.updates.borrow_mut();
        *total += delta_time;
    }
}

impl Scene for ProbeScene {
    fn on_enter(&mut self) {
        let mut count: RefMut<'_, u32> = self.entered.borrow_mut();
        *count += 1;
    }

    fn on_exit(&mut self) {
        let mut count: RefMut<'_, u32> = self.exited.borrow_mut();
        *count += 1;
    }

    fn on_render(&self, draw_list: &mut DrawList) {
        let _: &str = &self.name;
        draw_list.fill_circle(Vector2D::zero(), 1.0, Color::white());
    }

    fn name(&self) -> &str {
        &self.name
    }
}
