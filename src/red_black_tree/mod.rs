mod rb_operations;

/// Internal implementation of `RedBlackTree` nodes.
pub mod node;

/// For visualizing (Graphviz, DOT format).
pub mod visualization;

use node::RBNode;

/// A self-balancing Red-Black Tree implementation.
///
/// Maintains balance properties:
/// - Every node is either red or black
/// - The root is always black
/// - All leaves (NIL) are black
/// - Red nodes cannot have red children
/// - All paths from root to leaves contain the same number of black nodes
///
/// Standard BST invariants where for each node:
/// - All values in the left subtree are less than the node's value
/// - All values in the right subtree are greater than the node's value
/// - Duplicate values are not allowed
#[derive(Debug)]
pub struct RedBlackTree<T: PartialOrd + Clone> {
    /// Root node of the tree (private to maintain invariants)
    root: Option<Box<RBNode<T>>>,

    /// Cached minimum value (None if tree is empty)
    min_value: Option<T>,

    /// Cached maximum value (None if tree is empty)
    max_value: Option<T>,
}

impl<T: PartialOrd + Clone> RedBlackTree<T> {
    /// Check if the tree maintains Red-Black properties.
    pub fn is_valid_red_black_tree(&self) -> bool {
        // Property 1: Root must be black
        if let Some(root) = &self.root {
            if root.is_red() {
                return false;
            }
        }

        // Check other properties recursively
        self.check_red_property(&self.root) && self.check_black_height(&self.root).is_some()
    }

    /// Checks that no red node has a red child.
    fn check_red_property(&self, node: &Option<Box<RBNode<T>>>) -> bool {
        match node {
            Some(node) => {
                if node.is_red() {
                    // Red node cannot have red children
                    if RBNode::is_red_node(&node.left) || RBNode::is_red_node(&node.right) {
                        return false;
                    }
                }
                self.check_red_property(&node.left) && self.check_red_property(&node.right)
            }
            None => true,
        }
    }

    /// Checks that all paths have the same black height.
    /// Returns Some(height) if valid, None if invalid.
    fn check_black_height(&self, node: &Option<Box<RBNode<T>>>) -> Option<usize> {
        match node {
            None => Some(1), // NIL nodes are black
            Some(node) => {
                let left_height = self.check_black_height(&node.left)?;
                let right_height = self.check_black_height(&node.right)?;

                if left_height != right_height {
                    return None;
                }

                if node.is_black() {
                    Some(left_height + 1)
                } else {
                    Some(left_height)
                }
            }
        }
    }

