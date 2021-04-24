#![allow(dead_code)]
pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_rectangle_cover(rectangles: Vec<Vec<i32>>) -> bool {
        let mut m = HashMap::new();
        for r in rectangles {
            let flag = m.entry((r[2], r[1])).or_insert(0);
            if (*flag & 1) > 0 {
                return false;
            }
            *flag |= 1;
            let flag = m.entry((r[0], r[1])).or_insert(0);
            if (*flag & 2) > 0 {
                return false;
            }
            *flag |= 2;
            let flag = m.entry((r[2], r[3])).or_insert(0);
            if (*flag & 4) > 0 {
                return false;
            }
            *flag |= 4;
            let flag = m.entry((r[0], r[3])).or_insert(0);
            if (*flag & 8) > 0 {
                return false;
            }
            *flag |= 8;
        }
        let mut vertices = 0;
        for &flag in m.values() {
            if flag == 6 || flag == 7 || flag == 9 || flag == 11 || flag == 13 || flag == 14 {
                return false;
            }
            if flag == 1 || flag == 2 || flag == 4 || flag == 8 {
                vertices += 1;
            }
        }
        vertices == 4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_rectangle_cover() {
        assert_eq!(
            Solution::is_rectangle_cover(vec![
                vec![1, 1, 3, 3],
                vec![3, 1, 4, 2],
                vec![3, 2, 4, 4],
                vec![1, 3, 2, 4],
                vec![2, 3, 3, 4]
            ]),
            true
        );
        assert_eq!(
            Solution::is_rectangle_cover(vec![
                vec![1, 1, 2, 3],
                vec![1, 3, 2, 4],
                vec![3, 1, 4, 2],
                vec![3, 2, 4, 4]
            ]),
            false
        );
        assert_eq!(
            Solution::is_rectangle_cover(vec![
                vec![1, 1, 3, 3],
                vec![3, 1, 4, 2],
                vec![1, 3, 2, 4],
                vec![3, 2, 4, 4]
            ]),
            false
        );
        assert_eq!(
            Solution::is_rectangle_cover(vec![
                vec![1, 1, 3, 3],
                vec![3, 1, 4, 2],
                vec![1, 3, 2, 4],
                vec![2, 2, 4, 4]
            ]),
            false
        );
    }
}
