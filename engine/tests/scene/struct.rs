use super::*;

pub struct Probe {
    pub name: String,
    pub enters: Rc<Cell<u32>>,
    pub exits: Rc<Cell<u32>>,
    pub updates: Rc<Cell<u32>>,
}