    /// Check BST invariant for Red-Black Tree.
    pub fn is_valid_bst(&self) -> bool {
        fn check<T: PartialOrd>(
            node: &Option<Box<RBNode<T>>>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_and_isnt_empty_tree() {
        let rbt_1 = RedBlackTree::<i32>::new();
        assert!(rbt_1.is_empty());

        let mut rbt_2 = RedBlackTree::<i32>::new();
        rbt_2.insert(42);
        assert!(!rbt_2.is_empty());
    }

    #[test]
    fn contains_in_empty_tree() {
        let rbt = RedBlackTree::<i32>::new();
        assert!(!rbt.contains(&0));
    }

    #[test]
    fn contains_in_single_node_tree() {
        let mut rbt = RedBlackTree::new();
        rbt.insert(1);
        assert!(rbt.contains(&1));
    }

    #[test]
    fn contains_basic() {
        let mut rbt = RedBlackTree::new();
        let values = vec![5, 3, 7, 2, 4, 6, 8];

        for value in &values {
            rbt.insert(*value);
        }

        for value in &values {
            assert!(rbt.contains(value));
        }

        assert!(!rbt.contains(&0));
        assert!(!rbt.contains(&9));
    }

    #[test]
    fn insert_maintains_red_black_properties() {
        let mut rbt = RedBlackTree::new();
        let values = vec![7, 3, 18, 10, 22, 8, 11, 26, 2, 6, 13];

        for value in values {
            rbt.insert(value);
            assert!(rbt.is_valid_red_black_tree(), "Tree invalid after inserting {}", value);
            assert!(rbt.is_valid_bst(), "BST property violated after inserting {}", value);
        }
    }

    #[test]
    fn remove_from_empty_tree() {
        let mut rbt = RedBlackTree::<i32>::new();
        rbt.remove(&42);
        assert!(!rbt.contains(&42));
        assert_eq!(rbt.min(), None);
        assert_eq!(rbt.max(), None);
    }

    #[test]
    fn remove_from_single_node_tree() {
        let mut rbt = RedBlackTree::new();
        rbt.insert(1);
        assert!(rbt.contains(&1));

        rbt.remove(&1);
        assert!(!rbt.contains(&1));
        assert_eq!(rbt.min(), None);
        assert_eq!(rbt.max(), None);
    }

    #[test]
    fn remove_maintains_red_black_properties() {
        let mut rbt = RedBlackTree::new();
        let values = vec![7, 3, 18, 10, 22, 8, 11, 26, 2, 6, 13];

        for value in &values {
            rbt.insert(*value);
        }

        for value in &values {
            rbt.remove(value);
            assert!(!rbt.contains(value));
            assert!(rbt.is_valid_red_black_tree(), "Tree invalid after removing {}", value);
            assert!(rbt.is_valid_bst(), "BST property violated after removing {}", value);
        }
    }

    #[test]
    fn min_max_in_empty_tree() {
        let rbt = RedBlackTree::<i32>::new();
        assert_eq!(rbt.min(), None);
        assert_eq!(rbt.max(), None);
    }

    #[test]
    fn min_max_basic() {
        let mut rbt = RedBlackTree::new();
        let values = vec![5, 3, 7, 2, 4, 6, 8];

        for value in &values {
            rbt.insert(*value);
        }

        assert_eq!(rbt.min(), Some(&2));
        assert_eq!(rbt.max(), Some(&8));
    }

    #[test]
    fn min_max_after_removal() {
        let mut rbt = RedBlackTree::new();
        let values = vec![5, 3, 7, 2, 4, 6, 8];

        for value in &values {
            rbt.insert(*value);
        }

        rbt.remove(&2);
        assert_eq!(rbt.min(), Some(&3));

        rbt.remove(&8);
        assert_eq!(rbt.max(), Some(&7));
    }

    #[test]
    fn height_in_empty_tree() {
        let rbt = RedBlackTree::<i32>::new();
        assert_eq!(rbt.height(), 0);
    }

    #[test]
    fn height_basic() {
        let mut rbt = RedBlackTree::new();
        let values = vec![5, 3, 7, 2, 4, 6, 8];

        for value in &values {
            rbt.insert(*value);
        }

        // Red-Black trees are balanced, so height should be reasonable
        assert!(rbt.height() <= 2 * ((values.len() + 1) as f64).log2().ceil() as usize);
    }

    #[test]
    fn pre_order_in_empty_tree() {
        let rbt = RedBlackTree::<i32>::new();
        assert_eq!(rbt.pre_order(), Vec::<&i32>::new());
    }

    #[test]
    fn in_order_in_empty_tree() {
        let rbt = RedBlackTree::<i32>::new();
        assert_eq!(rbt.in_order(), Vec::<&i32>::new());
    }

    #[test]
    fn in_order_returns_sorted() {
        let mut rbt = RedBlackTree::new();
        let values = vec![5, 3, 7, 2, 4, 6, 8];

        for value in &values {
            rbt.insert(*value);
        }

        let in_order = rbt.in_order();
        assert_eq!(in_order, vec![&2, &3, &4, &5, &6, &7, &8]);
    }

    #[test]
    fn post_order_in_empty_tree() {
        let rbt = RedBlackTree::<i32>::new();
        assert_eq!(rbt.post_order(), Vec::<&i32>::new());
    }

    #[test]
    fn level_order_in_empty_tree() {
        let rbt = RedBlackTree::<i32>::new();
        assert_eq!(rbt.level_order(), Vec::<&i32>::new());
    }

    #[test]
    fn number_of_elements_in_empty_tree() {
        let rbt = RedBlackTree::<i32>::new();
        assert_eq!(rbt.number_of_elements(), 0);
    }

    #[test]
    fn number_of_elements_basic() {
        let mut rbt = RedBlackTree::new();
        let values = vec![5, 3, 7, 2, 4, 6, 8];

        for value in &values {
            rbt.insert(*value);
        }

        assert_eq!(rbt.number_of_elements(), values.len());
    }

    #[test]
    fn ceil_in_empty_tree() {
        let rbt = RedBlackTree::<i32>::new();
        assert_eq!(rbt.ceil(&0), None);
    }

    #[test]
    fn ceil_basic() {
        let mut rbt = RedBlackTree::<i32>::new();
        rbt.insert(1);
        rbt.insert(2);
        rbt.insert(5);

        assert_eq!(rbt.ceil(&6), None);
        assert_eq!(rbt.ceil(&5), Some(&5));
        assert_eq!(rbt.ceil(&4), Some(&5));
        assert_eq!(rbt.ceil(&3), Some(&5));
        assert_eq!(rbt.ceil(&2), Some(&2));
        assert_eq!(rbt.ceil(&1), Some(&1));
        assert_eq!(rbt.ceil(&0), Some(&1));
    }

    #[test]
    fn floor_in_empty_tree() {
        let rbt = RedBlackTree::<i32>::new();
        assert_eq!(rbt.floor(&0), None);
    }

    #[test]
    fn floor_basic() {
        let mut rbt = RedBlackTree::<i32>::new();
        rbt.insert(1);
        rbt.insert(2);
        rbt.insert(5);

        assert_eq!(rbt.floor(&6), Some(&5));
        assert_eq!(rbt.floor(&5), Some(&5));
        assert_eq!(rbt.floor(&4), Some(&2));
        assert_eq!(rbt.floor(&3), Some(&2));
        assert_eq!(rbt.floor(&2), Some(&2));
        assert_eq!(rbt.floor(&1), Some(&1));
        assert_eq!(rbt.floor(&0), None);
    }

    #[test]
    fn sequential_insert_maintains_balance() {
        let mut rbt = RedBlackTree::new();

        // Insert in ascending order (would create degenerate BST)
        for i in 1..=20 {
            rbt.insert(i);
        }

        assert!(rbt.is_valid_red_black_tree());
        assert!(rbt.is_valid_bst());
        // Height should be logarithmic
        assert!(rbt.height() <= 10);
    }

    #[test]
    fn reverse_sequential_insert_maintains_balance() {
        let mut rbt = RedBlackTree::new();

        // Insert in descending order
        for i in (1..=20).rev() {
            rbt.insert(i);
        }

        assert!(rbt.is_valid_red_black_tree());
        assert!(rbt.is_valid_bst());
        assert!(rbt.height() <= 10);
    }
}
