use super::*;

pub struct ProbeScene {
    pub name: String,
    pub entered: Rc<RefCell<u32>>,
    pub exited: Rc<RefCell<u32>>,
    pub updates: Rc<RefCell<f64>>,
}
