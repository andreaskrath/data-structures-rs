#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
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

    /// Inserts the provided value into the `BinaryTree`,
    /// and preserves the properties of the binary tree.
    ///
    /// # Panic
    /// The function will panic if a comparison of elements is impossible with the [`PartialOrd`] trait.
    ///
    /// # Examples
    /// ```
    /// # use ds_rs::binary_tree::BinaryTree;
    /// let mut tree = BinaryTree::new();
    /// tree.insert(5);
    /// assert_eq!(tree.root(), Some(&5));
    /// assert_eq!(tree.height(), 1);
    /// assert_eq!(tree.count(), 1);
    /// ```
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
                                Ord::Less => root.set_left(value),
                                Ord::Greater => root.set_right(value),
                            }
                            self.height = level;
                            self.count += 1;
                            return;
                        }
                        (None, Some(_)) => match value.partial_cmp(root.value()).unwrap() {
                            Ord::Equal => return,
                            Ord::Less => {
                                root.set_left(value);
                                self.height = level;
                                self.count += 1;
                                return;
                            }
                            Ord::Greater => root = root.right_mut().unwrap(),
                        },
                        (Some(_), None) => match value.partial_cmp(root.value()).unwrap() {
                            Ord::Equal => return,
                            Ord::Less => root = root.left_mut().unwrap(),
                            Ord::Greater => {
                                root.set_right(value);
                                self.height = level;
                                self.count += 1;
                                return;
                            }
                        },
                        (Some(_), Some(_)) => match value.partial_cmp(root.value()).unwrap() {
                            Ord::Equal => return,
                            Ord::Less => root = root.left_mut().unwrap(),
                            Ord::Greater => root = root.right_mut().unwrap(),
                        },
                    }

                    level += 1;
                }
            }
            None => {
                // This ensures that root is not an imcomparable value.
                _ = value.partial_cmp(&value).unwrap();

                self.root = Some(Box::new(Item::new(value)));
                self.count = 1;
                self.height = 1;
            }
        }
    }

    /// Returns `true` if the binary tree contains no elements.
    ///
    /// # Examples
    /// ```
    /// # use ds_rs::binary_tree::BinaryTree;
    /// let mut tree = BinaryTree::new();
    /// assert!(tree.is_empty());
    ///
    /// tree.insert(0);
    /// assert!(!tree.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// Clears the binary tree of all elements.
    ///
    /// # Examples
    /// ```
    /// # use ds_rs::binary_tree::BinaryTree;
    /// let mut tree = BinaryTree::new();
    /// assert!(tree.is_empty());
    ///
    /// tree.insert(0);
    /// assert!(!tree.is_empty());
    ///
    /// tree.clear();
    /// assert!(tree.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.root = None;
        self.count = 0;
        self.height = 0;
    }

    /// Returns the height of the binary tree.
    ///
    /// An empty tree has a height of `0`.
    ///
    /// # Examples
    /// ```
    /// # use ds_rs::binary_tree::BinaryTree;
    /// let mut tree = BinaryTree::new();
    /// assert_eq!(tree.height(), 0);
    ///
    /// // inserting a root makes the height 1
    /// tree.insert(0);
    /// assert_eq!(tree.height(), 1);
    ///
    /// // adding a child to the root increases the height
    /// tree.insert(-1);
    /// assert_eq!(tree.height(), 2);
    ///
    /// // this operation only adds a child on the same level
    /// // which does not increase the height
    /// tree.insert(1);
    /// assert_eq!(tree.height(), 2);
    /// ```
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns the value contained within the root element.
    ///
    /// # Examples
    /// ```
    /// # use ds_rs::binary_tree::BinaryTree;
    /// let mut tree = BinaryTree::new();
    /// assert_eq!(tree.root(), None);
    ///
    /// tree.insert(5);
    /// assert_eq!(tree.root(), Some(&5));
    /// ```
    pub fn root(&self) -> Option<&T> {
        match self.root.as_deref() {
            Some(v) => Some(v.value()),
            None => None,
        }
    }

    /// Returns the number of elements in the binary tree.
    /// # Examples
    /// ```
    /// # use ds_rs::binary_tree::BinaryTree;
    /// let mut tree = BinaryTree::new();
    /// assert_eq!(tree.count(), 0);
    /// tree.insert(5);
    /// assert_eq!(tree.count(), 1);
    ///
    /// // duplicates do not enter the tree
    /// // and therefore do not affect the count
    /// tree.insert(5);
    /// assert_eq!(tree.count(), 1);
    ///
    /// // but new values will
    /// tree.insert(6);
    /// assert_eq!(tree.count(), 2);
    /// ```
    pub fn count(&self) -> usize {
        self.count
    }
}

