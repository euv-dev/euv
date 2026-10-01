use super::*;

pub struct CountingComponent {
    pub name: String,
    pub started: Rc<RefCell<u32>>,
    pub updates: Rc<RefCell<u32>>,
    pub destroyed: Rc<RefCell<u32>>,
}
