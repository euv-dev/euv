use super::*;

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
