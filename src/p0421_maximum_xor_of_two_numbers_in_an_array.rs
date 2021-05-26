#![allow(dead_code)]
pub struct Solution;

#[derive(Debug, Clone)]
struct Node {
    children: [usize; 2],
}

impl Node {
    fn new() -> Self {
        Node { children: [0; 2] }
    }
}

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut nodes = vec![Node::new(); nums.len() * 31 + 1];
        let mut index = 1;
        let mut res = 0;
        let mut first = true;

        let is_set = |x: i32, i: i32| {
            if x & (1 << i) == 0 {
                0
            } else {
                1
            }
        };

        for x in nums {
            let mut root = 0;
            if !first {
                let mut xor = 0;
                for i in (0..31).rev() {
                    let set = is_set(x, i);
                    if nodes[root].children[set ^ 1] > 0 {
                        xor |= 1 << i;
                        root = nodes[root].children[set ^ 1];
                    } else {
                        root = nodes[root].children[set];
                    }
                }
                res = res.max(xor);
            } else {
                first = false;
            }
            root = 0;
            for i in (0..31).rev() {
                let set = is_set(x, i);
                if nodes[root].children[set] == 0 {
                    nodes[root].children[set] = index;
                    index += 1;
                }
                root = nodes[root].children[set];
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_maximum_xor() {
        assert_eq!(Solution::find_maximum_xor(vec![3, 10, 5, 25, 2, 8]), 28);
        assert_eq!(Solution::find_maximum_xor(vec![0]), 0);
        assert_eq!(Solution::find_maximum_xor(vec![2, 4]), 6);
    }
}
