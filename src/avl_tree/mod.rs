mod avl_operations;

/// Internal implementation of `AVLTree` nodes.
pub mod node;

/// For visualizing (Graphviz, DOT format).
pub mod visualization;

use node::AVLNode;

/// A self-balancing AVL tree implementation.
///
/// Maintains stricter balance than a standard `BinarySearchTree` by enforcing:
/// - The heights of any node's subtrees differ by at most 1
/// - Standard BST invariants where for each node:
///   - All values in the left subtree are less than the node's value
///   - All values in the right subtree are greater than the node's value
///   - Duplicate values are not allowed
#[derive(Debug)]
pub struct AVLTree<T: PartialOrd + Clone> {
    /// Root node of the tree (private to maintain invariants)
    root: Option<Box<AVLNode<T>>>,

    /// Cached minimum value (None if tree is empty)
    min_value: Option<T>,

    /// Cached maximum value (None if tree is empty)
    max_value: Option<T>,
}

impl<T: PartialOrd + Clone> AVLTree<T> {
    /// Check AVL Tree balance.
    pub fn is_balanced(&self) -> bool {
        fn check_balance<T: PartialOrd>(node: &Option<Box<AVLNode<T>>>) -> bool {
            match node {
                Some(node) => {
                    let balance = node.balance_factor();
                    balance.abs() <= 1 && check_balance(&node.left) && check_balance(&node.right)
                }
                None => true,
            }
        }
        check_balance(&self.root)
    }

    /// Check BST invariant for AVL Tree.
    pub fn is_valid_bst(&self) -> bool {
        fn check<T: PartialOrd>(
            node: &Option<Box<AVLNode<T>>>,
            min: Option<&T>,
            max: Option<&T>,
        ) -> bool {
            match node {
                Some(node) => {
                    if let Some(min_val) = min {
                        if &node.value <= min_val {
                            return false;
                        }
                    }
                    if let Some(max_val) = max {
                        if &node.value >= max_val {
                            return false;
                        }
                    }
                    check(&node.left, min, Some(&node.value))
                        && check(&node.right, Some(&node.value), max)
                }
                None => true,
            }
        }
        check(&self.root, None, None)
    }
}
