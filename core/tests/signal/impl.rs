use super::*;

impl Clone for CountedSignalValue {
    fn clone(&self) -> Self {
        self.get_clones().set(self.get_clones().get() + 1);
        Self::new(self.get_value(), Rc::clone(self.get_clones()))
    }
}

impl PartialEq for CountedSignalValue {
    fn eq(&self, other: &Self) -> bool {
        self.get_value() == other.get_value()
    }
}
