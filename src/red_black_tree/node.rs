/// Node color in a Red-Black Tree.
///
/// Red-Black Trees maintain balance using color properties:
/// - Every node is either red or black
/// - The root is always black
/// - Red nodes cannot have red children
/// - All paths from root to leaves have the same number of black nodes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}

/// A node in a Red-Black Tree structure with color tracking.
///
/// Each node contains:
/// - A `value` of generic type `T`
/// - Optional left/right child nodes (wrapped in `Box`)
/// - A color (Red or Black) for maintaining balance properties
///
/// Maintains the Red-Black Tree invariants through rebalancing operations.
#[derive(Debug, Clone)]
pub struct RBNode<T: PartialOrd> {
    /// The value stored in this node.
    pub value: T,

    /// Left child node (values less than parent).
    pub left: Option<Box<RBNode<T>>>,

    /// Right child node (values greater than parent).
    pub right: Option<Box<RBNode<T>>>,

    /// Color of this node (Red or Black).
    pub color: Color,
}

impl<T: PartialOrd> RBNode<T> {
    /// Creates a new Red `RBNode` with the given `value` and no children.
    ///
    /// New nodes are initially red as they will be rebalanced during insertion.
    pub fn new(value: T) -> Self {
        RBNode {
            value,
            left: None,
            right: None,
            color: Color::Red,
        }
    }

    /// Checks if this node is red.
    pub fn is_red(&self) -> bool {
        self.color == Color::Red
    }

    /// Checks if this node is black.
    pub fn is_black(&self) -> bool {
        self.color == Color::Black
    }

    /// Checks if a node option is red (None is considered black).
    pub fn is_red_node(node: &Option<Box<Self>>) -> bool {
        node.as_ref().map_or(false, |n| n.is_red())
    }

    /// Performs a left rotation around this node.
    ///
    /// Before:
    /// ```text
    ///     x                y
    ///    / \              / \
    ///   a   y     =>     x   c
    ///      / \          / \
    ///     b   c        a   b
    /// ```
    pub fn rotate_left(mut self: Box<Self>) -> Box<Self> {
        let mut new_root = self.right.take().expect("Right child must exist for left rotation");
        new_root.color = self.color;
        self.color = Color::Red;
        self.right = new_root.left.take();
        new_root.left = Some(self);
        new_root
    }

    /// Performs a right rotation around this node.
    ///
    /// Before:
    /// ```text
    ///       y            x
    ///      / \          / \
    ///     x   c   =>   a   y
    ///    / \              / \
    ///   a   b            b   c
    /// ```
    pub fn rotate_right(mut self: Box<Self>) -> Box<Self> {
        let mut new_root = self.left.take().expect("Left child must exist for right rotation");
        new_root.color = self.color;
        self.color = Color::Red;
        self.left = new_root.right.take();
        new_root.right = Some(self);
        new_root
    }

    /// Flips the colors of this node and its two children.
    ///
    /// Used during insertion when both children are red.
    pub fn flip_colors(&mut self) {
        self.color = match self.color {
            Color::Red => Color::Black,
            Color::Black => Color::Red,
        };
        if let Some(left) = &mut self.left {
            left.color = match left.color {
                Color::Red => Color::Black,
                Color::Black => Color::Red,
            };
        }
        if let Some(right) = &mut self.right {
            right.color = match right.color {
                Color::Red => Color::Black,
                Color::Black => Color::Red,
            };
        }
    }
}
