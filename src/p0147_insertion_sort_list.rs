#![allow(dead_code)]
pub struct Solution;

use crate::util::ListNode;

impl Solution {
    pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut tail = &mut head;
        while tail.is_some() {
            let mut x = tail.take();
            let (mut i, mut v) = (0, x.as_ref().unwrap().val);
            let mut p = &x.as_ref().unwrap().next;
            let mut step = 0;
            while let Some(node) = p {
                step += 1;
                if node.val < v {
                    i = step;
                    v = node.val;
                }
                p = &node.next;
            }
            if i == 0 {
                *tail = x;
                tail = &mut tail.as_mut().unwrap().next;
                continue;
            }
            let mut p = &mut x;
            for _ in 0..i - 1 {
                p = &mut p.as_mut().unwrap().next;
            }
            let mut target = p.as_mut().unwrap().next.take();
            p.as_mut().unwrap().next = target.as_mut().unwrap().next.take();
            target.as_mut().unwrap().next = x.take();
            *tail = target;
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
