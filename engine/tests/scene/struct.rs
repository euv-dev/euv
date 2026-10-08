use super::*;

pub struct ProbeScene {
    pub name: String,
    pub entered: Rc<RefCell<u32>>,
    pub exited: Rc<RefCell<u32>>,
    pub updates: Rc<RefCell<f64>>,
}

pub struct Probe {
    pub name: String,
    pub enters: Rc<Cell<u32>>,
    pub exits: Rc<Cell<u32>>,
    pub updates: Rc<Cell<u32>>,
}
