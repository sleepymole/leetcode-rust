#![allow(dead_code)]
pub struct Solution;

use crate::util::ListNode;

impl Solution {
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut cur = &mut head;
        while let Some(node) = cur {
            let mut min = (0, node.val);
            let mut step = 0;
            let mut next = &node.next;
            while let Some(node) = next {
                step += 1;
                if node.val < min.1 {
                    min = (step, node.val);
                }
                next = &node.next;
            }
            let mut front = cur.take();
            let mut next = &mut front;
            for _ in 0..min.0 {
                next = &mut next.as_mut().unwrap().next;
            }
            let mut new_front = next.take();
            *next = new_front.as_mut().unwrap().next.take();
            new_front.as_mut().unwrap().next = front;
            *cur = new_front;
            cur = &mut cur.as_mut().unwrap().next;
        }
        head
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort_list() {
        assert_eq!(
            Solution::insertion_sort_list(list!(4, 2, 1, 3)),
            list!(1, 2, 3, 4)
        );
        assert_eq!(
            Solution::insertion_sort_list(list!(-1, 5, 3, 4, 0)),
            list!(-1, 0, 3, 4, 5)
        );
    }
}
