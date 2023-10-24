use crate::shared::Item;

#[derive(Default, Debug, Clone)]
pub struct BinaryTree<T>
where
    T: PartialOrd,
{
    root: Option<Box<Item<T>>>,
    count: usize,
    height: usize,
}

impl<T: PartialOrd> BinaryTree<T> {
    pub fn new() -> Self {
        Self {
            root: None,
            count: 0,
            height: 0,
        }
    }

    /// Returns `true` if the binary tree contains no elements.
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// Clears the binary tree of all elements.
    pub fn clear(&mut self) {
        self.root = None;
        self.count = 0;
        self.height = 0;
    }
}
