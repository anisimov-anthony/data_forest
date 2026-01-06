use super::*;
use std::cmp::Ordering;
use std::collections::VecDeque;

impl<T: PartialOrd + Clone> AVLTree<T> {
    /// Creates a new empty `AVLTree`.
    pub fn new() -> Self {
        AVLTree {
            root: None,
            min_value: None,
            max_value: None,
        }
    }

    /// Checks if the tree is empty.
    ///
    /// # Complexity:
    /// *O*(1) - checks if root is `None`.
    ///
    /// The logic is the same as in `BST`.
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// Inserts a `value` into the tree while maintaining AVL balance properties.
    ///
    /// Automatically performs rotations to maintain balance factor ∈ [-1, 0, 1].
    ///
    /// # Complexity:
    /// *O*(log n) - guaranteed due to AVL balancing.
    pub fn insert(&mut self, value: T) {
        self.root = AVLTree::insert_rec(self.root.take(), value);

        self.min_value = self.refind_min();
        self.max_value = self.refind_max();
    }

    fn insert_rec(node: Option<Box<AVLNode<T>>>, value: T) -> Option<Box<AVLNode<T>>> {
        match node {
            None => Some(Box::new(AVLNode::new(value))),
            Some(mut n) => {
                match value.partial_cmp(&n.value) {
                    Some(Ordering::Less) => {
                        n.left = AVLTree::insert_rec(n.left.take(), value);
                    }
                    Some(Ordering::Greater) => {
                        n.right = AVLTree::insert_rec(n.right.take(), value);
                    }
                    _ => return Some(n),
                }

                n.update_height();
                Some(n.rebalance())
            }
        }
    }

    /// Removes a `value` from the tree while maintaining AVL balance properties.
    ///
    /// Performs automatic rebalancing through rotations after deletion.
    ///
    /// # Complexity:
    /// *O*(log n) - guaranteed due to AVL balancing.
    pub fn remove(&mut self, value: &T)
    where
        T: PartialOrd + Clone,
    {
        self.root = Self::remove_node(self.root.take(), value);

        self.min_value = self.refind_min();
        self.max_value = self.refind_max();
    }

    fn remove_node(node: Option<Box<AVLNode<T>>>, value: &T) -> Option<Box<AVLNode<T>>>
    where
        T: PartialOrd + Clone,
    {
        match node {
            None => None,
            Some(mut n) => {
                match value.partial_cmp(&n.value) {
                    Some(Ordering::Less) => {
                        n.left = Self::remove_node(n.left.take(), value);
                    }
                    Some(Ordering::Greater) => {
                        n.right = Self::remove_node(n.right.take(), value);
                    }
                    Some(Ordering::Equal) => {
                        // Found the node to delete
                        return match (n.left.take(), n.right.take()) {
                            (None, None) => None,
                            (Some(left), None) => Some(left),
                            (None, Some(right)) => Some(right),
                            (Some(left), Some(right)) => {
                                let (min_val, new_right) = Self::detach_min(right);
                                n.value = min_val;
                                n.right = new_right;
                                n.left = Some(left);
                                n.update_height();
                                Some(Box::new(*n.rebalance()))
                            }
                        };
                    }
                    None => return Some(n),
                }

                n.update_height();
                Some(Box::new(*n.rebalance()))
            }
        }
    }

    fn detach_min(mut node: Box<AVLNode<T>>) -> (T, Option<Box<AVLNode<T>>>)
    where
        T: PartialOrd + Clone,
    {
        if let Some(left) = node.left.take() {
            let (min_val, new_left) = Self::detach_min(left);
            node.left = new_left;
            node.update_height();
            let balanced = node.rebalance();
            (min_val, Some(Box::new(*balanced)))
        } else {
            (node.value.clone(), node.right)
        }
    }

    /// Checks if the tree contains a `value`.
    ///
    /// # Complexity:
    /// *O*(log n) - guaranteed due to AVL balancing.
    ///
    /// The logic is the same as in `BST`.
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
    ///
    /// The logic is the same as in `BST`.
    pub fn min(&self) -> Option<&T> {
        self.min_value.as_ref()
    }

    /// Returns a reference to the maximum element of the tree or `None` if tree is empty.
    ///
    /// # Complexity:
    /// *O*(1) (due to storing the maximum element inside the tree structure).
    ///
    /// The logic is the same as in `BST`.
    pub fn max(&self) -> Option<&T> {
        self.max_value.as_ref()
    }