impl<T: PartialOrd> From<Vec<T>> for BinaryTree<T> {
    /// Creates a `BinaryTree<T>` from `Vec<T>`.
    ///
    /// # Panic
    /// The function will panic if a comparison of elements is impossible with the [`PartialOrd`] trait.
    fn from(vec: Vec<T>) -> Self {
        let mut tree = BinaryTree::new();
        for v in vec {
            tree.insert(v);
        }

        tree
    }
}
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Item<T> {
    value: T,
    left: Option<Box<Item<T>>>,
    right: Option<Box<Item<T>>>,
}

impl<T: PartialOrd> Item<T> {
    /// Constructs a new empty `Item<T>`.
    ///
    /// An item has no left or right child.
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

    /// Creates a new `Item` from the provided value, and set it as the left child of `self`.
    pub fn set_left(&mut self, value: T) {
        self.left = Some(Box::new(Item::new(value)))
    }

    /// Creates a new `Item` from the provided value, and set it as the right child of `self`.
    pub fn set_right(&mut self, value: T) {
        self.right = Some(Box::new(Item::new(value)))
    }
}

#[cfg(test)]
mod item {
    use super::Item;

    #[test]
    fn gets_the_value() {
        let item = Item {
            value: 5,
            left: None,
            right: None,
        };
        assert_eq!(item.value(), &5);
    }

    #[test]
    fn gets_left_child_that_is_none() {
        let item = Item {
            value: 0,
            left: None,
            right: None,
        };
        let expected = None;

        assert_eq!(item.left(), expected);
    }

    #[test]
    fn gets_left_child_that_is_some() {
        let item = Item {
            value: 0,
            left: Some(Box::new(Item {
                value: 5,
                left: None,
                right: None,
            })),
            right: None,
        };
        let expected = Some(&Item {
            value: 5,
            left: None,
            right: None,
        });

        assert_eq!(item.left(), expected);
    }

    #[test]
    fn gets_right_child_that_is_none() {
        let item = Item {
            value: 0,
            left: None,
            right: None,
        };
        let expected = None;

        assert_eq!(item.right(), expected);
    }

    #[test]
    fn gets_right_child_that_is_some() {
        let item = Item {
            value: 0,
            left: None,
            right: Some(Box::new(Item {
                value: 5,
                left: None,
                right: None,
            })),
        };
        let expected = Some(&Item {
            value: 5,
            left: None,
            right: None,
        });

        assert_eq!(item.right(), expected);
    }

    #[test]
    fn gets_mut_left_child_that_is_none() {
        let mut item = Item {
            value: 0,
            left: None,
            right: None,
        };
        let expected = None;

        assert_eq!(item.left_mut(), expected);
    }

    #[test]
    fn gets_mut_left_child_that_is_some() {
        let mut item = Item {
            value: 0,
            left: Some(Box::new(Item {
                value: -1,
                left: None,
                right: None,
            })),
            right: None,
        };
        let mut expected = Item {
            value: -1,
            left: None,
            right: None,
        };

        assert_eq!(item.left_mut(), Some(&mut expected));
    }

    #[test]
    fn gets_mut_right_child_that_is_none() {
        let mut item = Item {
            value: 0,
            left: None,
            right: None,
        };
        let expected = None;

        assert_eq!(item.right_mut(), expected);
    }

    #[test]
    fn gets_mut_right_child_that_is_some() {
        let mut item = Item {
            value: 0,
            left: None,
            right: Some(Box::new(Item {
                value: 1,
                left: None,
                right: None,
            })),
        };
        let mut expected = Item {
            value: 1,
            left: None,
            right: None,
        };

        assert_eq!(item.right_mut(), Some(&mut expected));
    }

