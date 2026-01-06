use super::*;
use node::Color;
use std::cmp::Ordering;
use std::collections::VecDeque;

impl<T: PartialOrd + Clone> RedBlackTree<T> {
    /// Creates a new empty `RedBlackTree`.
    pub fn new() -> Self {
        RedBlackTree {
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

    /// Inserts a `value` into the tree while maintaining Red-Black Tree properties.
    ///
    /// # Complexity:
    /// - Average: *O*(log n)
    /// - Worst: *O*(log n) (due to balancing)
    /// - Best: *O*(1) (empty tree)
    pub fn insert(&mut self, value: T) {
        // Update min/max values
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

        self.root = Self::insert_recursive(self.root.take(), value);

        // Ensure root is black
        if let Some(root) = &mut self.root {
            root.color = Color::Black;
        }
    }

    /// Recursively inserts a value and maintains Red-Black Tree properties.
    fn insert_recursive(node: Option<Box<RBNode<T>>>, value: T) -> Option<Box<RBNode<T>>> {
        let mut node = match node {
            None => return Some(Box::new(RBNode::new(value))),
            Some(n) => n,
        };

        match value.partial_cmp(&node.value) {
            Some(Ordering::Less) => {
                node.left = Self::insert_recursive(node.left.take(), value);
            }
            Some(Ordering::Greater) => {
                node.right = Self::insert_recursive(node.right.take(), value);
            }
            Some(Ordering::Equal) | None => {
                // Duplicate or incomparable values are not inserted
                return Some(node);
            }
        }

        // Balance the tree
        Some(Self::balance(node))
    }

    /// Balances the tree after insertion using rotations and color flips.
    ///
    /// Handles the following cases:
    /// 1. Right child is red and left child is black: rotate left
    /// 2. Left child and left-left grandchild are both red: rotate right
    /// 3. Both children are red: flip colors
    fn balance(mut node: Box<RBNode<T>>) -> Box<RBNode<T>> {
        // Case 1: Right child is red and left child is black - rotate left
        if RBNode::is_red_node(&node.right) && !RBNode::is_red_node(&node.left) {
            node = node.rotate_left();
        }

        // Case 2: Left child and left-left grandchild are both red - rotate right
        if RBNode::is_red_node(&node.left)
            && node.left.as_ref().map_or(false, |left| RBNode::is_red_node(&left.left)) {
            node = node.rotate_right();
        }

        // Case 3: Both children are red - flip colors
        if RBNode::is_red_node(&node.left) && RBNode::is_red_node(&node.right) {
            node.flip_colors();
        }

        node
    }

    /// Checks if the tree contains a `value`.
    ///
    /// # Complexity:
    /// - Average: *O*(log n)
    /// - Worst: *O*(log n) (due to balancing)
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
    /// - Worst: *O*(log n) (due to balancing)
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
    /// - Worst: *O*(log n) (due to balancing)
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
        let mut last_visited: Option<&Box<RBNode<T>>> = None;

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
    /// - Worst case: *O*(log n) - Red-Black Trees are always balanced
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
    /// - Worst case: *O*(log n) - Red-Black Trees are always balanced
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

    /// Removes a `value` from the tree while maintaining Red-Black Tree properties.
    ///
    /// # Complexity:
    /// - Average: *O*(log n)
    /// - Worst: *O*(log n) (due to balancing)
    /// - Best: *O*(1) (leaf node)
    pub fn remove(&mut self, value: &T)
    where
        T: PartialOrd + Clone,
    {
        if self.root.is_none() {
            return;
        }

        self.root = Self::remove_recursive(self.root.take(), value);

        // Ensure root is black
        if let Some(root) = &mut self.root {
            root.color = Color::Black;
        }

        self.min_value = self.refind_min();
        self.max_value = self.refind_max();
    }

    /// Recursively removes a value and maintains Red-Black Tree properties.
    fn remove_recursive(node: Option<Box<RBNode<T>>>, value: &T) -> Option<Box<RBNode<T>>> {
        let mut node = node?;

        match value.partial_cmp(&node.value) {
            Some(Ordering::Less) => {
                if node.left.is_some() {
                    // Ensure we can delete from left subtree
                    if !RBNode::is_red_node(&node.left)
                        && node.left.as_ref().map_or(false, |left| !RBNode::is_red_node(&left.left)) {
                        node = Self::move_red_left(node);
                    }
                    node.left = Self::remove_recursive(node.left.take(), value);
                }
            }
            _ => {
                // Handle equal or greater case
                if RBNode::is_red_node(&node.left) {
                    node = node.rotate_right();
                }

                // Value found at bottom
                if value.partial_cmp(&node.value) == Some(Ordering::Equal) && node.right.is_none() {
                    return None;
                }

                if node.right.is_some() {
                    // Ensure we can delete from right subtree
                    if !RBNode::is_red_node(&node.right)
                        && node.right.as_ref().map_or(false, |right| !RBNode::is_red_node(&right.left)) {
                        node = Self::move_red_right(node);
                    }

                    if value.partial_cmp(&node.value) == Some(Ordering::Equal) {
                        // Replace with successor
                        let min_value = Self::find_min(&node.right);
                        node.value = min_value.clone();
                        node.right = Self::remove_min(node.right.take());
                    } else {
                        node.right = Self::remove_recursive(node.right.take(), value);
                    }
                }
            }
        }

        Some(Self::fix_up(node))
    }

    /// Finds the minimum value in a subtree.
    fn find_min(node: &Option<Box<RBNode<T>>>) -> &T {
        let mut current = node.as_ref().unwrap();
        while let Some(left) = &current.left {
            current = left;
        }
        &current.value
    }

    /// Removes the minimum node from a subtree.
    fn remove_min(node: Option<Box<RBNode<T>>>) -> Option<Box<RBNode<T>>> {
        let mut node = node?;

        if node.left.is_none() {
            return None;
        }

        if !RBNode::is_red_node(&node.left)
            && node.left.as_ref().map_or(false, |left| !RBNode::is_red_node(&left.left)) {
            node = Self::move_red_left(node);
        }

        node.left = Self::remove_min(node.left.take());
        Some(Self::fix_up(node))
    }

    /// Moves a red node to the left to prepare for deletion.
    fn move_red_left(mut node: Box<RBNode<T>>) -> Box<RBNode<T>> {
        node.flip_colors();
        if node.right.as_ref().map_or(false, |right| RBNode::is_red_node(&right.left)) {
            if let Some(right) = node.right.take() {
                node.right = Some(right.rotate_right());
            }
            node = node.rotate_left();
            node.flip_colors();
        }
        node
    }

    /// Moves a red node to the right to prepare for deletion.
    fn move_red_right(mut node: Box<RBNode<T>>) -> Box<RBNode<T>> {
        node.flip_colors();
        if node.left.as_ref().map_or(false, |left| RBNode::is_red_node(&left.left)) {
            node = node.rotate_right();
            node.flip_colors();
        }
        node
    }

    /// Fixes up the tree after deletion to maintain Red-Black properties.
    fn fix_up(mut node: Box<RBNode<T>>) -> Box<RBNode<T>> {
        if RBNode::is_red_node(&node.right) {
            node = node.rotate_left();
        }

        if RBNode::is_red_node(&node.left)
            && node.left.as_ref().map_or(false, |left| RBNode::is_red_node(&left.left)) {
            node = node.rotate_right();
        }

        if RBNode::is_red_node(&node.left) && RBNode::is_red_node(&node.right) {
            node.flip_colors();
        }

        node
    }
}

impl<T: PartialOrd + Clone> Default for RedBlackTree<T> {
    fn default() -> Self {
        Self::new()
    }
}
