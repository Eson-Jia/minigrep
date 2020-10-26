use std::rc::Rc;
use std::cell::RefCell;

// Definition for singly-linked list.
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


// Definition for a binary tree node.
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

struct Solution();


impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode::new(0));
        let mut tail = &mut head;
        let mut r_l1 = l1.as_ref();
        let mut r_l2 = l2.as_ref();
        let mut value =0;
        loop {
            if r_l1.is_none() && r_l2.is_none() && value ==0 {
                break;
            }
             match r_l1 {
                Some(t)=>{
                    value+=t.val;
                     r_l1 = t.next.as_ref();
                },
                None=>{},
            };
            match r_l2{
                Some(t)=>{
                    value +=t.val;
                    r_l2 = t.next.as_ref();
                },
                None=> {},
            };
            tail.next =Some(Box::new(ListNode::new((value)%10)));
            value/=10;
            tail = tail.next.as_mut().unwrap();
        }
        head.next
    }
}

fn constructor_list(data:&[i32])->Option<Box<ListNode>>{
    let mut head = Box::new(ListNode::new(0));
    let mut tail = &mut head;
    for ele in data.iter(){
        tail.next = Option::from(Box::new(ListNode::new(*ele)));
        tail = tail.next.as_mut().unwrap();
    }
    head.next
}

#[cfg(test)]
mod tests{
    use crate::{Solution, ListNode, constructor_list};
    #[test]
    fn test_add_two_numbers(){
        let result = Solution::add_two_numbers(constructor_list(&[2,4,3]), constructor_list(&[5,6,4]));
        let mut result = result.as_ref();
        while let Some(ele) = result{
            println!("{}",ele.val);
            result = ele.next.as_ref();
        }
    }
}

fn main() {}
