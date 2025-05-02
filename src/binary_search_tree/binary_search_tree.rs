use super::node::BinaryNode;
use std::cmp::Ordering;
use std::collections::VecDeque;

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

    pub fn pre_order(&self) -> Vec<&T> {
        let mut result = Vec::new();
        let mut stack = Vec::new();
        let mut current = &self.root;

        while !stack.is_empty() || current.is_some() {
            while let Some(node) = current {
                result.push(&node.value);
                stack.push(node);
                current = &node.left;
            }

            current = &stack.pop().unwrap().right;
        }

        result
    }

    pub fn in_order(&self) -> Vec<&T> {
        let mut result = Vec::new();
        let mut stack = Vec::new();
        let mut current = &self.root;

        while !stack.is_empty() || current.is_some() {
            while let Some(node) = current {
                stack.push(node);
                current = &node.left;
            }

            if let Some(node) = stack.pop() {
                result.push(&node.value);
                current = &node.right;
            }
        }

        result
    }

    pub fn post_order(&self) -> Vec<&T> {
        let mut result = Vec::new();
        let mut stack = Vec::new();
        let mut current = &self.root;
        let mut last_visited: Option<&Box<BinaryNode<T>>> = None;

        while !stack.is_empty() || current.is_some() {
            while let Some(node) = current {
                stack.push(node);
                current = &node.left;
            }

            if let Some(node) = stack.last() {
                let right_visited = match (&node.right, last_visited) {
                    (Some(right), Some(last)) => std::ptr::eq(right.as_ref(), last.as_ref()),
                    _ => false,
                };

                if node.right.is_none() || right_visited {
                    result.push(&node.value);
                    last_visited = stack.pop();
                } else {
                    current = &node.right;
                }
            }
        }

        result
    }

    pub fn level_order(&self) -> Vec<&T> {
        let mut result = Vec::new();
        let mut queue = VecDeque::new();

        if let Some(root) = &self.root {
            queue.push_back(root);
        }

        while let Some(node) = queue.pop_front() {
            result.push(&node.value);

            if let Some(left) = &node.left {
                queue.push_back(left);
            }
            if let Some(right) = &node.right {
                queue.push_back(right);
            }
        }

        result
    }

    pub fn number_of_elements(&self) -> usize {
        self.pre_order().len() as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bst_rs::{BinarySearchTree as BinarySearchTreeOther, IterativeBST as IterativeBSTOther};
    use proptest::prelude::*;

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

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 1000,
            ..ProptestConfig::default()
        })]
        #[test]
        fn prop_insert_contains(values in prop::collection::vec(any::<i32>(), 1..1000)) {
            let mut bst = BinarySearchTree::new();

            for &v in &values {
                bst.insert(v);
            }

            for &v in &values {
                assert!(bst.contains(&v));
            }
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

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 1000,
            ..ProptestConfig::default()
        })]
        #[test]
        fn prop_remove(values in prop::collection::vec(any::<i32>(), 1..1000)) {
            let mut bst = BinarySearchTree::new();

            for &v in &values {
                bst.insert(v);
            }

            for &v in &values {
                assert!(bst.contains(&v));
            }

            for &v in &values {
                bst.remove(&v);
            }

            for &v in &values {
                assert!(!bst.contains(&v));
            }
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

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 1000,
            ..ProptestConfig::default()
        })]
        #[test]
        fn prop_min(values in prop::collection::vec(any::<i32>(), 1..1000)) {
            let mut bst = BinarySearchTree::new();

            for &v in &values {
                bst.insert(v);
            }

            assert_eq!(bst.min(), values.iter().min());
        }
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

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 1000,
            ..ProptestConfig::default()
        })]
        #[test]
        fn prop_max(values in prop::collection::vec(any::<i32>(), 1..1000)) {
            let mut bst = BinarySearchTree::new();

            for &v in &values {
                bst.insert(v);
            }

            assert_eq!(bst.max(), values.iter().max());
        }
    }

    #[test]
    fn max_min_are_similar_for_single_element_tree() {
        let mut bst = BinarySearchTree::new();
        bst.insert(1);

        assert!(bst.min() == bst.max() && bst.min() == Some(&1));
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 1000,
            ..ProptestConfig::default()
        })]
        #[test]
        fn prop_max_min_are_similar_for_single_element_tree(value in any::<i32>()) {
            let mut bst = BinarySearchTree::new();
            bst.insert(value);

            assert!(bst.min() == bst.max() && bst.min() == Some(&value));
        }
    }

    #[test]
    fn pre_order_in_empty_tree() {
        let bst = BinarySearchTree::<i32>::new();

        assert_eq!(bst.pre_order(), Vec::<&i32>::new());
    }

    #[test]
    fn pre_order_in_degenerate_trees() {
        let mut bst_degenerate_right = BinarySearchTree::new();
        let mut bst_degenerate_left = BinarySearchTree::new();

        for i in 0..=10 {
            bst_degenerate_right.insert(i);
        }
        for i in (0..=10).rev() {
            bst_degenerate_left.insert(i);
        }

        assert_eq!(
            bst_degenerate_right.pre_order(),
            vec![&0, &1, &2, &3, &4, &5, &6, &7, &8, &9, &10]
        );
        assert_eq!(
            bst_degenerate_left.pre_order(),
            vec![&10, &9, &8, &7, &6, &5, &4, &3, &2, &1, &0]
        );
    }

    #[test]
    fn pre_order_basic() {
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

        assert_eq!(
            bst_diff_heights_null.pre_order(),
            vec![&&5, &&3, &&2, &&4, &&7, &&6, &&8]
        );

        assert_eq!(
            bst_diff_heights_one.pre_order(),
            vec![&&4, &&2, &&1, &&3, &&6, &&5]
        );

        assert_eq!(
            bst_diff_heights_two.pre_order(),
            vec![&&8, &&4, &&2, &&1, &&6, &&7, &&12, &&10, &&14]
        );
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 1000,
            ..ProptestConfig::default()
        })]
        #[test]
        fn prop_pre_order(values in prop::collection::vec(any::<i32>(), 1..1000)) {
            let mut bst = BinarySearchTree::new();
            let mut bst_comparing = IterativeBSTOther::new();

            for &v in &values {
                bst.insert(v);
                bst_comparing.insert(v);
            }

            assert_eq!(bst.pre_order(), bst_comparing.pre_order_vec());
        }
    }

    #[test]
    fn in_order_in_empty_tree() {
        let bst = BinarySearchTree::<i32>::new();

        assert_eq!(bst.in_order(), Vec::<&i32>::new());
    }

    #[test]
    fn in_order_in_degenerate_trees() {
        let mut bst_degenerate_right = BinarySearchTree::new();
        let mut bst_degenerate_left = BinarySearchTree::new();

        for i in 0..=10 {
            bst_degenerate_right.insert(i);
        }
        for i in (0..=10).rev() {
            bst_degenerate_left.insert(i);
        }

        assert_eq!(
            bst_degenerate_right.in_order(),
            vec![&0, &1, &2, &3, &4, &5, &6, &7, &8, &9, &10]
        );
        assert_eq!(
            bst_degenerate_left.in_order(),
            vec![&0, &1, &2, &3, &4, &5, &6, &7, &8, &9, &10]
        );
    }

    #[test]
    fn in_order_basic() {
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

        assert_eq!(
            bst_diff_heights_null.in_order(),
            vec![&&2, &&3, &&4, &&5, &&6, &&7, &&8]
        );

        assert_eq!(
            bst_diff_heights_one.in_order(),
            vec![&&1, &&2, &&3, &&4, &&5, &&6]
        );

        assert_eq!(
            bst_diff_heights_two.in_order(),
            vec![&&1, &&2, &&4, &&6, &&7, &&8, &&10, &&12, &&14]
        );
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 1000,
            ..ProptestConfig::default()
        })]
        #[test]
        fn prop_in_order(values in prop::collection::vec(any::<i32>(), 1..1000)) {
            let mut bst = BinarySearchTree::new();
            let mut bst_comparing = IterativeBSTOther::new();

            for &v in &values {
                bst.insert(v);
                bst_comparing.insert(v);
            }

            assert_eq!(bst.in_order(), bst_comparing.in_order_vec());
        }
    }

    #[test]
    fn post_order_in_empty_tree() {
        let bst = BinarySearchTree::<i32>::new();

        assert_eq!(bst.post_order(), Vec::<&i32>::new());
    }

    #[test]
    fn post_order_in_degenerate_trees() {
        let mut bst_degenerate_right = BinarySearchTree::new();
        let mut bst_degenerate_left = BinarySearchTree::new();

        for i in 0..=10 {
            bst_degenerate_right.insert(i);
        }
        for i in (0..=10).rev() {
            bst_degenerate_left.insert(i);
        }

        assert_eq!(
            bst_degenerate_right.post_order(),
            vec![&10, &9, &8, &7, &6, &5, &4, &3, &2, &1, &0]
        );
        assert_eq!(
            bst_degenerate_left.post_order(),
            vec![&0, &1, &2, &3, &4, &5, &6, &7, &8, &9, &10]
        );
    }

    #[test]
    fn post_order_basic() {
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

        assert_eq!(
            bst_diff_heights_null.post_order(),
            vec![&&2, &&4, &&3, &&6, &&8, &&7, &&5]
        );

        assert_eq!(
            bst_diff_heights_one.post_order(),
            vec![&&1, &&3, &&2, &&5, &&6, &&4]
        );

        assert_eq!(
            bst_diff_heights_two.post_order(),
            vec![&&1, &&2, &&7, &&6, &&4, &&10, &&14, &&12, &&8]
        );
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 1000,
            ..ProptestConfig::default()
        })]
        #[test]
        fn prop_post_order(values in prop::collection::vec(any::<i32>(), 1..1000)) {
            let mut bst = BinarySearchTree::new();
            let mut bst_comparing = IterativeBSTOther::new();

            for &v in &values {
                bst.insert(v);
                bst_comparing.insert(v);
            }

            assert_eq!(bst.post_order(), bst_comparing.post_order_vec());
        }
    }

    #[test]
    fn level_order_in_empty_tree() {
        let bst = BinarySearchTree::<i32>::new();

        assert_eq!(bst.level_order(), Vec::<&i32>::new());
    }

    #[test]
    fn level_order_in_degenerate_trees() {
        let mut bst_degenerate_right = BinarySearchTree::new();
        let mut bst_degenerate_left = BinarySearchTree::new();

        for i in 0..=10 {
            bst_degenerate_right.insert(i);
        }
        for i in (0..=10).rev() {
            bst_degenerate_left.insert(i);
        }

        assert_eq!(
            bst_degenerate_right.level_order(),
            vec![&0, &1, &2, &3, &4, &5, &6, &7, &8, &9, &10]
        );
        assert_eq!(
            bst_degenerate_left.level_order(),
            vec![&10, &9, &8, &7, &6, &5, &4, &3, &2, &1, &0]
        );
    }

    #[test]
    fn level_order_basic() {
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

        assert_eq!(
            bst_diff_heights_null.level_order(),
            vec![&&5, &&3, &&7, &&2, &&4, &&6, &&8]
        );

        assert_eq!(
            bst_diff_heights_one.level_order(),
            vec![&&4, &&2, &&6, &&1, &&3, &&5]
        );

        assert_eq!(
            bst_diff_heights_two.level_order(),
            vec![&&8, &&4, &&12, &&2, &&6, &&10, &&14, &&1, &&7]
        );
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 1000,
            ..ProptestConfig::default()
        })]
        #[test]
        fn prop_level_order(values in prop::collection::vec(any::<i32>(), 1..1000)) {
            let mut bst = BinarySearchTree::new();
            let mut bst_comparing = IterativeBSTOther::new();

            for &v in &values {
                bst.insert(v);
                bst_comparing.insert(v);
            }

            assert_eq!(bst.level_order(), bst_comparing.level_order_vec());
        }
    }

    #[test]
    fn number_of_elements_in_empty_tree() {
        let bst = BinarySearchTree::<i32>::new();

        assert_eq!(bst.number_of_elements(), 0);
    }

    #[test]
    fn number_of_elements_in_degenerate_trees() {
        let mut bst_degenerate_right = BinarySearchTree::new();
        let mut bst_degenerate_left = BinarySearchTree::new();

        for i in 0..=10 {
            bst_degenerate_right.insert(i);
        }
        for i in (0..=10).rev() {
            bst_degenerate_left.insert(i);
        }

        assert_eq!(bst_degenerate_right.number_of_elements(), 10 + 1);
        assert_eq!(bst_degenerate_left.number_of_elements(), 10 + 1);
    }

    #[test]
    fn number_of_elements_basic() {
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

        assert_eq!(bst_diff_heights_null.number_of_elements(), values_1.len());
        assert_eq!(bst_diff_heights_one.number_of_elements(), values_2.len());
        assert_eq!(bst_diff_heights_two.number_of_elements(), values_3.len());
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 1000,
            ..ProptestConfig::default()
        })]
        #[test]
        fn prop_number_of_elements(values in prop::collection::vec(any::<i32>(), 1..1000)) {
            let mut bst = BinarySearchTree::new();

            for &v in &values {
                bst.insert(v);
            }

            assert_eq!(bst.number_of_elements(), values.iter().collect::<std::collections::HashSet<_>>().len());
        }
    }
}
