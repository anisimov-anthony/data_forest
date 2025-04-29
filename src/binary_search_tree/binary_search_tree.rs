use super::node::BinaryNode;
use std::cmp::Ordering;

pub struct BinarySearchTree<T: Ord> {
    pub root: Option<Box<BinaryNode<T>>>,
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree { root: None }
    }

    pub fn insert(&mut self, value: T) {
        let mut cursor = &mut self.root;

        while let Some(current_node) = cursor {
            if current_node.value < value {
                cursor = &mut current_node.right;
            } else if current_node.value > value {
                cursor = &mut current_node.left;
            } else {
                return;
            }
        }

        *cursor = Some(Box::new(BinaryNode::new(value)));
    }

    fn pass_and_detach_local_minimum(root: &mut Option<Box<BinaryNode<T>>>) -> Option<T> {
        if root.is_none() {
            return None;
        }

        if root.as_mut().unwrap().left.is_none() {
            let node = root.take().unwrap();
            *root = node.right;
            return Some(node.value);
        }

        let mut parent = root.as_mut().unwrap();
        while parent.left.as_ref().unwrap().left.is_some() {
            parent = parent.left.as_mut().unwrap();
        }

        let leftmost = parent.left.take().unwrap();
        parent.left = leftmost.right;
        Some(leftmost.value)
    }

    pub fn remove(&mut self, value: &T) {
        let mut cursor = &mut self.root;
        while let Some(current) = cursor {
            match value.cmp(&current.value) {
                Ordering::Less => cursor = &mut cursor.as_mut().unwrap().left,
                Ordering::Greater => cursor = &mut cursor.as_mut().unwrap().right,
                Ordering::Equal => match (current.left.as_mut(), current.right.as_mut()) {
                    (None, None) => *cursor = None,
                    (Some(_), None) => *cursor = current.left.take(),
                    (None, Some(_)) => *cursor = current.right.take(),
                    (Some(_), Some(_)) => {
                        cursor.as_mut().unwrap().value =
                            Self::pass_and_detach_local_minimum(&mut current.right).unwrap();
                    }
                },
            }
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        let mut cursor = &self.root;

        while let Some(current_node) = cursor {
            if current_node.value < *value {
                cursor = &current_node.right;
            } else if current_node.value > *value {
                cursor = &current_node.left;
            } else {
                return true;
            }
        }

        false
    }

    pub fn min(&self) -> Option<&T> {
        let mut cursor = &self.root;

        while let Some(current_node) = cursor {
            if current_node.left.is_some() {
                cursor = &current_node.left;
            } else {
                return Some(&current_node.value);
            }
        }
        None
    }

    pub fn max(&self) -> Option<&T> {
        let mut cursor = &self.root;

        while let Some(current_node) = cursor {
            if current_node.right.is_some() {
                cursor = &current_node.right;
            } else {
                return Some(&current_node.value);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_in_empty_tree() {
        let bst = BinarySearchTree::<i32>::new();

        assert!(!bst.contains(&0));
    }

    #[test]
    fn contains_in_single_node_tree() {
        let mut bst = BinarySearchTree::new();
        bst.insert(1);

        assert!(bst.contains(&1));
    }

    #[test]
    fn contains_in_degenerate_trees() {
        let mut bst_degenerate_right = BinarySearchTree::new();
        let mut bst_degenerate_left = BinarySearchTree::new();

        for i in 0..=10 {
            bst_degenerate_right.insert(i);
        }
        for i in (0..=10).rev() {
            bst_degenerate_left.insert(i);
        }

        for i in 0..=10 {
            assert!(bst_degenerate_right.contains(&i));
            assert!(bst_degenerate_left.contains(&i));
        }
    }

    #[test]
    fn contains_basic() {
        let mut bst_diff_heights_null = BinarySearchTree::new();
        let mut bst_diff_heights_one = BinarySearchTree::new();
        let mut bst_diff_heights_two = BinarySearchTree::new();

        let values_1 = vec![5, 3, 7, 2, 4, 6, 8];
        let values_2 = vec![4, 2, 6, 1, 3, 5];
        let values_3 = vec![8, 4, 12, 2, 6, 10, 14, 1, 7];
        for value in &values_1 {
            bst_diff_heights_null.insert(value);
        }
        for value in &values_2 {
            bst_diff_heights_one.insert(value);
        }
        for value in &values_3 {
            bst_diff_heights_two.insert(value);
        }

        for value in &values_1 {
            assert!(bst_diff_heights_null.contains(&value));
        }
        for value in &values_2 {
            assert!(bst_diff_heights_one.contains(&value));
        }
        for value in &values_3 {
            assert!(bst_diff_heights_two.contains(&value));
        }
    }

    #[test]
    fn remove_from_empty_tree() {
        let mut bst = BinarySearchTree::<i32>::new();

        bst.remove(&42);

        assert!(!bst.contains(&42));
        assert_eq!(bst.min(), None);
        assert_eq!(bst.max(), None);
    }

    #[test]
    fn remove_from_single_node_tree() {
        let mut bst = BinarySearchTree::new();
        bst.insert(1);
        bst.remove(&1);

        assert!(!bst.contains(&1));
    }

    #[test]
    fn remove_from_degenerate_trees() {
        let mut bst_degenerate_right = BinarySearchTree::new();
        let mut bst_degenerate_left = BinarySearchTree::new();

        for i in 0..=10 {
            bst_degenerate_right.insert(i);
        }
        for i in (0..=10).rev() {
            bst_degenerate_left.insert(i);
        }

        for i in 0..=10 {
            bst_degenerate_right.remove(&i);
            bst_degenerate_left.remove(&i);
            assert!(!bst_degenerate_right.contains(&i));
            assert!(!bst_degenerate_left.contains(&i));
        }
    }

    #[test]
    fn remove_basic() {
        let mut bst_diff_heights_null = BinarySearchTree::new();
        let mut bst_diff_heights_one = BinarySearchTree::new();
        let mut bst_diff_heights_two = BinarySearchTree::new();

        let values_1 = vec![5, 3, 7, 2, 4, 6, 8];
        let values_2 = vec![4, 2, 6, 1, 3, 5];
        let values_3 = vec![8, 4, 12, 2, 6, 10, 14, 1, 7];
        for value in &values_1 {
            bst_diff_heights_null.insert(value);
        }
        for value in &values_2 {
            bst_diff_heights_one.insert(value);
        }
        for value in &values_3 {
            bst_diff_heights_two.insert(value);
        }

        for value in &values_1 {
            bst_diff_heights_null.remove(&value);
            assert!(!bst_diff_heights_null.contains(&value));
        }
        for value in &values_2 {
            bst_diff_heights_one.remove(&value);
            assert!(!bst_diff_heights_one.contains(&value));
        }
        for value in &values_3 {
            bst_diff_heights_two.remove(&value);
            assert!(!bst_diff_heights_two.contains(&value));
        }
    }

    #[test]
    fn min_in_empty_tree() {
        let bst = BinarySearchTree::<i32>::new();

        assert_eq!(bst.min(), None);
    }

    #[test]
    fn min_in_degenerate_trees() {
        let mut bst_degenerate_right = BinarySearchTree::new();
        let mut bst_degenerate_left = BinarySearchTree::new();

        for i in 0..=10 {
            bst_degenerate_right.insert(i);
        }
        for i in (0..=10).rev() {
            bst_degenerate_left.insert(i);
        }

        assert_eq!(bst_degenerate_right.min(), Some(&0));
        assert_eq!(bst_degenerate_left.min(), Some(&0));
    }

    #[test]
    fn min_basic() {
        let mut bst_diff_heights_null = BinarySearchTree::new();
        let mut bst_diff_heights_one = BinarySearchTree::new();
        let mut bst_diff_heights_two = BinarySearchTree::new();

        let values_1 = vec![5, 3, 7, 2, 4, 6, 8];
        let values_2 = vec![4, 2, 6, 1, 3, 5];
        let values_3 = vec![8, 4, 12, 2, 6, 10, 14, 1, 7];

        for value in &values_1 {
            bst_diff_heights_null.insert(value);
        }
        for value in &values_2 {
            bst_diff_heights_one.insert(value);
        }
        for value in &values_3 {
            bst_diff_heights_two.insert(value);
        }

        assert_eq!(bst_diff_heights_null.min(), values_1.iter().min().as_ref());
        assert_eq!(bst_diff_heights_one.min(), values_2.iter().min().as_ref());
        assert_eq!(bst_diff_heights_two.min(), values_3.iter().min().as_ref());
    }

    #[test]
    fn max_in_empty_tree() {
        let bst = BinarySearchTree::<i32>::new();

        assert_eq!(bst.max(), None);
    }

    #[test]
    fn max_in_degenerate_trees() {
        let mut bst_degenerate_right = BinarySearchTree::new();
        let mut bst_degenerate_left = BinarySearchTree::new();

        for i in 0..=10 {
            bst_degenerate_right.insert(i);
        }
        for i in (0..=10).rev() {
            bst_degenerate_left.insert(i);
        }

        assert_eq!(bst_degenerate_right.max(), Some(&10));
        assert_eq!(bst_degenerate_left.max(), Some(&10));
    }

    #[test]
    fn max_basic() {
        let mut bst_diff_heights_null = BinarySearchTree::new();
        let mut bst_diff_heights_one = BinarySearchTree::new();
        let mut bst_diff_heights_two = BinarySearchTree::new();

        let values_1 = vec![5, 3, 7, 2, 4, 6, 8];
        let values_2 = vec![4, 2, 6, 1, 3, 5];
        let values_3 = vec![8, 4, 12, 2, 6, 10, 14, 1, 7];
        for value in &values_1 {
            bst_diff_heights_null.insert(value);
        }
        for value in &values_2 {
            bst_diff_heights_one.insert(value);
        }
        for value in &values_3 {
            bst_diff_heights_two.insert(value);
        }

        assert_eq!(bst_diff_heights_null.max(), values_1.iter().max().as_ref());
        assert_eq!(bst_diff_heights_one.max(), values_2.iter().max().as_ref());
        assert_eq!(bst_diff_heights_two.max(), values_3.iter().max().as_ref());
    }

    #[test]
    fn max_min_are_similar_for_single_element_tree() {
        let mut bst = BinarySearchTree::new();
        bst.insert(1);

        assert!(bst.min() == bst.max() && bst.min() == Some(&1));
    }
}
