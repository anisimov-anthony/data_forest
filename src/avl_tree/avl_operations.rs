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
    /// # Complexity
    /// *O*(1) - checks if root is `None`
    ///
    /// The logic is the same as in `BST`
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

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

    fn pass_and_detach_local_minimum(root: &mut Option<Box<AVLNode<T>>>) -> Option<T> {
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

    /// Returns a reference to the minimum element of the tree or `None` if tree is empty.
    ///
    /// # Complexity:
    /// *O*(1) (due to storing the minimum element inside the tree structure).
    ///
    /// The logic is the same as in `BST`
    pub fn min(&self) -> Option<&T> {
        self.min_value.as_ref()
    }

    /// Returns a reference to the maximum element of the tree or `None` if tree is empty.
    ///
    /// # Complexity:
    /// *O*(1) (due to storing the maximum element inside the tree structure).
    ///
    /// The logic is the same as in `BST`
    pub fn max(&self) -> Option<&T> {
        self.max_value.as_ref()
    }

    ///
    /// The logic is the same as in `BST`
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

    ///
    /// The logic is the same as in `BST`
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

    ///
    /// The logic is the same as in `BST`
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

    ///
    /// The logic is the same as in `BST`
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

    ///
    /// The logic is the same as in `BST`
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

    ///
    /// The logic is the same as in `BST`
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

    ///
    /// The logic is the same as in `BST`
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

    ///
    /// The logic is the same as in `BST`
    pub fn number_of_elements(&self) -> usize {
        self.pre_order().len()
    }

    ///
    /// The logic is the same as in `BST`
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

    ///
    /// The logic is the same as in `BST`
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

    ///
    /// The logic is the same as in `BST`
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
    /// The logic is the same as in `BST`
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
