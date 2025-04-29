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
    use rand::seq::SliceRandom;
    use rand::thread_rng;

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
    fn remove_basic() {
        // Arrange
        let mut bst = BinarySearchTree::new();
        let mut values = vec![
            8, 3, 10, 1, 6, 14, 4, 7, 13, 11, 23, 1, 9, -99, 7, 32, 67, 5, 2, 17,
        ];
        let mut rng = thread_rng();
        values.shuffle(&mut rng);

        for value in &values {
            bst.insert(*value);
        }

        for value in &values {
            assert!(bst.contains(value));
        }

        let mut rng = thread_rng();
        values.shuffle(&mut rng);

        // Act & Assert
        for value in &values {
            bst.remove(value);
            assert!(!bst.contains(value));
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
