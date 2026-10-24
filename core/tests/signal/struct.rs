use super::*;

#[derive(Debug, Data, New)]
pub(crate) struct CountedSignalValue {
    #[get(pub(crate), type(copy))]
    #[set(pub(crate))]
    pub(crate) value: usize,
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(crate) clones: Rc<Cell<usize>>,
}
