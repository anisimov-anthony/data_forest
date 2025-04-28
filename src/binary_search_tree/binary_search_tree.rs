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

    pub fn contains(&self, value: T) -> bool {
        let mut cursor = &self.root;

        while let Some(current_node) = cursor {
            if current_node.value < value {
                cursor = &current_node.right;
            } else if current_node.value > value {
                cursor = &current_node.left;
            } else {
                return true;
            }
        }

        false
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
        bst.insert(5);
        bst.insert(10);
        bst.insert(5);
        bst.insert(6);
        bst.insert(4);
        bst.insert(100);
        bst.insert(3);

        // Assert
        assert_eq!(bst.contains(5), true);
        assert_eq!(bst.contains(10), true);
        assert_eq!(bst.contains(6), true);
        assert_eq!(bst.contains(4), true);
        assert_eq!(bst.contains(100), true);
        assert_eq!(bst.contains(3), true);
    }
}
