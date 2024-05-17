use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let tree = Some(Rc::new(RefCell::new(TreeNode::new(222))));
    println!("{:?}", invert_tree(tree));
}

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(value) => {
            let root_value = value.borrow();
            let mut new_root = TreeNode::new(root_value.val);
            new_root.left = invert_tree(root_value.right.clone());
            new_root.right = invert_tree(root_value.left.clone());
            Some(Rc::new(RefCell::new(new_root)))
        }
        None => None
    }
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