    #[test]
    fn sets_left() {
        let mut item = Item {
            value: 0,
            left: None,
            right: None,
        };
        let expected = Item {
            value: 0,
            left: Some(Box::new(Item {
                value: -1,
                left: None,
                right: None,
            })),
            right: None,
        };

        item.set_left(-1);
        assert_eq!(item, expected);
    }

    #[test]
    fn sets_right() {
        let mut item = Item {
            value: 0,
            left: None,
            right: None,
        };
        let expected = Item {
            value: 0,
            left: None,
            right: Some(Box::new(Item {
                value: 1,
                left: None,
                right: None,
            })),
        };

        item.set_right(1);
        assert_eq!(item, expected);
    }
}

#[cfg(test)]
mod binary_tree_getters {
    use super::{BinaryTree, Item};

    #[test]
    fn count() {
        let tree: BinaryTree<()> = BinaryTree {
            root: None,
            count: 5,
            height: 0,
        };
        let expected = 5;
        assert_eq!(tree.count(), expected);
    }

    #[test]
    fn height() {
        let tree: BinaryTree<()> = BinaryTree {
            root: None,
            count: 0,
            height: 3,
        };
        let expected = 3;
        assert_eq!(tree.height(), expected);
    }

    #[test]
    fn is_empty() {
        let tree = BinaryTree {
            root: Some(Box::new(Item {
                value: 5,
                left: None,
                right: None,
            })),
            count: 1,
            height: 1,
        };
        let empty_tree: BinaryTree<()> = BinaryTree {
            root: None,
            count: 0,
            height: 0,
        };

        assert!(!tree.is_empty());
        assert!(empty_tree.is_empty());
    }

    #[test]
    fn clear() {
        let mut tree = BinaryTree {
            root: Some(Box::new(Item {
                value: 5,
                left: None,
                right: None,
            })),
            count: 1,
            height: 1,
        };
        let expected: BinaryTree<i32> = BinaryTree {
            root: None,
            count: 0,
            height: 0,
        };

        assert_ne!(tree, expected);

        tree.clear();

        assert_eq!(tree, expected);
    }

