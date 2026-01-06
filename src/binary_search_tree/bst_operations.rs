use super::*;
use std::cmp::Ordering;
use std::collections::VecDeque;

impl<T: PartialOrd + Clone> BinarySearchTree<T> {
    /// Creates a new empty `BinarySearchTree`.
    pub fn new() -> Self {
        BinarySearchTree {
            root: None,
            min_value: None,
            max_value: None,
        }
    }

    /// Checks if the tree is empty.
    ///
    /// # Complexity:
    /// *O*(1) - checks if root is `None`.
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// Inserts a `value` into the tree while maintaining tree properties (min/max values).
    ///
    /// # Complexity:
    /// - Average: *O*(log n)
    /// - Worst: *O*(n) (degenerate/unbalanced trees)
    /// - Best: *O*(1) (empty tree)
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

    /// Removes a `value` from the tree while maintaining tree properties (min/max values).
    ///
    /// # Complexity:
    /// - Average: *O*(log n)
    /// - Worst: *O*(n) (degenerate/unbalanced trees)
    /// - Best: *O*(1) (leaf node)
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

    /// Checks if the tree contains a `value`.
    ///
    /// # Complexity:
    /// - Average: *O*(log n)
    /// - Worst: *O*(n) (degenerate/unbalanced trees)
    /// - Best: *O*(1) (root match)
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
    /// # Complexity:
    /// *O*(1) (due to storing the minimum element inside the tree structure).
    pub fn min(&self) -> Option<&T> {
        self.min_value.as_ref()
    }

    /// Returns a reference to the maximum element of the tree or `None` if tree is empty.
    ///
    /// # Complexity:
    /// *O*(1) (due to storing the maximum element inside the tree structure).
    pub fn max(&self) -> Option<&T> {
        self.max_value.as_ref()
    }

    /// Each time the tree is updated, you need to re-search for the minimum.
    ///
    /// # Complexity
    /// - Average: *O*(log n)
    /// - Worst: *O*(n) (degenerate/unbalanced trees)
    /// - Best: *O*(1) (root match)
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

    /// Each time the tree is updated, you need to re-search for the maximum.
    ///
    /// # Complexity:
    /// - Average: *O*(log n)
    /// - Worst: *O*(n) (degenerate/unbalanced trees)
    /// - Best: *O*(1) (root match)
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

    /// Returns the height of the tree (longest path from root to leaf).
    ///
    /// # Complexity:
    /// *O*(n) - visits all nodes.
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

    /// Returns references to the elements of the tree in the order of a preorder traversal.
    ///
    /// # Complexity:
    /// *O*(n) - visits all nodes.
    ///
    /// # Example:
    ///
    /// If such a tree is given
    ///```text
    ///      4
    ///     / \
    ///    2   5
    ///   / \   \
    ///  1   3   6
    ///```
    ///
    /// Then the result of this traversal will be like this: `vec![&4, &2, &1, &3, &5, &6]`.
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

    /// Returns references to the elements of the tree in the order of a inorder traversal.
    ///
    /// # Complexity:
    /// *O*(n) - visits all nodes.
    ///
    /// # Example:
    ///
    /// If such a tree is given
    ///```text
    ///      4
    ///     / \
    ///    2   5
    ///   / \   \
    ///  1   3   6
    ///```
    /// Then the result of this traversal will be like this: `vec![&1, &2, &3, &4, &5, &6]`.
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

    /// Returns references to the elements of the tree in the order of a postorder traversal.
    ///
    /// # Complexity:
    /// *O*(n) - visits all nodes.
    ///
    /// # Example:
    ///
    /// If such a tree is given
    ///```text
    ///      4
    ///     / \
    ///    2   5
    ///   / \   \
    ///  1   3   6
    ///```
    /// Then the result of this traversal will be like this: `vec![&1, &3, &2, &6, &5, &4]`.
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

