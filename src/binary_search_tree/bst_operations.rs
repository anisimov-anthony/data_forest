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
    /// # Complexity
    /// *O*(1) - checks if root is `None`
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    /// Inserts a `value` into the tree while maintaining tree properties (min/max values).
    ///
    /// # Complexity
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
    /// # Complexity
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
    /// # Complexity
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

    /// Returns the height of the tree (longest path from root to leaf).
    ///
    /// # Complexity:
    /// *O*(n) - visits all nodes
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
    /// *O*(n) - visits all nodes
    ///
    /// # Example:
    ///
    /// If such a tree is given
    ///```text
    ///      1
    ///     / \
    ///    2   3
    ///   / \   \
    ///  4   5   6
    ///```
    ///
    /// Then the result of this traversal will be like this: `vec![&1, &2, &4, &5, &3, &6]`
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
    /// *O*(n) - visits all nodes
    ///
    /// # Example:
    ///
    /// If such a tree is given
    ///```text
    ///      1
    ///     / \
    ///    2   3
    ///   / \   \
    ///  4   5   6
    ///```
    /// Then the result of this traversal will be like this: `vec![&4, &2, &5, &1, &3, &6]`
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
    /// *O*(n) - visits all nodes
    ///
    /// # Example:
    ///
    /// If such a tree is given
    ///```text
    ///      1
    ///     / \
    ///    2   3
    ///   / \   \
    ///  4   5   6
    ///```
    /// Then the result of this traversal will be like this: `vec![&4, &5, &2, &6, &3, &1]`
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
    /// *O*(n) - visits all nodes
    ///
    /// # Example:
    ///
    /// If such a tree is given
    ///```text
    ///      1
    ///     / \
    ///    2   3
    ///   / \   \
    ///  4   5   6
    ///```
    /// Then the result of this traversal will be like this: `vec![&1, &2, &3, &4, &5, &6]`
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
    /// *O*(n) - traverses entire tree
    pub fn number_of_elements(&self) -> usize {
        self.pre_order().len()
    }

    /// Returns a value that is the rounded `value` to the nearest larger in the tree,
    /// or returns `None` (if the tree is empty or if such rounding is not possible for this tree and
    /// given `value`).
    ///
    /// # Complexity
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
    /// # Complexity  
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