    /// Each time the tree is updated, you need to re-search for the minimum.
    ///
    /// # Complexity:
    /// *O*(log n) - guaranteed due to AVL balancing.
    ///
    /// The logic is the same as in `BST`.
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
    /// *O*(log n) - guaranteed due to AVL balancing.
    ///
    /// The logic is the same as in `BST`.
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
    ///
    /// The logic is the same as in `BST`.
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
    ///
    /// The logic is the same as in `BST`.
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
    ///
    /// The logic is the same as in `BST`.
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
    ///
    /// The logic is the same as in `BST`.
    pub fn post_order(&self) -> Vec<&T> {
        let mut result = Vec::new();
        let mut stack = Vec::new();
        let mut current = &self.root;
        let mut last_visited: Option<&Box<AVLNode<T>>> = None;

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
    ///
    /// The logic is the same as in `BST`.
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
    ///
    /// The logic is the same as in `BST`.
    pub fn number_of_elements(&self) -> usize {
        self.pre_order().len()
    }

    /// Returns a value that is the rounded `value` to the nearest larger in the tree,
    /// or returns `None` (if the tree is empty or if such rounding is not possible for this tree and
    /// given `value`).
    ///
    /// # Complexity:
    /// *O*(log n) - guaranteed due to AVL balancing.
    ///
    /// The logic is the same as in `BST`.
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
    /// *O*(log n) - guaranteed due to AVL balancing.
    ///
    /// The logic is the same as in `BST`.
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
    ///
    /// The logic is the same as in `BST`.
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

impl<T: PartialOrd + Clone> Default for AVLTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_and_isnt_empty_tree() {
        let avl_1 = AVLTree::<i32>::new();
        assert!(avl_1.is_empty());

        let mut avl_2 = AVLTree::<i32>::new();
        avl_2.insert(42);
        assert!(!avl_2.is_empty());
    }

    #[test]
    fn contains_in_empty_tree() {
        let avl = AVLTree::<i32>::new();

        assert!(!avl.contains(&42));
    }

    #[test]
    fn contains_in_single_node_tree() {
        let mut avl = AVLTree::new();
        avl.insert(1);

        assert!(avl.contains(&1));
    }

    #[test]
    fn contains_basic() {
        let mut avl_diff_heights_null = AVLTree::new();
        let mut avl_diff_heights_one = AVLTree::new();
        let mut avl_diff_heights_two = AVLTree::new();

        let values_1 = vec![5, 3, 7, 2, 4, 6, 8];
        let values_2 = vec![4, 2, 6, 1, 3, 5];
        let values_3 = vec![8, 4, 12, 2, 6, 10, 1, 7];
        for value in &values_1 {
            avl_diff_heights_null.insert(value);
        }
        for value in &values_2 {
            avl_diff_heights_one.insert(value);
        }
        for value in &values_3 {
            avl_diff_heights_two.insert(value);
        }

        for value in &values_1 {
            assert!(avl_diff_heights_null.contains(&value));
        }
        for value in &values_2 {
            assert!(avl_diff_heights_one.contains(&value));
        }
        for value in &values_3 {
            assert!(avl_diff_heights_two.contains(&value));
        }
    }

    #[test]
    fn remove_from_empty_tree() {
        let mut avl = AVLTree::<i32>::new();

        avl.remove(&42);

        assert!(!avl.contains(&42));
        assert_eq!(avl.min(), None);
        assert_eq!(avl.max(), None);
    }

    #[test]
    fn remove_from_single_node_tree_check_min_max_updating() {
        let mut avl = AVLTree::new();
        assert!(avl.max() == avl.min() && avl.max().is_none());

        avl.insert(1);
        assert!(avl.max() == avl.min() && avl.max() == Some(&1));
        assert!(avl.contains(&1));

        avl.remove(&1);
        assert!(avl.max() == avl.min() && avl.max().is_none());
        assert!(!avl.contains(&1));
    }

    #[test]
    fn remove_basic_check_min_max_updating() {
        let mut avl1 = AVLTree::new();
        let mut avl2 = AVLTree::new();
        let mut avl3 = AVLTree::new();

        let values1 = vec![5, 3, 7, 2, 4, 6, 8];
        let values2 = vec![4, 2, 6, 1, 3, 5];
        let values3 = vec![8, 4, 12, 2, 6, 10, 1, 7];

        for &v in &values1 {
            avl1.insert(v);
        }
        for &v in &values2 {
            avl2.insert(v);
        }
        for &v in &values3 {
            avl3.insert(v);
        }

        let mut remaining1 = values1.clone();
        let mut remaining2 = values2.clone();
        let mut remaining3 = values3.clone();

        for &v in &values1 {
            avl1.remove(&v);
            remaining1.retain(|&x| x != v);

            assert!(!avl1.contains(&v));
            assert_eq!(avl1.min(), remaining1.iter().min());
            assert_eq!(avl1.max(), remaining1.iter().max());
        }

        for &v in &values2 {
            avl2.remove(&v);
            remaining2.retain(|&x| x != v);

            assert!(!avl2.contains(&v));
            assert_eq!(avl2.min(), remaining2.iter().min());
            assert_eq!(avl2.max(), remaining2.iter().max());
        }

        for &v in &values3 {
            avl3.remove(&v);
            remaining3.retain(|&x| x != v);

            assert!(!avl3.contains(&v));
            assert_eq!(avl3.min(), remaining3.iter().min());
            assert_eq!(avl3.max(), remaining3.iter().max());
        }
    }

    #[test]
    fn min_in_empty_tree() {
        let avl = AVLTree::<i32>::new();

        assert_eq!(avl.min(), None);
    }

    #[test]
    fn min_basic_check_updating() {
        let mut avl_diff_heights_null = AVLTree::new();
        let mut avl_diff_heights_one = AVLTree::new();
        let mut avl_diff_heights_two = AVLTree::new();

        let values_1 = vec![5, 3, 7, 2, 4, 6, 8];
        let values_2 = vec![4, 2, 6, 1, 3, 5];
        let values_3 = vec![8, 4, 12, 2, 6, 10, 1, 7];

        let mut current_min_1 = None;
        let mut current_min_2 = None;
        let mut current_min_3 = None;

        for value in &values_1 {
            avl_diff_heights_null.insert(*value);
            current_min_1 = Some(*value)
                .filter(|&x| current_min_1.is_none_or(|min| x < min))
                .or(current_min_1);
            assert_eq!(avl_diff_heights_null.min(), current_min_1.as_ref());
        }

        for value in &values_2 {
            avl_diff_heights_one.insert(*value);
            current_min_2 = Some(*value)
                .filter(|&x| current_min_2.is_none_or(|min| x < min))
                .or(current_min_2);
            assert_eq!(avl_diff_heights_one.min(), current_min_2.as_ref());
        }

        for value in &values_3 {
            avl_diff_heights_two.insert(*value);
            current_min_3 = Some(*value)
                .filter(|&x| current_min_3.is_none_or(|min| x < min))
                .or(current_min_3);
            assert_eq!(avl_diff_heights_two.min(), current_min_3.as_ref());
        }
    }

    #[test]
    fn max_in_empty_tree() {
        let avl = AVLTree::<i32>::new();

        assert_eq!(avl.max(), None);
    }

    #[test]
    fn max_basic_check_updating() {
        let mut avl_diff_heights_null = AVLTree::new();
        let mut avl_diff_heights_one = AVLTree::new();
        let mut avl_diff_heights_two = AVLTree::new();

        let values_1 = vec![5, 3, 7, 2, 4, 6, 8];
        let values_2 = vec![4, 2, 6, 1, 3, 5];
        let values_3 = vec![8, 4, 12, 2, 6, 10, 1, 7];

        let mut current_max_1 = None;
        let mut current_max_2 = None;
        let mut current_max_3 = None;

        for value in &values_1 {
            avl_diff_heights_null.insert(*value);
            current_max_1 = Some(*value)
                .filter(|&x| current_max_1.is_none_or(|max| x > max))
                .or(current_max_1);
            assert_eq!(avl_diff_heights_null.max(), current_max_1.as_ref());
        }

        for value in &values_2 {
            avl_diff_heights_one.insert(*value);
            current_max_2 = Some(*value)
                .filter(|&x| current_max_2.is_none_or(|max| x > max))
                .or(current_max_2);
            assert_eq!(avl_diff_heights_one.max(), current_max_2.as_ref());
        }

        for value in &values_3 {
            avl_diff_heights_two.insert(*value);
            current_max_3 = Some(*value)
                .filter(|&x| current_max_3.is_none_or(|max| x > max))
                .or(current_max_3);
            assert_eq!(avl_diff_heights_two.max(), current_max_3.as_ref());
        }
    }

    #[test]
    fn max_min_are_similar_for_single_element_tree() {
        let mut avl = AVLTree::new();
        avl.insert(1);

        assert!(avl.min() == avl.max() && avl.min() == Some(&1));
    }

    #[test]
    fn height_in_empty_tree() {
        let avl = AVLTree::<i32>::new();

        assert_eq!(avl.height(), 0);
    }

    #[test]
    fn height_basic() {
        let mut avl_diff_heights_null = AVLTree::new();
        let mut avl_diff_heights_one = AVLTree::new();
        let mut avl_diff_heights_two = AVLTree::new();

        let values_1 = vec![5, 3, 7, 2, 4, 6, 8];
        let values_2 = vec![4, 2, 6, 1, 3, 5];
        let values_3 = vec![8, 4, 12, 2, 6, 10, 1, 7];
        for value in &values_1 {
            avl_diff_heights_null.insert(value);
        }
        for value in &values_2 {
            avl_diff_heights_one.insert(value);
        }
        for value in &values_3 {
            avl_diff_heights_two.insert(value);
        }

        assert_eq!(avl_diff_heights_null.height(), 2);
        assert_eq!(avl_diff_heights_one.height(), 2);
        assert_eq!(avl_diff_heights_two.height(), 3);
    }

    #[test]
    fn pre_order_in_empty_tree() {
        let avl = AVLTree::<i32>::new();

        assert_eq!(avl.pre_order(), Vec::<&i32>::new());
    }

    #[test]
    fn pre_order_basic() {
        let mut avl_diff_heights_null = AVLTree::new();
        let mut avl_diff_heights_one = AVLTree::new();
        let mut avl_diff_heights_two = AVLTree::new();

        let values_1 = vec![5, 3, 7, 2, 4, 6, 8];
        let values_2 = vec![4, 2, 6, 1, 3, 5];
        let values_3 = vec![8, 4, 12, 2, 6, 10, 1, 7];
        for value in &values_1 {
            avl_diff_heights_null.insert(value);
        }
        for value in &values_2 {
            avl_diff_heights_one.insert(value);
        }
        for value in &values_3 {
            avl_diff_heights_two.insert(value);
        }

        assert_eq!(
            avl_diff_heights_null.pre_order(),
            vec![&&5, &&3, &&2, &&4, &&7, &&6, &&8]
        );

        assert_eq!(
            avl_diff_heights_one.pre_order(),
            vec![&&4, &&2, &&1, &&3, &&6, &&5]
        );

        assert_eq!(
            avl_diff_heights_two.pre_order(),
            vec![&&8, &&4, &&2, &&1, &&6, &&7, &&12, &&10]
        );
    }

    #[test]
    fn in_order_in_empty_tree() {
        let avl = AVLTree::<i32>::new();

        assert_eq!(avl.in_order(), Vec::<&i32>::new());
    }

    #[test]
    fn in_order_basic() {
        let mut avl_diff_heights_null = AVLTree::new();
        let mut avl_diff_heights_one = AVLTree::new();
        let mut avl_diff_heights_two = AVLTree::new();

        let values_1 = vec![5, 3, 7, 2, 4, 6, 8];
        let values_2 = vec![4, 2, 6, 1, 3, 5];
        let values_3 = vec![8, 4, 12, 2, 6, 10, 1, 7];
        for value in &values_1 {
            avl_diff_heights_null.insert(value);
        }
        for value in &values_2 {
            avl_diff_heights_one.insert(value);
        }
        for value in &values_3 {
            avl_diff_heights_two.insert(value);
        }

        assert_eq!(
            avl_diff_heights_null.in_order(),
            vec![&&2, &&3, &&4, &&5, &&6, &&7, &&8]
        );

        assert_eq!(
            avl_diff_heights_one.in_order(),
            vec![&&1, &&2, &&3, &&4, &&5, &&6]
        );

        assert_eq!(
            avl_diff_heights_two.in_order(),
            vec![&&1, &&2, &&4, &&6, &&7, &&8, &&10, &&12]
        );
    }

    #[test]
    fn post_order_in_empty_tree() {
        let avl = AVLTree::<i32>::new();

        assert_eq!(avl.post_order(), Vec::<&i32>::new());
    }

    #[test]
    fn post_order_basic() {
        let mut avl_diff_heights_null = AVLTree::new();
        let mut avl_diff_heights_one = AVLTree::new();
        let mut avl_diff_heights_two = AVLTree::new();

        let values_1 = vec![5, 3, 7, 2, 4, 6, 8];
        let values_2 = vec![4, 2, 6, 1, 3, 5];
        let values_3 = vec![8, 4, 12, 2, 6, 10, 1, 7];
        for value in &values_1 {
            avl_diff_heights_null.insert(value);
        }
        for value in &values_2 {
            avl_diff_heights_one.insert(value);
        }
        for value in &values_3 {
            avl_diff_heights_two.insert(value);
        }

        assert_eq!(
            avl_diff_heights_null.post_order(),
            vec![&&2, &&4, &&3, &&6, &&8, &&7, &&5]
        );

        assert_eq!(
            avl_diff_heights_one.post_order(),
            vec![&&1, &&3, &&2, &&5, &&6, &&4]
        );

        assert_eq!(
            avl_diff_heights_two.post_order(),
            vec![&&1, &&2, &&7, &&6, &&4, &&10, &&12, &&8]
        );
    }

    #[test]
    fn level_order_in_empty_tree() {
        let avl = AVLTree::<i32>::new();

        assert_eq!(avl.level_order(), Vec::<&i32>::new());
    }

    #[test]
    fn level_order_basic() {
        let mut avl_diff_heights_null = AVLTree::new();
        let mut avl_diff_heights_one = AVLTree::new();
        let mut avl_diff_heights_two = AVLTree::new();

        let values_1 = vec![5, 3, 7, 2, 4, 6, 8];
        let values_2 = vec![4, 2, 6, 1, 3, 5];
        let values_3 = vec![8, 4, 12, 2, 6, 10, 1, 7];
        for value in &values_1 {
            avl_diff_heights_null.insert(value);
        }
        for value in &values_2 {
            avl_diff_heights_one.insert(value);
        }
        for value in &values_3 {
            avl_diff_heights_two.insert(value);
        }

        assert_eq!(
            avl_diff_heights_null.level_order(),
            vec![&&5, &&3, &&7, &&2, &&4, &&6, &&8]
        );

        assert_eq!(
            avl_diff_heights_one.level_order(),
            vec![&&4, &&2, &&6, &&1, &&3, &&5]
        );

        assert_eq!(
            avl_diff_heights_two.level_order(),
            vec![&&8, &&4, &&12, &&2, &&6, &&10, &&1, &&7]
        );
    }

    #[test]
    fn number_of_elements_in_empty_tree() {
        let avl = AVLTree::<i32>::new();

        assert_eq!(avl.number_of_elements(), 0);
    }

    #[test]
    fn number_of_elements_basic() {
        let mut avl_diff_heights_null = AVLTree::new();
        let mut avl_diff_heights_one = AVLTree::new();
        let mut avl_diff_heights_two = AVLTree::new();

        let values_1 = vec![5, 3, 7, 2, 4, 6, 8];
        let values_2 = vec![4, 2, 6, 1, 3, 5];
        let values_3 = vec![8, 4, 12, 2, 6, 10, 1, 7];
        for value in &values_1 {
            avl_diff_heights_null.insert(value);
        }
        for value in &values_2 {
            avl_diff_heights_one.insert(value);
        }
        for value in &values_3 {
            avl_diff_heights_two.insert(value);
        }

        assert_eq!(avl_diff_heights_null.number_of_elements(), values_1.len());
        assert_eq!(avl_diff_heights_one.number_of_elements(), values_2.len());
        assert_eq!(avl_diff_heights_two.number_of_elements(), values_3.len());
    }

    #[test]
    fn ceil_in_empty_tree() {
        let avl = AVLTree::<i32>::new();

        assert_eq!(avl.ceil(&0), None);
    }

    #[test]
    fn ceil_basic() {
        let mut avl = AVLTree::<i32>::new();

        avl.insert(1);
        avl.insert(2);
        avl.insert(5);

        assert_eq!(avl.ceil(&6), None);
        assert_eq!(avl.ceil(&5), Some(&5));
        assert_eq!(avl.ceil(&4), Some(&5));
        assert_eq!(avl.ceil(&3), Some(&5));
        assert_eq!(avl.ceil(&2), Some(&2));
        assert_eq!(avl.ceil(&1), Some(&1));
        assert_eq!(avl.ceil(&0), Some(&1));
    }

    #[test]
    fn floor_in_empty_tree() {
        let avl = AVLTree::<i32>::new();

        assert_eq!(avl.floor(&0), None);
    }

    #[test]
    fn floor_basic() {
        let mut avl = AVLTree::<i32>::new();

        avl.insert(1);
        avl.insert(2);
        avl.insert(5);

        assert_eq!(avl.floor(&6), Some(&5));
        assert_eq!(avl.floor(&5), Some(&5));
        assert_eq!(avl.floor(&4), Some(&2));
        assert_eq!(avl.floor(&3), Some(&2));
        assert_eq!(avl.floor(&2), Some(&2));
        assert_eq!(avl.floor(&1), Some(&1));
        assert_eq!(avl.floor(&0), None);
    }
}
