/// A node in an AVL tree structure with height tracking.
///
/// Each node contains:
/// - A `value` of generic type `T`
/// - Optional left/right child nodes (wrapped in `Box`)
/// - Height information for balancing
///
/// Maintains the AVL invariant: balance factor ∈ [-1, 0, 1]
#[derive(Debug, Clone)]
pub struct AVLNode<T: PartialOrd> {
    /// The value stored in this node.
    pub value: T,

    /// Left child node (values less than parent).
    pub left: Option<Box<AVLNode<T>>>,

    /// Right child node (values greater than parent).
    pub right: Option<Box<AVLNode<T>>>,

    /// Height of this node's subtree (leaf nodes have height 1).
    pub height: usize,
}

impl<T: PartialOrd> AVLNode<T> {
    /// Creates a new `AVLNode` with the given `value` and default height (1) and no children.
    pub fn new(value: T) -> Self {
        AVLNode {
            value,
            left: None,
            right: None,
            height: 1,
        }
    }

    /// Updates this node's height based on children's heights.
    pub fn update_height(&mut self) {
        self.height = 1 + std::cmp::max(Self::height(&self.left), Self::height(&self.right));
    }

    /// Returns the height of a node (0 for `None`).
    pub fn height(node: &Option<Box<Self>>) -> usize {
        node.as_ref().map_or(0, |n| n.height)
    }

    /// Calculates the balance factor (left_height - right_height).
    ///
    /// Returns:
    /// - Positive if left-heavy
    /// - Negative if right-heavy
    /// - 0 if perfectly balanced
    pub fn balance_factor(&self) -> i32 {
        let left_height = Self::height(&self.left) as i32;
        let right_height = Self::height(&self.right) as i32;
        left_height - right_height
    }

    /// Performs automatic rebalancing if needed.
    ///
    /// Applies one of four rotation cases when balance factor is outside [-1, 1]:
    /// - Left-Left (single right rotation)
    /// - Right-Right (single left rotation)
    /// - Left-Right (double rotation)
    /// - Right-Left (double rotation)
    pub fn rebalance(self: Box<Self>) -> Box<Self> {
        match self.balance_factor() {
            bf if bf > 1 => {
                if self.left.as_ref().unwrap().balance_factor() >= 0 {
                    self.ll_rotation()
                } else {
                    self.lr_rotation()
                }
            }
            bf if bf < -1 => {
                if self.right.as_ref().unwrap().balance_factor() <= 0 {
                    self.rr_rotation()
                } else {
                    self.rl_rotation()
                }
            }
            _ => self,
        }
    }

    /// Performs a left-left case rotation.
    fn ll_rotation(mut self: Box<Self>) -> Box<Self> {
        let mut new_root = self.left.take().unwrap();
        self.left = new_root.right.take();
        self.update_height();
        new_root.right = Some(self);
        new_root.update_height();
        new_root
    }

    /// Performs a right-right case rotation.
    fn rr_rotation(mut self: Box<Self>) -> Box<Self> {
        let mut new_root = self.right.take().unwrap();
        self.right = new_root.left.take();
        self.update_height();
        new_root.left = Some(self);
        new_root.update_height();
        new_root
    }

    /// Performs a right-left case rotation.
    fn rl_rotation(mut self: Box<Self>) -> Box<Self> {
        let right = self.right.take().unwrap();
        self.right = Some(right.ll_rotation());
        self.rr_rotation()
    }

    /// Performs a left-right case rotation.
    fn lr_rotation(mut self: Box<Self>) -> Box<Self> {
        let left = self.left.take().unwrap();
        self.left = Some(left.rr_rotation());
        self.ll_rotation()
    }
}
