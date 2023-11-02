use std::collections::VecDeque;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinaryTree<T>
where
    T: PartialOrd,
{
    root: Option<Box<Node<T>>>,
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
    /// # Panics
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

        if let Some(mut root) = self.root.as_deref_mut() {
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
                        self.height = self.height.max(level);
                        self.count += 1;
                        return;
                    }
                    (None, Some(_)) => match value.partial_cmp(root.value()).unwrap() {
                        Ord::Equal => return,
                        Ord::Less => {
                            root.set_left(value);
                            self.height = self.height.max(level);
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
                            self.height = self.height.max(level);
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
        } else {
            // This ensures that root is not an imcomparable value.
            _ = value.partial_cmp(&value).unwrap();

            self.root = Some(Box::new(Node::new(value)));
            self.count = 1;
            self.height = 1;
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
    #[inline]
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
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn count(&self) -> usize {
        self.count
    }

    /// Returns a non-consuming iterator over the `BinaryTree`.
    ///
    /// The iterator yields all items in the tree using the **preorder tree traversal techinque**.
    ///
    /// # Examples
    /// ```
    /// # use ds_rs::binary_tree::BinaryTree;
    /// let tree = BinaryTree::from(vec![5, 4, 6]);
    /// let mut tree_iter = tree.iter();
    ///
    /// assert_eq!(tree_iter.next(), Some(&5));
    /// assert_eq!(tree_iter.next(), Some(&4));
    /// assert_eq!(tree_iter.next(), Some(&6));
    ///
    /// // the iterator is now empty
    /// assert_eq!(tree_iter.next(), None);
    /// ```
    #[inline]
    #[must_use = "iterators are evaluated lazily"]
    pub fn iter(&self) -> Iter<'_, T> {
        self.as_ref().into_iter()
    }

    /// Returns the smallest element in the `BinaryTree`.
    ///
    /// # Time Complexity
    ///
    /// The implementation uses the properties of a binary tree to efficiently
    /// find and return the smallest element, meaning for a balanced tree this
    /// will be near `log(n)`, which is likely to be faster than an iterator.
    ///
    /// However, an unbalanced tree will be closer to linear time.
    ///
    /// # Examples
    /// ```
    /// # use ds_rs::binary_tree::BinaryTree;
    /// let tree = BinaryTree::from(vec![8, 4, 6, 16, -5, 25]);
    /// assert_eq!(tree.min(), Some(&-5));
    /// ```
    pub fn min(&self) -> Option<&T> {
        if let Some(mut node) = self.root.as_deref() {
            loop {
                match (node.left(), node.right()) {
                    (None, None | Some(_)) => return Some(node.value()),
                    (Some(left), None | Some(_)) => node = left,
                }
            }
        } else {
            None
        }
    }

    /// Returns the largest element in the `BinaryTree`.
    ///
    /// # Time Complexity
    ///
    /// The implementation uses the properties of a binary tree to efficiently
    /// find and return the largest element, meaning for a balanced tree this
    /// will be near `log(n)`, which is likely to be faster than an iterator.
    ///
    /// However, an unbalanced tree will be closer to linear time.
    ///
    /// # Examples
    /// ```
    /// # use ds_rs::binary_tree::BinaryTree;
    /// let tree = BinaryTree::from(vec![8, 4, 6, 16, -5, 25]);
    /// assert_eq!(tree.max(), Some(&25));
    /// ```
    pub fn max(&self) -> Option<&T> {
        if let Some(mut node) = self.root.as_deref() {
            loop {
                match (node.left(), node.right()) {
                    (None | Some(_), None) => return Some(node.value()),
                    (None | Some(_), Some(right)) => node = right,
                }
            }
        } else {
            None
        }
    }

    /// Returns `true` if the `BinaryTree` contains an element with the given value.
    ///
    /// # Panics
    /// The function will panic if a comparison of elements is impossible with the [`PartialOrd`] trait.
    ///
    /// # Time Complexity
    ///
    /// This implementation uses the properties of a binary tree to efficiently
    /// find the element with the given value, provided that it exists in the tree.
    ///
    /// As a result, a balanced tree will be near `log(n)`, which is likely to be faster than an iterator.
    ///
    /// However, an unbalanced tree will be closer to linear time.
    ///
    /// # Examples
    /// ```
    /// # use ds_rs::binary_tree::BinaryTree;
    /// let tree = BinaryTree::from(vec![8, 4, 6, 16, -5, 25]);
    /// assert!(tree.contains(&8));
    /// assert!(!tree.contains(&5));
    /// ```
    pub fn contains(&self, target: &T) -> bool {
        use std::cmp::Ordering as O;

        if let Some(mut node) = self.root.as_deref() {
            loop {
                match target.partial_cmp(node.value()).unwrap() {
                    O::Equal => return true,
                    O::Less => {
                        if let Some(left) = node.left() {
                            node = left;
                        } else {
                            break;
                        }
                    }
                    O::Greater => {
                        if let Some(right) = node.right() {
                            node = right;
                        } else {
                            break;
                        }
                    }
                }
            }
        }

        false
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

impl<T: PartialOrd> AsRef<BinaryTree<T>> for BinaryTree<T> {
    /// Returns an immutable reference to the `BinaryTree`.
    #[inline]
    fn as_ref(&self) -> &BinaryTree<T> {
        self
    }
}

impl<T: PartialOrd> AsMut<BinaryTree<T>> for BinaryTree<T> {
    /// Returns a mutable reference to the `BinaryTree`.
    #[inline]
    fn as_mut(&mut self) -> &mut BinaryTree<T> {
        self
    }
}

impl<T: PartialOrd> FromIterator<T> for BinaryTree<T> {
    /// Constructs a `BinaryTree<T>` from an iterator for `T`.
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut tree = BinaryTree::new();

        for v in iter {
            tree.insert(v);
        }

        tree
    }
}

impl<T: PartialOrd> Extend<T> for BinaryTree<T> {
    /// Extends the `BinaryTree` with the contents of the provided iterator.
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for v in iter {
            self.insert(v);
        }
    }
}

impl<T: PartialOrd> IntoIterator for BinaryTree<T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    /// Returns a consuming iterator over the `BinaryTree`.
    ///
    /// The iterator yields all items in the tree using the **preorder tree traversal techinque**.
    ///
    /// # Examples
    /// ```
    /// # use ds_rs::binary_tree::BinaryTree;
    /// let tree = BinaryTree::from(vec![5, 4, 6]);
    /// let mut tree_iter = tree.into_iter();
    ///
    /// assert_eq!(tree_iter.next(), Some(5));
    /// assert_eq!(tree_iter.next(), Some(4));
    /// assert_eq!(tree_iter.next(), Some(6));
    ///
    /// // the iterator is now empty
    /// assert_eq!(tree_iter.next(), None);
    /// ```
    #[must_use = "iterators are evaluated lazily"]
    fn into_iter(self) -> Self::IntoIter {
        let mut values = Vec::with_capacity(self.count);
        let mut queue = VecDeque::new();

        if let Some(root) = self.root {
            queue.push_front(root);

            while let Some(node) = queue.pop_front() {
                values.push(node.value);

                if let Some(right) = node.right {
                    queue.push_front(right);
                }

                if let Some(left) = node.left {
                    queue.push_front(left);
                }
            }
        }

        IntoIter {
            vec: values.into_iter(),
        }
    }
}

