pub struct BinaryNode<T: PartialOrd> {
    pub value: T,
    pub left: Option<Box<BinaryNode<T>>>,
    pub right: Option<Box<BinaryNode<T>>>,
}

impl<T: PartialOrd> BinaryNode<T> {
    pub fn new(value: T) -> Self {
        BinaryNode {
            value,
            left: None,
            right: None,
        }
    }
}
