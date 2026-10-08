use super::*;

pub struct CountingComponent {
    pub name: String,
    pub started: Rc<RefCell<u32>>,
    pub updates: Rc<RefCell<u32>>,
    pub destroyed: Rc<RefCell<u32>>,
}

pub struct Recorder {
    pub starts: Rc<Cell<u32>>,
    pub updates: Rc<Cell<u32>>,
    pub renders: Rc<Cell<u32>>,
    pub destroys: Rc<Cell<u32>>,
    pub name: String,
}

pub struct Counters {
    pub starts: Rc<Cell<u32>>,
    pub updates: Rc<Cell<u32>>,
    pub renders: Rc<Cell<u32>>,
    pub destroys: Rc<Cell<u32>>,
}