/// An iterator that moves out of the `BinaryTree`.
///
/// This `struct` is created by the `into_iter` method on [`BinaryTree`] (provided by the [`IntoIterator`] trait).
pub struct IntoIter<T> {
    vec: std::vec::IntoIter<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.vec.next()
    }
}

impl<'a, T: PartialOrd> IntoIterator for &'a BinaryTree<T> {
    type Item = &'a T;

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let mut values = Vec::with_capacity(self.count);
        let mut queue = VecDeque::new();

        if let Some(root) = &self.root {
            queue.push_front(root);

            while let Some(node) = queue.pop_front() {
                values.push(&node.value);

                if let Some(right) = &node.right {
                    queue.push_front(right);
                }

                if let Some(left) = &node.left {
                    queue.push_front(left);
                }
            }
        }

        Iter {
            vec: values,
            index: 0,
        }
    }
}

/// An iterator that borrows from the `BinaryTree`.
///
/// This `struct` is created by the `iter` method on [`BinaryTree`].
pub struct Iter<'a, T> {
    vec: Vec<&'a T>,
    index: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let val = match self.vec.get(self.index) {
            Some(_) => Some(self.vec[self.index]),
            None => None,
        };
        self.index += 1;

        val
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Node<T> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T: PartialOrd> Node<T> {
    /// Constructs a new empty `Node<T>`.
    ///
    /// An node has no left or right child.
    pub fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    /// Returns a reference to the value of the node.
    #[inline]
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Returns an `Option` containing a reference to the left child of the node.
    #[inline]
    pub fn left(&self) -> Option<&Self> {
        self.left.as_deref()
    }

    /// Returns an `Option` containing a reference to the right child of the node.
    #[inline]
    pub fn right(&self) -> Option<&Self> {
        self.right.as_deref()
    }

    /// Returns an `Option` containing a mutable reference to the left child of the node.
    #[inline]
    pub fn left_mut(&mut self) -> Option<&mut Self> {
        self.left.as_deref_mut()
    }

    /// Returns an `Option` containing a mutable reference to the right child of the node.
    #[inline]
    pub fn right_mut(&mut self) -> Option<&mut Self> {
        self.right.as_deref_mut()
    }

    /// Creates a new `Node` from the provided value, and set it as the left child of `self`.
    #[inline]
    pub fn set_left(&mut self, value: T) {
        self.left = Some(Box::new(Node::new(value)));
    }

    /// Creates a new `Node` from the provided value, and set it as the right child of `self`.
    #[inline]
    pub fn set_right(&mut self, value: T) {
        self.right = Some(Box::new(Node::new(value)));
    }
}