    /// Returns references to the elements of the tree in the order of a level order traversal.
    ///
    /// # Complexity:
    /// *O*(n) - visits all nodes.
    ///
    /// # Example:
    ///
    /// If such a tree is given
    ///```text
    ///      4
    ///     / \
    ///    2   5
    ///   / \   \
    ///  1   3   6
    ///```
    /// Then the result of this traversal will be like this: `vec![&4, &2, &5, &1, &3, &6]`.
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

    /// Returns the number of elements of the tree (the number of elements in the vector
    /// for the preorder traversal).
    ///
    /// # Complexity:
    /// *O*(n) - traverses entire tree.
    pub fn number_of_elements(&self) -> usize {
        self.pre_order().len()
    }

    /// Returns a value that is the rounded `value` to the nearest larger in the tree,
    /// or returns `None` (if the tree is empty or if such rounding is not possible for this tree and
    /// given `value`).
    ///
    /// # Complexity:
    /// - Best case: *O*(1) - when the value matches the root node
    /// - Average case: *O*(log n) - for balanced trees
    /// - Worst case: *O*(n) - for degenerate/unbalanced trees
    pub fn ceil(&self, value: &T) -> Option<&T> {
        self.root.as_ref()?;

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

    /// Returns a value that is the rounded `value` to the nearest smaller in the tree,
    /// or returns `None` (if the tree is empty or if such rounding is not possible for this tree and
    /// given `value`).
    ///
    /// # Complexity:
    /// - Best case: *O*(1) - when the value matches the root node  
    /// - Average case: *O*(log n) - for balanced trees  
    /// - Worst case: *O*(n) - for degenerate/unbalanced trees  
    pub fn floor(&self, value: &T) -> Option<&T> {
        self.root.as_ref()?;

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

    /// Performs a tree traversal and returns all pairs of connections between nodes.
    pub fn find_connections(&self) -> Vec<(&T, &T)> {
        let mut result = Vec::new();
        let mut queue = std::collections::VecDeque::new();

        if let Some(root) = &self.root {
            queue.push_back(root);
        }

        while let Some(node) = queue.pop_front() {
            if let Some(left) = &node.left {
                queue.push_back(left);
                result.push((&node.value, &left.value));
            }
            if let Some(right) = &node.right {
                queue.push_back(right);
                result.push((&node.value, &right.value));
            }
        }

        result
    }
}

impl<T: PartialOrd + Clone> Default for BinarySearchTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_and_isnt_empty_tree() {
        let bst_1 = BinarySearchTree::<i32>::new();
        assert!(bst_1.is_empty());

        let mut bst_2 = BinarySearchTree::<i32>::new();
        bst_2.insert(42);
        assert!(!bst_2.is_empty());
    }

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
        let values_3 = vec![8, 4, 12, 2, 6, 10, 1, 7];
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
    fn remove_from_single_node_tree_check_min_max_updating() {
        let mut bst = BinarySearchTree::new();
        assert!(bst.max() == bst.min() && bst.max().is_none());

        bst.insert(1);
        assert!(bst.max() == bst.min() && bst.max() == Some(&1));
        assert!(bst.contains(&1));

        bst.remove(&1);
        assert!(bst.max() == bst.min() && bst.max().is_none());
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
        let values3 = vec![8, 4, 12, 2, 6, 10, 1, 7];

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
        let values_3 = vec![8, 4, 12, 2, 6, 10, 1, 7];

        let mut current_min_1 = None;
        let mut current_min_2 = None;
        let mut current_min_3 = None;

        for value in &values_1 {
            bst_diff_heights_null.insert(*value);
            current_min_1 = Some(*value)
                .filter(|&x| current_min_1.is_none_or(|min| x < min))
                .or(current_min_1);
            assert_eq!(bst_diff_heights_null.min(), current_min_1.as_ref());
        }

        for value in &values_2 {
            bst_diff_heights_one.insert(*value);
            current_min_2 = Some(*value)
                .filter(|&x| current_min_2.is_none_or(|min| x < min))
                .or(current_min_2);
            assert_eq!(bst_diff_heights_one.min(), current_min_2.as_ref());
        }

        for value in &values_3 {
            bst_diff_heights_two.insert(*value);
            current_min_3 = Some(*value)
                .filter(|&x| current_min_3.is_none_or(|min| x < min))
                .or(current_min_3);
            assert_eq!(bst_diff_heights_two.min(), current_min_3.as_ref());
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
        let values_3 = vec![8, 4, 12, 2, 6, 10, 1, 7];

        let mut current_max_1 = None;
        let mut current_max_2 = None;
        let mut current_max_3 = None;

        for value in &values_1 {
            bst_diff_heights_null.insert(*value);
            current_max_1 = Some(*value)
                .filter(|&x| current_max_1.is_none_or(|max| x > max))
                .or(current_max_1);
            assert_eq!(bst_diff_heights_null.max(), current_max_1.as_ref());
        }

        for value in &values_2 {
            bst_diff_heights_one.insert(*value);
            current_max_2 = Some(*value)
                .filter(|&x| current_max_2.is_none_or(|max| x > max))
                .or(current_max_2);
            assert_eq!(bst_diff_heights_one.max(), current_max_2.as_ref());
        }

        for value in &values_3 {
            bst_diff_heights_two.insert(*value);
            current_max_3 = Some(*value)
                .filter(|&x| current_max_3.is_none_or(|max| x > max))
                .or(current_max_3);
            assert_eq!(bst_diff_heights_two.max(), current_max_3.as_ref());
        }
    }

    #[test]
    fn max_min_are_similar_for_single_element_tree() {
        let mut bst = BinarySearchTree::new();
        bst.insert(1);

        assert!(bst.min() == bst.max() && bst.min() == Some(&1));
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
        let values_3 = vec![8, 4, 12, 2, 6, 10, 1, 7];
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
        let values_3 = vec![8, 4, 12, 2, 6, 10, 1, 7];
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
            vec![&&8, &&4, &&2, &&1, &&6, &&7, &&12, &&10]
        );
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
        let values_3 = vec![8, 4, 12, 2, 6, 10, 1, 7];
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
            vec![&&1, &&2, &&4, &&6, &&7, &&8, &&10, &&12]
        );
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
        let values_3 = vec![8, 4, 12, 2, 6, 10, 1, 7];
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
            vec![&&1, &&2, &&7, &&6, &&4, &&10, &&12, &&8]
        );
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
        let values_3 = vec![8, 4, 12, 2, 6, 10, 1, 7];
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
            vec![&&8, &&4, &&12, &&2, &&6, &&10, &&1, &&7]
        );
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
        let values_3 = vec![8, 4, 12, 2, 6, 10, 1, 7];
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

    #[test]
    fn ceil_basic() {
        let mut bst = BinarySearchTree::<i32>::new();

        bst.insert(1);
        bst.insert(2);
        bst.insert(5);

        assert_eq!(bst.ceil(&6), None);
        assert_eq!(bst.ceil(&5), Some(&5));
        assert_eq!(bst.ceil(&4), Some(&5));
        assert_eq!(bst.ceil(&3), Some(&5));
        assert_eq!(bst.ceil(&2), Some(&2));
        assert_eq!(bst.ceil(&1), Some(&1));
        assert_eq!(bst.ceil(&0), Some(&1));
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

    #[test]
    fn floor_basic() {
        let mut bst = BinarySearchTree::<i32>::new();

        bst.insert(1);
        bst.insert(2);
        bst.insert(5);

        assert_eq!(bst.floor(&6), Some(&5));
        assert_eq!(bst.floor(&5), Some(&5));
        assert_eq!(bst.floor(&4), Some(&2));
        assert_eq!(bst.floor(&3), Some(&2));
        assert_eq!(bst.floor(&2), Some(&2));
        assert_eq!(bst.floor(&1), Some(&1));
        assert_eq!(bst.floor(&0), None);
    }
}
