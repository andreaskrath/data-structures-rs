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

    /// Returns a reference to the value of the item.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Returns an `Option` containing a mutable reference to the left child of the item.
    pub fn left(&mut self) -> Option<&mut Self> {
        self.left.as_deref_mut()
    }

    /// Returns an `Option` containing a mutable reference to the right child of the item.
    pub fn right(&mut self) -> Option<&mut Self> {
        self.right.as_deref_mut()
    }
}
