#[derive(Default, Debug, Clone, PartialEq)]
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

    pub fn insert(&mut self, value: T) {
        use std::cmp::Ordering as Ord;

        match self.root.as_deref_mut() {
            Some(mut root) => {
                // An empty tree is height 0, while a tree with only a root is height 1
                // meaning this arm, which is entered when root is not empty
                // automatically starts at level 2 in terms of height.
                let mut level: usize = 2;
                loop {
                    match (root.left(), root.right()) {
                        (None, None) => {
                            match value.partial_cmp(root.value()).unwrap() {
                                Ord::Equal => return,
                                Ord::Less => {
                                    let node = Item::new(value);
                                    root.set_left(node);
                                }
                                Ord::Greater => {
                                    let node = Item::new(value);
                                    root.set_right(node);
                                }
                            }
                            self.height = level;
                            self.count += 1;
                            return;
                        }
                        (None, Some(r)) => todo!(),
                        (Some(l), None) => todo!(),
                        (Some(l), Some(r)) => todo!(),
                    }

                    level += 1;
                }
            }
            None => {
                self.root = Some(Box::new(Item::new(value)));
                self.count = 1;
                self.height = 1;
            }
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

#[derive(Debug, Default, Clone, PartialEq)]
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

    /// Returns an `Option` containing a reference to the left child of the item.
    pub fn left(&self) -> Option<&Self> {
        self.left.as_deref()
    }

    /// Returns an `Option` containing a reference to the right child of the item.
    pub fn right(&self) -> Option<&Self> {
        self.right.as_deref()
    }

    /// Returns an `Option` containing a mutable reference to the left child of the item.
    pub fn left_mut(&mut self) -> Option<&mut Self> {
        self.left.as_deref_mut()
    }

    /// Returns an `Option` containing a mutable reference to the right child of the item.
    pub fn right_mut(&mut self) -> Option<&mut Self> {
        self.right.as_deref_mut()
    }

    pub fn set_left(&mut self, value: T) {
        self.left = Some(Box::new(Item::new(value)))
    }

    pub fn set_right(&mut self, value: T) {
        self.right = Some(Box::new(Item::new(value)))
    }
}

#[cfg(test)]
mod binary_tree_insert {
    use super::{BinaryTree, Item};

    #[test]
    fn insert_one_element_that_becomes_root() {
        let mut tree = BinaryTree::new();
        let expected = BinaryTree {
            root: Some(Box::new(Item {
                value: 5,
                left: None,
                right: None,
            })),
            count: 1,
            height: 1,
        };
        tree.insert(5);
        assert_eq!(tree, expected);
    }

    #[test]
    fn inserts_two_elements_second_is_left_child() {
        let mut tree = BinaryTree::new();
        let expected = BinaryTree {
            root: Some(Box::new(Item {
                value: 5,
                left: Some(Box::new(Item {
                    value: 4,
                    left: None,
                    right: None,
                })),
                right: None,
            })),
            count: 2,
            height: 2,
        };
        tree.insert(5);
        tree.insert(4);
        assert_eq!(tree, expected);
    }

    #[test]
    fn inserts_two_elements_second_is_right_child() {
        let mut tree = BinaryTree::new();
        let expected = BinaryTree {
            root: Some(Box::new(Item {
                value: 5,
                left: None,
                right: Some(Box::new(Item {
                    value: 6,
                    left: None,
                    right: None,
                })),
            })),
            count: 2,
            height: 2,
        };
        tree.insert(5);
        tree.insert(6);
        assert_eq!(tree, expected);
    }
}