    #[test]
    fn root() {
        let tree = BinaryTree {
            root: Some(Box::new(Item {
                value: 5,
                left: None,
                right: None,
            })),
            count: 1,
            height: 1,
        };
        let empty_tree: BinaryTree<()> = BinaryTree {
            root: None,
            count: 0,
            height: 0,
        };

        assert_eq!(tree.root(), Some(&5));
        assert_eq!(empty_tree.root(), None);
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

    #[test]
    fn discards_duplicate_inserts_of_root() {
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
        tree.insert(5);
        assert_eq!(tree, expected);
    }

    #[test]
    fn inserts_three_elements_second_and_third_are_right_children() {
        let mut tree = BinaryTree::new();
        let expected = BinaryTree {
            root: Some(Box::new(Item {
                value: 1,
                left: None,
                right: Some(Box::new(Item {
                    value: 2,
                    left: None,
                    right: Some(Box::new(Item {
                        value: 3,
                        left: None,
                        right: None,
                    })),
                })),
            })),
            count: 3,
            height: 3,
        };
        tree.insert(1);
        tree.insert(2);
        tree.insert(3);
        assert_eq!(tree, expected);
    }

    #[test]
    fn discards_duplicates_of_right_children() {
        let mut tree = BinaryTree::new();
        let expected = BinaryTree {
            root: Some(Box::new(Item {
                value: 1,
                left: None,
                right: Some(Box::new(Item {
                    value: 2,
                    left: None,
                    right: Some(Box::new(Item {
                        value: 3,
                        left: None,
                        right: None,
                    })),
                })),
            })),
            count: 3,
            height: 3,
        };
        tree.insert(1);
        tree.insert(2);
        tree.insert(3);
        tree.insert(1);
        tree.insert(2);
        tree.insert(3);
        assert_eq!(tree, expected);
    }

    #[test]
    fn inserts_three_elements_second_and_third_are_left_children() {
        let mut tree = BinaryTree::new();
        let expected = BinaryTree {
            root: Some(Box::new(Item {
                value: 3,
                left: Some(Box::new(Item {
                    value: 2,
                    left: Some(Box::new(Item {
                        value: 1,
                        left: None,
                        right: None,
                    })),
                    right: None,
                })),
                right: None,
            })),
            count: 3,
            height: 3,
        };
        tree.insert(3);
        tree.insert(2);
        tree.insert(1);
        assert_eq!(tree, expected);
    }

    #[test]
    fn discards_duplicates_of_left_children() {
        let mut tree = BinaryTree::new();
        let expected = BinaryTree {
            root: Some(Box::new(Item {
                value: 3,
                left: Some(Box::new(Item {
                    value: 2,
                    left: Some(Box::new(Item {
                        value: 1,
                        left: None,
                        right: None,
                    })),
                    right: None,
                })),
                right: None,
            })),
            count: 3,
            height: 3,
        };
        tree.insert(3);
        tree.insert(2);
        tree.insert(1);
        tree.insert(1);
        tree.insert(2);
        tree.insert(3);
        assert_eq!(tree, expected);
    }

    #[test]
    fn inserts_four_elements_zig_zag_starting_left() {
        let mut tree = BinaryTree::new();
        let expected = BinaryTree {
            root: Some(Box::new(Item {
                value: 10,
                left: Some(Box::new(Item {
                    value: 0,
                    left: None,
                    right: Some(Box::new(Item {
                        value: 5,
                        left: Some(Box::new(Item {
                            value: 3,
                            left: None,
                            right: None,
                        })),
                        right: None,
                    })),
                })),
                right: None,
            })),
            count: 4,
            height: 4,
        };
        tree.insert(10);
        tree.insert(0);
        tree.insert(5);
        tree.insert(3);
        assert_eq!(tree, expected);
    }

    #[test]
    fn inserts_four_elements_zig_zag_starting_right() {
        let mut tree = BinaryTree::new();
        let expected = BinaryTree {
            root: Some(Box::new(Item {
                value: 0,
                left: None,
                right: Some(Box::new(Item {
                    value: 10,
                    left: Some(Box::new(Item {
                        value: 3,
                        left: None,
                        right: Some(Box::new(Item {
                            value: 5,
                            left: None,
                            right: None,
                        })),
                    })),
                    right: None,
                })),
            })),
            count: 4,
            height: 4,
        };
        tree.insert(0);
        tree.insert(10);
        tree.insert(3);
        tree.insert(5);
        assert_eq!(tree, expected);
    }

    #[test]
    fn creates_tree_with_root_and_two_direct_children() {
        let mut tree1 = BinaryTree::new();
        let mut tree2 = BinaryTree::new();
        let expected = BinaryTree {
            root: Some(Box::new(Item {
                value: 2,
                left: Some(Box::new(Item {
                    value: 1,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Item {
                    value: 3,
                    left: None,
                    right: None,
                })),
            })),
            count: 3,
            height: 2,
        };

        // Creating left child first.
        tree1.insert(2);
        tree1.insert(1);
        tree1.insert(3);

        // Creating right child first.
        tree2.insert(2);
        tree2.insert(3);
        tree2.insert(1);

        assert_eq!(tree1, expected);
        assert_eq!(tree2, expected);
    }

    #[test]
    fn creates_three_layer_tree_one_layer_at_a_time() {
        let mut tree = BinaryTree::new();
        let expected = BinaryTree {
            root: Some(Box::new(Item {
                value: 50,
                left: Some(Box::new(Item {
                    value: 25,
                    left: Some(Box::new(Item {
                        value: 13,
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::new(Item {
                        value: 37,
                        left: None,
                        right: None,
                    })),
                })),
                right: Some(Box::new(Item {
                    value: 75,
                    left: Some(Box::new(Item {
                        value: 63,
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::new(Item {
                        value: 87,
                        left: None,
                        right: None,
                    })),
                })),
            })),
            count: 7,
            height: 3,
        };

        tree.insert(50);
        tree.insert(25);
        tree.insert(75);
        tree.insert(13);
        tree.insert(37);
        tree.insert(63);
        tree.insert(87);

        assert_eq!(tree, expected);
    }

    #[test]
    #[should_panic]
    fn incomparable_elements_in_empty_tree_panics() {
        let mut tree = BinaryTree::new();
        tree.insert(f64::NAN);
    }

    #[test]
    #[should_panic]
    fn incomparable_elements_in_non_empty_tree_panics() {
        let mut tree = BinaryTree::new();
        tree.insert(2.0);
        tree.insert(f64::NAN);
    }
}