#[cfg(test)]
mod node {
    use super::Node;

    #[test]
    fn gets_the_value() {
        let node = Node {
            value: 5,
            left: None,
            right: None,
        };
        assert_eq!(node.value(), &5);
    }

    #[test]
    fn gets_left_child_that_is_none() {
        let node = Node {
            value: 0,
            left: None,
            right: None,
        };
        let expected = None;

        assert_eq!(node.left(), expected);
    }

    #[test]
    fn gets_left_child_that_is_some() {
        let node = Node {
            value: 0,
            left: Some(Box::new(Node {
                value: 5,
                left: None,
                right: None,
            })),
            right: None,
        };
        let expected = Some(&Node {
            value: 5,
            left: None,
            right: None,
        });

        assert_eq!(node.left(), expected);
    }

    #[test]
    fn gets_right_child_that_is_none() {
        let node = Node {
            value: 0,
            left: None,
            right: None,
        };
        let expected = None;

        assert_eq!(node.right(), expected);
    }

    #[test]
    fn gets_right_child_that_is_some() {
        let node = Node {
            value: 0,
            left: None,
            right: Some(Box::new(Node {
                value: 5,
                left: None,
                right: None,
            })),
        };
        let expected = Some(&Node {
            value: 5,
            left: None,
            right: None,
        });

        assert_eq!(node.right(), expected);
    }

    #[test]
    fn gets_mut_left_child_that_is_none() {
        let mut node = Node {
            value: 0,
            left: None,
            right: None,
        };
        let expected = None;

        assert_eq!(node.left_mut(), expected);
    }

    #[test]
    fn gets_mut_left_child_that_is_some() {
        let mut node = Node {
            value: 0,
            left: Some(Box::new(Node {
                value: -1,
                left: None,
                right: None,
            })),
            right: None,
        };
        let mut expected = Node {
            value: -1,
            left: None,
            right: None,
        };

        assert_eq!(node.left_mut(), Some(&mut expected));
    }

