mod bst_operations;

/// Internal implementation of `BinarySearchTree` nodes.
pub mod node;

/// For visualizing (Graphviz, DOT format).
pub mod visualization;

use node::BinaryNode;

/// A binary search tree implementation.
///
/// This tree maintains the binary search tree invariant where for each node:
/// - All values in the left subtree are less than the node's value
/// - All values in the right subtree are greater than the node's value
/// - Duplicate values are not allowed
#[derive(Debug)]
pub struct BinarySearchTree<T: PartialOrd + Clone> {
    /// Root node of the tree (private to maintain invariants)
    root: Option<Box<BinaryNode<T>>>,

    /// Cached minimum value (None if tree is empty)
    min_value: Option<T>,

    /// Cached maximum value (None if tree is empty)
    max_value: Option<T>,
}
