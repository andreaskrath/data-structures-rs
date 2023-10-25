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
    /// Constructs a new empty `BinaryTree<T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ds_rs::binary_tree::BinaryTree;
    /// # #[allow(unused_mut)]
    /// let mut tree: BinaryTree<i32> = BinaryTree::new();
    /// ```
    #[inline]
    #[must_use]
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
    /// Returns the height of the binary tree.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the value contained within the root element.
    pub fn root(&self) -> Option<&T> {
        match self.root.as_deref() {
            Some(v) => Some(v.value()),
            None => None,
        }
    }

    /// Returns the number of elements in the binary tree.
    pub fn count(&self) -> usize {
        self.count
    }
}