    #[test]
    fn gets_mut_right_child_that_is_none() {
        let mut node = Node {
            value: 0,
            left: None,
            right: None,
        };
        let expected = None;

        assert_eq!(node.right_mut(), expected);
    }

    #[test]
    fn gets_mut_right_child_that_is_some() {
        let mut node = Node {
            value: 0,
            left: None,
            right: Some(Box::new(Node {
                value: 1,
                left: None,
                right: None,
            })),
        };
        let mut expected = Node {
            value: 1,
            left: None,
            right: None,
        };

        assert_eq!(node.right_mut(), Some(&mut expected));
    }

    #[test]
    fn sets_left() {
        let mut node = Node {
            value: 0,
            left: None,
            right: None,
        };
        let expected = Node {
            value: 0,
            left: Some(Box::new(Node {
                value: -1,
                left: None,
                right: None,
            })),
            right: None,
        };

        node.set_left(-1);
        assert_eq!(node, expected);
    }

    #[test]
    fn sets_right() {
        let mut node = Node {
            value: 0,
            left: None,
            right: None,
        };
        let expected = Node {
            value: 0,
            left: None,
            right: Some(Box::new(Node {
                value: 1,
                left: None,
                right: None,
            })),
        };

        node.set_right(1);
        assert_eq!(node, expected);
    }
}

