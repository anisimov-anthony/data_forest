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
