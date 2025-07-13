mod avl_operations;

pub mod node;

pub mod visualization;

use node::AVLNode;

#[derive(Debug)]
pub struct AVLTree<T: PartialOrd + Clone> {
    root: Option<Box<AVLNode<T>>>,
    min_value: Option<T>,
    max_value: Option<T>,
}

impl<T: Ord + Clone> AVLTree<T> {
    pub fn find_nodes_with_heights(&self) -> Vec<(T, usize)> {
        let mut nodes = Vec::new();
        Self::collect_nodes(&self.root, &mut nodes);
        nodes
    }

    fn collect_nodes(node: &Option<Box<AVLNode<T>>>, nodes: &mut Vec<(T, usize)>) {
        if let Some(node) = node {
            nodes.push((node.value.clone(), node.height));
            Self::collect_nodes(&node.left, nodes);
            Self::collect_nodes(&node.right, nodes);
        }
    }
}
