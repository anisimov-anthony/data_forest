use super::node::BinaryNode;
use std::cmp::Ordering;
use std::collections::VecDeque;

pub struct BinarySearchTree<T: PartialOrd + Clone> {
    pub root: Option<Box<BinaryNode<T>>>,
    min_value: Option<T>,
    max_value: Option<T>,
}

impl<T: PartialOrd + Clone> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree {
            root: None,
            min_value: None,
            max_value: None,
        }
    }

    pub fn insert(&mut self, value: T) {
        match (&self.min_value, &self.max_value) {
            (None, None) => {
                self.min_value = Some(value.clone());
                self.max_value = Some(value.clone());
            }
            (Some(min), Some(max)) => {
                if &value < min {
                    self.min_value = Some(value.clone());
                }
                if &value > max {
                    self.max_value = Some(value.clone());
                }
            }
            _ => unreachable!(),
        }

        let mut cursor = &mut self.root;

        while let Some(current_node) = cursor {
            match value.partial_cmp(&current_node.value) {
                Some(Ordering::Less) => cursor = &mut current_node.left,
                Some(Ordering::Greater) => cursor = &mut current_node.right,
                Some(Ordering::Equal) => return,
                None => return,
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

    pub fn remove(&mut self, value: &T)
    where
        T: PartialOrd + Clone,
    {
        let mut cursor = &mut self.root;

        while let Some(current) = cursor {
            match value.partial_cmp(&current.value) {
                Some(Ordering::Less) => {
                    cursor = &mut cursor.as_mut().unwrap().left;
                }
                Some(Ordering::Greater) => {
                    cursor = &mut cursor.as_mut().unwrap().right;
                }
                Some(Ordering::Equal) => {
                    match (current.left.as_mut(), current.right.as_mut()) {
                        (None, None) => *cursor = None,
                        (Some(_), None) => *cursor = current.left.take(),
                        (None, Some(_)) => *cursor = current.right.take(),
                        (Some(_), Some(_)) => {
                            cursor.as_mut().unwrap().value =
                                Self::pass_and_detach_local_minimum(&mut current.right).unwrap();
                        }
                    }
                    break;
                }
                None => {
                    break;
                }
            }
        }

        self.min_value = self.refind_min();
        self.max_value = self.refind_max();
    }

    pub fn contains(&self, value: &T) -> bool {
        let mut cursor = &self.root;

        while let Some(current_node) = cursor {
            match value.partial_cmp(&current_node.value) {
                Some(Ordering::Less) => cursor = &current_node.left,
                Some(Ordering::Greater) => cursor = &current_node.right,
                Some(Ordering::Equal) => return true,
                None => return false,
            }
        }

        false
    }

    /// Returns a reference to the minimum element of the tree or `None` if tree is empty.
    ///
    /// Complexity: *O*(1) (due to storing the minimum element inside the tree structure).
    pub fn min(&self) -> Option<&T> {
        self.min_value.as_ref()
    }

    /// Returns a reference to the maximum element of the tree or `None` if tree is empty.
    ///
    /// Complexity: *O*(1) (due to storing the maximum element inside the tree structure).
    pub fn max(&self) -> Option<&T> {
        self.max_value.as_ref()
    }

    fn refind_min(&self) -> Option<T> {
        let mut cursor = &self.root;

        while let Some(current_node) = cursor {
            match current_node.left {
                Some(_) => cursor = &current_node.left,
                None => return Some(current_node.value.clone()),
            }
        }
        None
    }

    fn refind_max(&self) -> Option<T> {
        let mut cursor = &self.root;

        while let Some(current_node) = cursor {
            if current_node.right.is_some() {
                cursor = &current_node.right;
            } else {
                return Some(current_node.value.clone());
            }
        }
        None
    }

    pub fn height(&self) -> usize {
        if self.root.is_none() {
            return 0;
        }

        let mut height = 0;
        let mut queue = VecDeque::new();

        if let Some(root) = &self.root {
            queue.push_back(root);
        }

        while !queue.is_empty() {
            let level_size = queue.len();

            for _ in 0..level_size {
                let node = queue.pop_front().unwrap();

                if let Some(left) = &node.left {
                    queue.push_back(left);
                }
                if let Some(right) = &node.right {
                    queue.push_back(right);
                }
            }

            if !queue.is_empty() {
                height += 1;
            }
        }

        height
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

    pub fn ceil(&self, value: &T) -> Option<&T> {
        if self.root.is_none() {
            return None;
        }

        let mut result = None;
        let mut cursor = &self.root;

        while let Some(node) = cursor {
            if &node.value == value {
                return Some(&node.value);
            }

            if &node.value < value {
                cursor = &node.right;
            } else {
                result = Some(&node.value);
                cursor = &node.left;
            }
        }

        result
    }

    pub fn floor(&self, value: &T) -> Option<&T> {
        if self.root.is_none() {
            return None;
        }

        let mut result = None;
        let mut cursor = &self.root;

        while let Some(node) = cursor {
            if &node.value == value {
                return Some(&node.value);
            }

            if &node.value > value {
                cursor = &node.left;
            } else {
                result = Some(&node.value);
                cursor = &node.right;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bst_rs::{BinarySearchTree as BinarySearchTreeOther, IterativeBST as IterativeBSTOther};
    use proptest::prelude::*;
    use std::collections::HashSet;

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
            cases: 111,
            ..ProptestConfig::default()
        })]
        #[test]
        fn prop_insert_contains(values in prop::collection::vec(any::<i32>(), 1..111)) {
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
    fn remove_from_single_node_tree_check_min_max_updating() {
        let mut bst = BinarySearchTree::new();
        assert!(bst.max() == bst.min() && bst.max() == None);

        bst.insert(1);
        assert!(bst.max() == bst.min() && bst.max() == Some(&1));
        assert!(bst.contains(&1));

        bst.remove(&1);
        assert!(bst.max() == bst.min() && bst.max() == None);
        assert!(!bst.contains(&1));
    }

    #[test]
    fn remove_from_degenerate_trees_check_min_max_updating() {
        let mut bst_degenerate_right = BinarySearchTree::new();
        let mut bst_degenerate_left = BinarySearchTree::new();

        for i in 0..=10 {
            bst_degenerate_right.insert(i);
            assert_eq!(bst_degenerate_right.min(), Some(&0));
            assert_eq!(bst_degenerate_right.max(), Some(&i));
        }
        for i in (0..=10).rev() {
            bst_degenerate_left.insert(i);
            assert_eq!(bst_degenerate_left.min(), Some(&i));
            assert_eq!(bst_degenerate_left.max(), Some(&10));
        }

        for i in 0..=10 {
            bst_degenerate_right.remove(&i);
            assert!(!bst_degenerate_right.contains(&i));

            bst_degenerate_left.remove(&i);
            assert!(!bst_degenerate_left.contains(&i));

            if i < 10 {
                assert_eq!(bst_degenerate_right.min(), Some(&(i + 1)));
                assert_eq!(bst_degenerate_right.max(), Some(&10));

                assert_eq!(bst_degenerate_left.min(), Some(&(i + 1)));
                assert_eq!(bst_degenerate_left.max(), Some(&10));
            } else {
                assert_eq!(bst_degenerate_right.min(), None);
                assert_eq!(bst_degenerate_right.max(), None);

                assert_eq!(bst_degenerate_left.min(), None);
                assert_eq!(bst_degenerate_left.max(), None);
            }
        }
    }

    #[test]
    fn remove_basic_check_min_max_updating() {
        let mut bst1 = BinarySearchTree::new();
        let mut bst2 = BinarySearchTree::new();
        let mut bst3 = BinarySearchTree::new();

        let values1 = vec![5, 3, 7, 2, 4, 6, 8];
        let values2 = vec![4, 2, 6, 1, 3, 5];
        let values3 = vec![8, 4, 12, 2, 6, 10, 14, 1, 7];

        for &v in &values1 {
            bst1.insert(v);
        }
        for &v in &values2 {
            bst2.insert(v);
        }
        for &v in &values3 {
            bst3.insert(v);
        }

        let mut remaining1 = values1.clone();
        let mut remaining2 = values2.clone();
        let mut remaining3 = values3.clone();

        for &v in &values1 {
            bst1.remove(&v);
            remaining1.retain(|&x| x != v);

            assert!(!bst1.contains(&v));
            assert_eq!(bst1.min(), remaining1.iter().min());
            assert_eq!(bst1.max(), remaining1.iter().max());
        }

        for &v in &values2 {
            bst2.remove(&v);
            remaining2.retain(|&x| x != v);

            assert!(!bst2.contains(&v));
            assert_eq!(bst2.min(), remaining2.iter().min());
            assert_eq!(bst2.max(), remaining2.iter().max());
        }

        for &v in &values3 {
            bst3.remove(&v);
            remaining3.retain(|&x| x != v);

            assert!(!bst3.contains(&v));
            assert_eq!(bst3.min(), remaining3.iter().min());
            assert_eq!(bst3.max(), remaining3.iter().max());
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 111,
            ..ProptestConfig::default()
        })]
        #[test]
        fn prop_remove_check_min_max_updating(values in prop::collection::vec(any::<i32>(), 1..100)) {
            let mut bst = BinarySearchTree::new();
            let unique_values: Vec<i32> = values.iter().cloned().collect::<HashSet<_>>().into_iter().collect();
            let mut remaining = unique_values.clone();

            for &v in &values {
                bst.insert(v);
            }

            for &v in &values {
                bst.remove(&v);
                remaining.retain(|&x| x != v);

                assert!(!bst.contains(&v));
                assert_eq!(bst.min(), remaining.iter().min());
                assert_eq!(bst.max(), remaining.iter().max());
            }
        }
    }

    #[test]
    fn min_in_empty_tree() {
        let bst = BinarySearchTree::<i32>::new();

        assert_eq!(bst.min(), None);
    }

    #[test]
    fn min_in_degenerate_trees_check_updating() {
        let mut bst_degenerate_right = BinarySearchTree::new();
        let mut bst_degenerate_left = BinarySearchTree::new();

        for i in 0..=10 {
            bst_degenerate_right.insert(i);
            assert_eq!(bst_degenerate_right.min(), Some(&0));
        }
        for i in (0..=10).rev() {
            bst_degenerate_left.insert(i);
            assert_eq!(bst_degenerate_left.min(), Some(&i));
        }
    }

    #[test]
    fn min_basic_check_updating() {
        let mut bst_diff_heights_null = BinarySearchTree::new();
        let mut bst_diff_heights_one = BinarySearchTree::new();
        let mut bst_diff_heights_two = BinarySearchTree::new();

        let values_1 = vec![5, 3, 7, 2, 4, 6, 8];
        let values_2 = vec![4, 2, 6, 1, 3, 5];
        let values_3 = vec![8, 4, 12, 2, 6, 10, 14, 1, 7];

        let mut current_min_1 = None;
        let mut current_min_2 = None;
        let mut current_min_3 = None;

        for value in &values_1 {
            bst_diff_heights_null.insert(*value);
            current_min_1 = Some(*value)
                .filter(|&x| current_min_1.map_or(true, |min| x < min))
                .or(current_min_1);
            assert_eq!(bst_diff_heights_null.min(), current_min_1.as_ref());
        }

        for value in &values_2 {
            bst_diff_heights_one.insert(*value);
            current_min_2 = Some(*value)
                .filter(|&x| current_min_2.map_or(true, |min| x < min))
                .or(current_min_2);
            assert_eq!(bst_diff_heights_one.min(), current_min_2.as_ref());
        }

        for value in &values_3 {
            bst_diff_heights_two.insert(*value);
            current_min_3 = Some(*value)
                .filter(|&x| current_min_3.map_or(true, |min| x < min))
                .or(current_min_3);
            assert_eq!(bst_diff_heights_two.min(), current_min_3.as_ref());
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 111,
            ..ProptestConfig::default()
        })]
        #[test]
        fn prop_min_check_updating(values in prop::collection::vec(any::<i32>(), 1..100)) {
            let mut bst = BinarySearchTree::new();
            let mut current_min = None;

            for &v in &values {
                bst.insert(v);
                current_min = Some(v)
                    .filter(|&x| current_min.map_or(true, |min| x < min))
                    .or(current_min);
                assert_eq!(bst.min(), current_min.as_ref());
            }
        }
    }

    #[test]
    fn max_in_empty_tree() {
        let bst = BinarySearchTree::<i32>::new();

        assert_eq!(bst.max(), None);
    }

    #[test]
    fn max_in_degenerate_trees_check_updating() {
        let mut bst_degenerate_right = BinarySearchTree::new();
        let mut bst_degenerate_left = BinarySearchTree::new();

        for i in 0..=10 {
            bst_degenerate_right.insert(i);
            assert_eq!(bst_degenerate_right.max(), Some(&i));
        }
        for i in (0..=10).rev() {
            bst_degenerate_left.insert(i);
            assert_eq!(bst_degenerate_left.max(), Some(&10));
        }
    }

    #[test]
    fn max_basic_check_updating() {
        let mut bst_diff_heights_null = BinarySearchTree::new();
        let mut bst_diff_heights_one = BinarySearchTree::new();
        let mut bst_diff_heights_two = BinarySearchTree::new();

        let values_1 = vec![5, 3, 7, 2, 4, 6, 8];
        let values_2 = vec![4, 2, 6, 1, 3, 5];
        let values_3 = vec![8, 4, 12, 2, 6, 10, 14, 1, 7];

        let mut current_max_1 = None;
        let mut current_max_2 = None;
        let mut current_max_3 = None;

        for value in &values_1 {
            bst_diff_heights_null.insert(*value);
            current_max_1 = Some(*value)
                .filter(|&x| current_max_1.map_or(true, |max| x > max))
                .or(current_max_1);
            assert_eq!(bst_diff_heights_null.max(), current_max_1.as_ref());
        }

        for value in &values_2 {
            bst_diff_heights_one.insert(*value);
            current_max_2 = Some(*value)
                .filter(|&x| current_max_2.map_or(true, |max| x > max))
                .or(current_max_2);
            assert_eq!(bst_diff_heights_one.max(), current_max_2.as_ref());
        }

        for value in &values_3 {
            bst_diff_heights_two.insert(*value);
            current_max_3 = Some(*value)
                .filter(|&x| current_max_3.map_or(true, |max| x > max))
                .or(current_max_3);
            assert_eq!(bst_diff_heights_two.max(), current_max_3.as_ref());
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 111,
            ..ProptestConfig::default()
        })]
        #[test]
        fn prop_max_check_updating(values in prop::collection::vec(any::<i32>(), 1..100)) {
            let mut bst = BinarySearchTree::new();
            let mut current_max = None;

            for &v in &values {
                bst.insert(v);
                current_max = Some(v)
                    .filter(|&x| current_max.map_or(true, |max| x > max))
                    .or(current_max);
                assert_eq!(bst.max(), current_max.as_ref());
            }
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
            cases: 111,
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
    fn height_in_empty_tree() {
        let bst = BinarySearchTree::<i32>::new();

        assert_eq!(bst.height(), 0);
    }

    #[test]
    fn height_in_degenerate_trees() {
        let mut bst_degenerate_right = BinarySearchTree::new();
        let mut bst_degenerate_left = BinarySearchTree::new();

        for i in 0..=10 {
            bst_degenerate_right.insert(i);
        }
        for i in (0..=10).rev() {
            bst_degenerate_left.insert(i);
        }

        assert_eq!(bst_degenerate_right.height(), 10);
        assert_eq!(bst_degenerate_left.height(), 10);
    }

    #[test]
    fn height_basic() {
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

        assert_eq!(bst_diff_heights_null.height(), 2);
        assert_eq!(bst_diff_heights_one.height(), 2);
        assert_eq!(bst_diff_heights_two.height(), 3);
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 111,
            ..ProptestConfig::default()
        })]
        #[test]
        fn prop_height(values in prop::collection::vec(any::<i32>(), 1..111)) {
            let mut bst = BinarySearchTree::new();
            let mut bst_comparing = IterativeBSTOther::new();

            for &v in &values {
                bst.insert(v);
                bst_comparing.insert(v);
            }

            if values.is_empty() {
                assert_eq!(bst.height(), 0);
            } else {
                assert_eq!(bst.height(), bst_comparing.height().unwrap_or(0) as usize);
            }
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
            cases: 111,
            ..ProptestConfig::default()
        })]
        #[test]
        fn prop_pre_order(values in prop::collection::vec(any::<i32>(), 1..111)) {
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
            cases: 111,
            ..ProptestConfig::default()
        })]
        #[test]
        fn prop_in_order(values in prop::collection::vec(any::<i32>(), 1..111)) {
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
            cases: 111,
            ..ProptestConfig::default()
        })]
        #[test]
        fn prop_post_order(values in prop::collection::vec(any::<i32>(), 1..111)) {
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
            cases: 111,
            ..ProptestConfig::default()
        })]
        #[test]
        fn prop_level_order(values in prop::collection::vec(any::<i32>(), 1..111)) {
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
            cases: 111,
            ..ProptestConfig::default()
        })]
        #[test]
        fn prop_number_of_elements(values in prop::collection::vec(any::<i32>(), 1..111)) {
            let mut bst = BinarySearchTree::new();

            for &v in &values {
                bst.insert(v);
            }

            assert_eq!(bst.number_of_elements(), values.iter().collect::<std::collections::HashSet<_>>().len());
        }
    }

    #[test]
    fn ceil_in_empty_tree() {
        let bst = BinarySearchTree::<i32>::new();

        assert_eq!(bst.ceil(&0), None);
    }

    #[test]
    fn ceil_in_degenerate_trees() {
        let mut bst_degenerate_right = BinarySearchTree::new();
        let mut bst_degenerate_left = BinarySearchTree::new();

        for i in 0..=10 {
            let val = i as f64 / 10.0;
            bst_degenerate_right.insert(val);
        }

        for i in (0..=10).rev() {
            let val = i as f64 / 10.0;
            bst_degenerate_left.insert(val);
        }

        assert_eq!(bst_degenerate_right.ceil(&0.0), Some(&0.0));
        assert_eq!(bst_degenerate_left.ceil(&0.0), Some(&0.0));

        assert_eq!(bst_degenerate_right.ceil(&(0.03)), Some(&0.1));
        assert_eq!(bst_degenerate_left.ceil(&(0.03)), Some(&0.1));
        assert_eq!(bst_degenerate_right.ceil(&(0.07)), Some(&0.1));
        assert_eq!(bst_degenerate_left.ceil(&(0.07)), Some(&0.1));

        assert_eq!(bst_degenerate_right.ceil(&1.1), None);
        assert_eq!(bst_degenerate_left.ceil(&1.1), None);
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 111,
            ..ProptestConfig::default()
        })]
        #[test]
        fn prop_ceil(values in prop::collection::vec(any::<i32>(), 1..111)) {
            let mut bst = BinarySearchTree::new();
            for &v in &values {
                bst.insert(v);
            }

            let unique_values: Vec<i32> = values.into_iter().collect::<HashSet<_>>().into_iter().collect();

            for &v in &unique_values {
                assert_eq!(bst.ceil(&v), Some(&v));
            }

            let test_points = {
                let mut points = Vec::new();
                points.push(i32::MIN);
                points.extend(unique_values.iter().cloned());
                let mut sorted_values = unique_values.clone();
                sorted_values.sort();
                for window in sorted_values.windows(2) {
                    if window[1] > window[0] + 1 {
                        points.push(window[0] + 1);
                    }
                }

                points.push(i32::MAX);
                points
            };

            for &i in &test_points {
                let expected = unique_values.iter()
                .filter(|&&x| x >= i)
                .min()
                .copied();
            assert_eq!(bst.ceil(&i), expected.as_ref());
            }
        }
    }

    #[test]
    fn floor_in_empty_tree() {
        let bst = BinarySearchTree::<i32>::new();

        assert_eq!(bst.floor(&0), None);
    }

    #[test]
    fn floor_in_degenerate_trees() {
        let mut bst_degenerate_right = BinarySearchTree::new();
        let mut bst_degenerate_left = BinarySearchTree::new();

        for i in 0..=10 {
            let val = i as f64 / 10.0;
            bst_degenerate_right.insert(val);
        }

        for i in (0..=10).rev() {
            let val = i as f64 / 10.0;
            bst_degenerate_left.insert(val);
        }

        assert_eq!(bst_degenerate_right.floor(&0.0), Some(&0.0));
        assert_eq!(bst_degenerate_left.floor(&0.0), Some(&0.0));

        assert_eq!(bst_degenerate_right.floor(&(0.03)), Some(&0.0));
        assert_eq!(bst_degenerate_left.floor(&(0.03)), Some(&0.0));
        assert_eq!(bst_degenerate_right.floor(&(0.07)), Some(&0.0));
        assert_eq!(bst_degenerate_left.floor(&(0.07)), Some(&0.0));

        assert_eq!(bst_degenerate_right.floor(&(-0.9)), None);
        assert_eq!(bst_degenerate_left.floor(&(-0.9)), None);
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            cases: 111,
            ..ProptestConfig::default()
        })]
        #[test]
        fn prop_floor(values in prop::collection::vec(any::<i32>(), 1..111)) {
            let mut bst = BinarySearchTree::new();
            for &v in &values {
                bst.insert(v);
            }

            let unique_values: Vec<i32> = values.into_iter().collect::<HashSet<_>>().into_iter().collect();

            for &v in &unique_values {
                assert_eq!(bst.floor(&v), Some(&v));
            }

            let test_points = {
                let mut points = Vec::new();
                points.push(i32::MIN);
                points.extend(unique_values.iter().cloned());
                let mut sorted_values = unique_values.clone();
                sorted_values.sort();
                for window in sorted_values.windows(2) {
                    if window[1] > window[0] + 1 {
                        points.push(window[0] + 1);
                    }
                }

                points.push(i32::MAX);
                points
            };

            for &i in &test_points {
                let expected = unique_values.iter()
                .filter(|&&x| x <= i)
                .max()
                .copied();
            assert_eq!(bst.floor(&i), expected.as_ref());
            }
        }
    }
}
