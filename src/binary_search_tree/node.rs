pub struct BinaryNode<T: Ord> {
    pub value: T,
    pub left: Option<Box<BinaryNode<T>>>,
    pub right: Option<Box<BinaryNode<T>>>,
}

impl<T: Ord> BinaryNode<T> {
    pub fn new(value: T) -> Self {
        BinaryNode {
            value,
            left: None,
            right: None,
        }
    }
}
