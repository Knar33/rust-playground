fn main() { 
  let list1 = None;
  let list2 = Some(Box::new(ListNode{next: None, val: 0}));

  println!("{:?}", merge_two_lists(list1, list2));
}

pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if let Some (listnode1) = list1.clone() {
      if let Some(listnode2) = list2.clone() {
        if listnode1.val < listnode2.val {
            Some(Box::new(ListNode {
              next: merge_two_lists(listnode1.next, list2),
              val: listnode1.val
            }))
        } else {
          Some(Box::new(ListNode {  next: merge_two_lists(list1, listnode2.next), val: listnode2.val }))
        }
      } else {
          Some(Box::new(ListNode { next: merge_two_lists(listnode1.next, None), val: listnode1.val }))
      }
    } else if let Some(listnode2) = list2.clone() {
        Some(Box::new(ListNode { next: merge_two_lists(listnode2.next, None), val: listnode2.val }))
    } else {
      None
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