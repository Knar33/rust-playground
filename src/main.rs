use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let tree = Some(Rc::new(RefCell::new(TreeNode::new(222))));
    println!("{:?}", invert_tree(tree));
}

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = &root {
        let mut node_borrow = node.borrow_mut();
        let left = node_borrow.left.take();
        let right = node_borrow.right.take();
        node_borrow.left = invert_tree(right);
        node_borrow.right = invert_tree(left);
    }
    root
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}