#[cfg(test)]
mod getters {
    use super::{BinaryTree, Node};

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
            root: Some(Box::new(Node {
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
            root: Some(Box::new(Node {
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
            root: Some(Box::new(Node {
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
mod insert {
    use super::{BinaryTree, Node};

    #[test]
    fn insert_one_element_that_becomes_root() {
        let mut tree = BinaryTree::new();
        let expected = BinaryTree {
            root: Some(Box::new(Node {
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
            root: Some(Box::new(Node {
                value: 5,
                left: Some(Box::new(Node {
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
            root: Some(Box::new(Node {
                value: 5,
                left: None,
                right: Some(Box::new(Node {
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
            root: Some(Box::new(Node {
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
            root: Some(Box::new(Node {
                value: 1,
                left: None,
                right: Some(Box::new(Node {
                    value: 2,
                    left: None,
                    right: Some(Box::new(Node {
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
            root: Some(Box::new(Node {
                value: 1,
                left: None,
                right: Some(Box::new(Node {
                    value: 2,
                    left: None,
                    right: Some(Box::new(Node {
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
            root: Some(Box::new(Node {
                value: 3,
                left: Some(Box::new(Node {
                    value: 2,
                    left: Some(Box::new(Node {
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
            root: Some(Box::new(Node {
                value: 3,
                left: Some(Box::new(Node {
                    value: 2,
                    left: Some(Box::new(Node {
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
            root: Some(Box::new(Node {
                value: 10,
                left: Some(Box::new(Node {
                    value: 0,
                    left: None,
                    right: Some(Box::new(Node {
                        value: 5,
                        left: Some(Box::new(Node {
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
            root: Some(Box::new(Node {
                value: 0,
                left: None,
                right: Some(Box::new(Node {
                    value: 10,
                    left: Some(Box::new(Node {
                        value: 3,
                        left: None,
                        right: Some(Box::new(Node {
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
            root: Some(Box::new(Node {
                value: 2,
                left: Some(Box::new(Node {
                    value: 1,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Node {
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
            root: Some(Box::new(Node {
                value: 50,
                left: Some(Box::new(Node {
                    value: 25,
                    left: Some(Box::new(Node {
                        value: 13,
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::new(Node {
                        value: 37,
                        left: None,
                        right: None,
                    })),
                })),
                right: Some(Box::new(Node {
                    value: 75,
                    left: Some(Box::new(Node {
                        value: 63,
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::new(Node {
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

    #[test]
    fn inserts_unevenly_and_ensures_correct_height() {
        let mut tree = BinaryTree::new();
        let expected = 3;

        tree.insert(2);
        tree.insert(1);
        tree.insert(0);
        tree.insert(3);

        assert_eq!(tree.height(), expected);
    }

    #[test]
    fn inserts_unevenly_and_ensures_correct_count() {
        let mut tree = BinaryTree::new();
        let expected = 4;

        tree.insert(2);
        tree.insert(1);
        tree.insert(0);
        tree.insert(3);

        assert_eq!(tree.count(), expected);
    }
}

#[cfg(test)]
mod min {
    use super::{BinaryTree, Node};

    #[test]
    fn empty_tree_returns_none() {
        let tree: BinaryTree<i32> = BinaryTree {
            root: None,
            count: 0,
            height: 0,
        };

        assert_eq!(tree.min(), None);
    }

    #[test]
    fn tree_with_only_root_returns_root() {
        let tree = BinaryTree {
            root: Some(Box::new(Node {
                value: 5,
                left: None,
                right: None,
            })),
            count: 0,
            height: 0,
        };

        assert_eq!(tree.min(), Some(&5));
    }

    #[test]
    fn tree_with_root_and_right_child_returns_root() {
        let tree = BinaryTree {
            root: Some(Box::new(Node {
                value: 5,
                left: None,
                right: Some(Box::new(Node {
                    value: 6,
                    left: None,
                    right: None,
                })),
            })),
            count: 0,
            height: 0,
        };

        assert_eq!(tree.min(), Some(&5));
    }

    #[test]
    fn tree_with_root_and_left_child_returns_left() {
        let tree = BinaryTree {
            root: Some(Box::new(Node {
                value: 5,
                left: Some(Box::new(Node {
                    value: 4,
                    left: None,
                    right: None,
                })),
                right: None,
            })),
            count: 0,
            height: 0,
        };

        assert_eq!(tree.min(), Some(&4));
    }

    #[test]
    fn tree_with_root_and_both_childen_returns_left() {
        let tree = BinaryTree {
            root: Some(Box::new(Node {
                value: 5,
                left: Some(Box::new(Node {
                    value: 4,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Node {
                    value: 6,
                    left: None,
                    right: None,
                })),
            })),
            count: 0,
            height: 0,
        };

        assert_eq!(tree.min(), Some(&4));
    }

    #[test]
    fn tree_with_root_and_multiple_right_returns_root() {
        let tree = BinaryTree {
            root: Some(Box::new(Node {
                value: 5,
                left: None,
                right: Some(Box::new(Node {
                    value: 6,
                    left: None,
                    right: Some(Box::new(Node {
                        value: 7,
                        left: None,
                        right: Some(Box::new(Node {
                            value: 8,
                            left: None,
                            right: None,
                        })),
                    })),
                })),
            })),
            count: 0,
            height: 0,
        };

        assert_eq!(tree.min(), Some(&5));
    }

    #[test]
    fn tree_with_root_and_multiple_left_returns_left_most_child() {
        let tree = BinaryTree {
            root: Some(Box::new(Node {
                value: 5,
                left: Some(Box::new(Node {
                    value: 4,
                    left: Some(Box::new(Node {
                        value: 3,
                        left: Some(Box::new(Node {
                            value: 2,
                            left: None,
                            right: None,
                        })),
                        right: None,
                    })),
                    right: None,
                })),
                right: None,
            })),
            count: 0,
            height: 0,
        };

        assert_eq!(tree.min(), Some(&2));
    }

    #[test]
    fn balanced_tree_returns_left_most_child() {
        let tree = BinaryTree {
            root: Some(Box::new(Node {
                value: 50,
                left: Some(Box::new(Node {
                    value: 25,
                    left: Some(Box::new(Node {
                        value: 13,
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::new(Node {
                        value: 37,
                        left: None,
                        right: None,
                    })),
                })),
                right: Some(Box::new(Node {
                    value: 75,
                    left: Some(Box::new(Node {
                        value: 63,
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::new(Node {
                        value: 87,
                        left: None,
                        right: None,
                    })),
                })),
            })),
            count: 7,
            height: 3,
        };

        assert_eq!(tree.min(), Some(&13));
    }
}

#[cfg(test)]
mod max {
    use super::{BinaryTree, Node};

    #[test]
    fn empty_tree_returns_none() {
        let tree: BinaryTree<i32> = BinaryTree {
            root: None,
            count: 0,
            height: 0,
        };

        assert_eq!(tree.max(), None);
    }

    #[test]
    fn tree_with_only_root_returns_root() {
        let tree = BinaryTree {
            root: Some(Box::new(Node {
                value: 5,
                left: None,
                right: None,
            })),
            count: 0,
            height: 0,
        };

        assert_eq!(tree.max(), Some(&5));
    }

    #[test]
    fn tree_with_root_and_right_child_returns_right() {
        let tree = BinaryTree {
            root: Some(Box::new(Node {
                value: 5,
                left: None,
                right: Some(Box::new(Node {
                    value: 6,
                    left: None,
                    right: None,
                })),
            })),
            count: 0,
            height: 0,
        };

        assert_eq!(tree.max(), Some(&6));
    }

    #[test]
    fn tree_with_root_and_left_child_returns_root() {
        let tree = BinaryTree {
            root: Some(Box::new(Node {
                value: 5,
                left: Some(Box::new(Node {
                    value: 4,
                    left: None,
                    right: None,
                })),
                right: None,
            })),
            count: 0,
            height: 0,
        };

        assert_eq!(tree.max(), Some(&5));
    }

    #[test]
    fn tree_with_root_and_both_childen_returns_right() {
        let tree = BinaryTree {
            root: Some(Box::new(Node {
                value: 5,
                left: Some(Box::new(Node {
                    value: 4,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Node {
                    value: 6,
                    left: None,
                    right: None,
                })),
            })),
            count: 0,
            height: 0,
        };

        assert_eq!(tree.max(), Some(&6));
    }

    #[test]
    fn tree_with_root_and_multiple_right_returns_right_most_child() {
        let tree = BinaryTree {
            root: Some(Box::new(Node {
                value: 5,
                left: None,
                right: Some(Box::new(Node {
                    value: 6,
                    left: None,
                    right: Some(Box::new(Node {
                        value: 7,
                        left: None,
                        right: Some(Box::new(Node {
                            value: 8,
                            left: None,
                            right: None,
                        })),
                    })),
                })),
            })),
            count: 0,
            height: 0,
        };

        assert_eq!(tree.max(), Some(&8));
    }

    #[test]
    fn tree_with_root_and_multiple_left_returns_root() {
        let tree = BinaryTree {
            root: Some(Box::new(Node {
                value: 5,
                left: Some(Box::new(Node {
                    value: 4,
                    left: Some(Box::new(Node {
                        value: 3,
                        left: Some(Box::new(Node {
                            value: 2,
                            left: None,
                            right: None,
                        })),
                        right: None,
                    })),
                    right: None,
                })),
                right: None,
            })),
            count: 0,
            height: 0,
        };

        assert_eq!(tree.max(), Some(&5));
    }

    #[test]
    fn balanced_tree_returns_right_most_child() {
        let tree = BinaryTree {
            root: Some(Box::new(Node {
                value: 50,
                left: Some(Box::new(Node {
                    value: 25,
                    left: Some(Box::new(Node {
                        value: 13,
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::new(Node {
                        value: 37,
                        left: None,
                        right: None,
                    })),
                })),
                right: Some(Box::new(Node {
                    value: 75,
                    left: Some(Box::new(Node {
                        value: 63,
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::new(Node {
                        value: 87,
                        left: None,
                        right: None,
                    })),
                })),
            })),
            count: 7,
            height: 3,
        };

        assert_eq!(tree.max(), Some(&87));
    }
}

#[cfg(test)]
mod contains {
    use super::{BinaryTree, Node};

    #[test]
    fn empty_tree_returns_false() {
        let tree: BinaryTree<i32> = BinaryTree {
            root: None,
            count: 0,
            height: 0,
        };

        assert!(!tree.contains(&0));
    }

    #[test]
    fn root_is_target_returns_true() {
        let tree = BinaryTree {
            root: Some(Box::new(Node {
                value: 0,
                left: None,
                right: None,
            })),
            count: 0,
            height: 0,
        };

        assert!(tree.contains(&0));
    }

    #[test]
    fn root_is_not_target_returns_false() {
        let tree = BinaryTree {
            root: Some(Box::new(Node {
                value: 0,
                left: None,
                right: None,
            })),
            count: 0,
            height: 0,
        };

        assert!(!tree.contains(&1));
    }

    #[test]
    fn left_most_is_target_returns_true() {
        let tree = BinaryTree {
            root: Some(Box::new(Node {
                value: 5,
                left: Some(Box::new(Node {
                    value: 4,
                    left: Some(Box::new(Node {
                        value: 3,
                        left: Some(Box::new(Node {
                            value: 2,
                            left: None,
                            right: None,
                        })),
                        right: None,
                    })),
                    right: None,
                })),
                right: None,
            })),
            count: 0,
            height: 0,
        };

        assert!(tree.contains(&2));
    }

    #[test]
    fn right_most_is_target_returns_true() {
        let tree = BinaryTree {
            root: Some(Box::new(Node {
                value: 5,
                left: None,
                right: Some(Box::new(Node {
                    value: 6,
                    left: None,
                    right: Some(Box::new(Node {
                        value: 7,
                        left: None,
                        right: Some(Box::new(Node {
                            value: 8,
                            left: None,
                            right: None,
                        })),
                    })),
                })),
            })),
            count: 0,
            height: 0,
        };

        assert!(tree.contains(&8));
    }
}

#[cfg(test)]
mod iterator_trait_impls {
    use super::{BinaryTree, Node};

    #[test]
    fn creates_tree_from_vec() {
        let values = vec![5, 4, 6];
        let expected = BinaryTree {
            root: Some(Box::new(Node {
                value: 5,
                left: Some(Box::new(Node {
                    value: 4,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Node {
                    value: 6,
                    left: None,
                    right: None,
                })),
            })),
            count: 3,
            height: 2,
        };

        let tree = BinaryTree::from(values);
        assert_eq!(tree, expected);
    }

    #[test]
    #[should_panic]
    fn panics_when_creating_from_vec_of_incomparable_elements() {
        let values = vec![5.0, 4.0, 6.0, f64::NAN];
        _ = BinaryTree::from(values);
    }

    #[test]
    fn into_iter_from_small_tree() {
        let tree = BinaryTree {
            root: Some(Box::new(Node {
                value: 5,
                left: Some(Box::new(Node {
                    value: 4,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Node {
                    value: 6,
                    left: None,
                    right: None,
                })),
            })),
            count: 3,
            height: 2,
        };

        let mut tree_iter = tree.into_iter();

        assert_eq!(tree_iter.next(), Some(5));
        assert_eq!(tree_iter.next(), Some(4));
        assert_eq!(tree_iter.next(), Some(6));
        assert_eq!(tree_iter.next(), None);
    }

    #[test]
    fn into_iter_from_large_tree() {
        let tree = BinaryTree {
            root: Some(Box::new(Node {
                value: 50,
                left: Some(Box::new(Node {
                    value: 25,
                    left: Some(Box::new(Node {
                        value: 13,
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::new(Node {
                        value: 37,
                        left: None,
                        right: None,
                    })),
                })),
                right: Some(Box::new(Node {
                    value: 75,
                    left: Some(Box::new(Node {
                        value: 63,
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::new(Node {
                        value: 87,
                        left: None,
                        right: None,
                    })),
                })),
            })),
            count: 7,
            height: 3,
        };

        let mut iter = tree.into_iter();

        assert_eq!(iter.next(), Some(50));
        assert_eq!(iter.next(), Some(25));
        assert_eq!(iter.next(), Some(13));
        assert_eq!(iter.next(), Some(37));
        assert_eq!(iter.next(), Some(75));
        assert_eq!(iter.next(), Some(63));
        assert_eq!(iter.next(), Some(87));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn creates_tree_from_iterator() {
        let tree = BinaryTree::from_iter(vec![5, 4, 6]);
        let expected = BinaryTree {
            root: Some(Box::new(Node {
                value: 5,
                left: Some(Box::new(Node {
                    value: 4,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Node {
                    value: 6,
                    left: None,
                    right: None,
                })),
            })),
            count: 3,
            height: 2,
        };

        assert_eq!(tree, expected)
    }

    #[test]
    fn iter_from_tree() {
        let tree = BinaryTree {
            root: Some(Box::new(Node {
                value: 5,
                left: Some(Box::new(Node {
                    value: 4,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Node {
                    value: 6,
                    left: None,
                    right: None,
                })),
            })),
            count: 3,
            height: 2,
        };

        let mut iter = tree.iter();

        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_from_large_tree() {
        let tree = BinaryTree {
            root: Some(Box::new(Node {
                value: 50,
                left: Some(Box::new(Node {
                    value: 25,
                    left: Some(Box::new(Node {
                        value: 13,
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::new(Node {
                        value: 37,
                        left: None,
                        right: None,
                    })),
                })),
                right: Some(Box::new(Node {
                    value: 75,
                    left: Some(Box::new(Node {
                        value: 63,
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::new(Node {
                        value: 87,
                        left: None,
                        right: None,
                    })),
                })),
            })),
            count: 7,
            height: 3,
        };

        let mut iter = tree.iter();

        assert_eq!(iter.next(), Some(&50));
        assert_eq!(iter.next(), Some(&25));
        assert_eq!(iter.next(), Some(&13));
        assert_eq!(iter.next(), Some(&37));
        assert_eq!(iter.next(), Some(&75));
        assert_eq!(iter.next(), Some(&63));
        assert_eq!(iter.next(), Some(&87));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn extends_collection_with_iterator() {
        let mut tree = BinaryTree {
            root: Some(Box::new(Node {
                value: 50,
                left: Some(Box::new(Node {
                    value: 25,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Node {
                    value: 75,
                    left: None,
                    right: None,
                })),
            })),
            count: 3,
            height: 2,
        };
        let expected = BinaryTree {
            root: Some(Box::new(Node {
                value: 50,
                left: Some(Box::new(Node {
                    value: 25,
                    left: Some(Box::new(Node {
                        value: 13,
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::new(Node {
                        value: 37,
                        left: None,
                        right: None,
                    })),
                })),
                right: Some(Box::new(Node {
                    value: 75,
                    left: Some(Box::new(Node {
                        value: 63,
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::new(Node {
                        value: 87,
                        left: None,
                        right: None,
                    })),
                })),
            })),
            count: 7,
            height: 3,
        };

        tree.extend([13, 37, 63, 87]);

        assert_eq!(tree, expected);
    }
}

#[cfg(all(test, feature = "serde"))]
mod json {
    use super::{BinaryTree, Node};
    use rstest::{fixture, rstest};

    #[fixture]
    fn json_tree() -> &'static str {
        r#"{"root":{"value":5,"left":{"value":4,"left":null,"right":null},"right":{"value":6,"left":null,"right":null}},"count":3,"height":2}"#
    }

    #[rstest]
    fn deserializes_tree_from_json(json_tree: &'static str) {
        let tree: BinaryTree<i32> =
            serde_json::from_str(json_tree).expect("should parse json into tree");
        let expected = BinaryTree {
            root: Some(Box::new(Node {
                value: 5,
                left: Some(Box::new(Node {
                    value: 4,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Node {
                    value: 6,
                    left: None,
                    right: None,
                })),
            })),
            count: 3,
            height: 2,
        };

        assert_eq!(tree, expected);
    }

    #[rstest]
    fn serialize_tree_into_json(json_tree: &'static str) {
        let tree = BinaryTree {
            root: Some(Box::new(Node {
                value: 5,
                left: Some(Box::new(Node {
                    value: 4,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Node {
                    value: 6,
                    left: None,
                    right: None,
                })),
            })),
            count: 3,
            height: 2,
        };
        let actual = serde_json::to_string(&tree).expect("should parse tree into json");

        assert_eq!(actual, json_tree);
    }
}
