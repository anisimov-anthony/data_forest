#[derive(Debug, Clone)]
pub struct AVLNode<T: PartialOrd> {
    pub value: T,
    pub left: Option<Box<AVLNode<T>>>,
    pub right: Option<Box<AVLNode<T>>>,
    pub height: usize,
}

impl<T: PartialOrd> AVLNode<T> {
    pub fn new(value: T) -> Self {
        AVLNode {
            value,
            left: None,
            right: None,
            height: 1,
        }
    }

    pub fn update_height(&mut self) {
        self.height = 1 + std::cmp::max(Self::height(&self.left), Self::height(&self.right));
    }

    pub fn height(node: &Option<Box<Self>>) -> usize {
        node.as_ref().map_or(0, |n| n.height)
    }

    pub fn balance_factor(&self) -> i32 {
        let left_height = Self::height(&self.left) as i32;
        let right_height = Self::height(&self.right) as i32;
        left_height - right_height
    }

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

    pub fn ll_rotation(mut self: Box<Self>) -> Box<Self> {
        let mut new_root = self.left.take().unwrap();
        self.left = new_root.right.take();
        self.update_height();
        new_root.right = Some(self);
        new_root.update_height();
        new_root
    }

    pub fn rr_rotation(mut self: Box<Self>) -> Box<Self> {
        let mut new_root = self.right.take().unwrap();
        self.right = new_root.left.take();
        self.update_height();
        new_root.left = Some(self);
        new_root.update_height();
        new_root
    }

    pub fn rl_rotation(mut self: Box<Self>) -> Box<Self> {
        let right = self.right.take().unwrap();
        self.right = Some(right.ll_rotation());
        self.rr_rotation()
    }

    pub fn lr_rotation(mut self: Box<Self>) -> Box<Self> {
        let left = self.left.take().unwrap();
        self.left = Some(left.rr_rotation());
        self.ll_rotation()
    }
}
