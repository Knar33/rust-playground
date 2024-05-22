fn main() { 
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  let l3: &mut Option<Box<ListNode>> = &mut Some(Box::new(ListNode::new(0)));
  let l1 = &mut l1.clone();
  let l2 = &mut l2.clone();
  add_two_numbers_recurse(l1, l2, l3, false)
}

pub fn add_two_numbers_recurse(&mut l1: &mut Option<Box<ListNode>>, &mut l2: &mut Option<Box<ListNode>>, &mut l3: &mut Option<Box<ListNode>>, carry: bool) -> Option<Box<ListNode>> {
  let mut l1_num = 0;
  let mut l2_num  = 0;

  if let Some(ref node1) = l1 {
    l1_num = node1.val;
    l1 = node1.next;
    if let Some(ref node2) = l2 {
      l2_num = node2.val;
      l2 = node2.next;
    }
  } else if let Some(node) = l2 {
    l2_num = node.val;
  } else {
    return None
  }

  let mut sum = l1_num + l2_num;
  if (carry) {
    sum += 1;
  }

  if let Some(node) = l3 {
    if sum >= 10 {
      node.val = sum - 10;
      l3 = node.next;
      return add_two_numbers_recurse(&mut l1, &mut l2, &mut l3, true)
    } else {
      node.val = sum;
      l3 = node.next;
      return add_two_numbers_recurse(&mut l1, &mut l2, &mut l3, false)
    }
  } else {
    return None
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
        assert_eq!(search_insert(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 4), 4);
        assert_eq!(search_insert(vec![0, 1, 2, 3, 5, 6, 7, 8, 9, 10], 4), 4);
    }
}