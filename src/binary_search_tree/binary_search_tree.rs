use super::node::BinaryNode;

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
    fn insert_contains_basic() {
        // Arrange
        let mut bst = BinarySearchTree::new();

        // Act
        let values = vec![5, 10, 6, 10, 4, 100, 3];
        for value in &values {
            bst.insert(value);
        }

        // Assert
        for value in &values {
            assert!(bst.contains(&value));
        }
    }

    #[test]
    fn min_basic() {
        let mut bst = BinarySearchTree::new();

        // Act
        let values = vec![5, 10, 3, 10, 4, 100, 6, 9];
        for value in &values {
            bst.insert(value);
        }

        // Assert
        assert_eq!(bst.min(), Some(&3).as_ref());
    }

    #[test]
    fn max_basic() {
        let mut bst = BinarySearchTree::new();

        // Act
        let values = vec![5, 10, 6, 100, 4, 10, 3, 31, 1];
        for value in &values {
            bst.insert(value);
        }

        // Assert
        assert_eq!(bst.max(), Some(&100).as_ref());
    }
}
