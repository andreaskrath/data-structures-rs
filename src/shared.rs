#[derive(Debug, Default, Clone)]
pub struct Item<T> {
    value: T,
    left: Option<Box<Item<T>>>,
    right: Option<Box<Item<T>>>,
}

impl<T: PartialOrd> Item<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn left(&mut self) -> Option<&mut Self> {
        self.left.as_deref_mut()
    }

    pub fn right(&mut self) -> Option<&mut Self> {
        self.right.as_deref_mut()
    }
}
