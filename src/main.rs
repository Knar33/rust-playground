fn main() { 
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  let l3 = Some(Box::new(ListNode::new(0)));
  let a = add_two_numbers_recurse(&l1, &l2, l3, false);
  let b = 0;
  return a
}

pub fn add_two_numbers_recurse(l1: &Option<Box<ListNode>>, l2: &Option<Box<ListNode>>, mut l3: Option<Box<ListNode>>, carry: bool) -> Option<Box<ListNode>> {
  if let Some(node1) = l1 {
    let l1_num = node1.val;
    if let Some(node2) = l2 {
      //both nodes exist
      let l2_num = node2.val;
      let mut sum = l1_num + l2_num + (if carry {1} else {0});

      let mut next_carry = false;
      if sum > 9 {
        next_carry = true;
        sum = sum - 10;
      }
      
      let l3_mut = l3.as_mut().unwrap();
      l3_mut.val = sum;
      l3_mut.next = add_two_numbers_recurse(&node1.next, &node2.next, Some(Box::new(ListNode::new(0))), next_carry);
      return l3;
    } else {
      //only one node exists
      let l3_mut = l3.as_mut().unwrap();
      l3_mut.val = node1.val + if carry {1} else {0};
      let mut next_carry = false;
      if l3_mut.val > 9 {
        next_carry = true;
        l3_mut.val = l3_mut.val - 10;
      }
      l3_mut.next = add_two_numbers_recurse(&node1.next, &None, Some(Box::new(ListNode::new(0))), next_carry);
      return l3;
    }
  } else if let Some(node) = l2 {
    //only one node exists
    let l3_mut = l3.as_mut().unwrap();
    l3_mut.val = node.val + if carry {1} else {0};
    let mut next_carry = false;
    if l3_mut.val > 9 {
        next_carry = true;
        l3_mut.val = l3_mut.val - 10;
    }
    l3_mut.next = add_two_numbers_recurse(&None, &node.next, Some(Box::new(ListNode::new(0))), next_carry);
    return l3;
  } else {
    //both empty nodes
    if carry {
        let l3_mut = l3.as_mut().unwrap();
        l3_mut.val = if carry {1} else {0};
        l3_mut.next = None;
        return l3;
    } else {
      None
    }
  }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asserts() {
      let a = add_two_numbers(
        Some(Box::new(ListNode{val: 9, next: Some(Box::new(ListNode{val: 9, next: Some(Box::new(ListNode{val: 9, next: Some(Box::new(ListNode{val: 9, next: Some(Box::new(ListNode{val: 9, next: Some(Box::new(ListNode{val: 9, next: Some(Box::new(ListNode{val: 9, next: None}))}))}))}))}))}))})),
        Some(Box::new(ListNode{val: 9, next: Some(Box::new(ListNode{val: 9, next: Some(Box::new(ListNode{val: 9, next: Some(Box::new(ListNode{val: 9, next: None}))}))}))}))
      );
      assert_eq!(a.unwrap().val, 7);
    }
}