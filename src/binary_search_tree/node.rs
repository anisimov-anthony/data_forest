/// A node in a binary tree structure.
///
/// Each node contains a `value` of generic type `T`
/// and optional left/right child nodes wrapped in `Box` for heap allocation.
#[derive(Debug)]
pub struct BinaryNode<T: PartialOrd> {
    /// The value stored in this node.
    pub value: T,

    /// Left child node (less than parent value).
    pub left: Option<Box<BinaryNode<T>>>,

    /// Right child node (greater than parent value).
    pub right: Option<Box<BinaryNode<T>>>,
}

impl<T: PartialOrd> BinaryNode<T> {
    /// Creates a new `BinaryNode` with the given `value` and no children.
    pub fn new(value: T) -> Self {
        BinaryNode {
            value,
            left: None,
            right: None,
        }
    }
}
