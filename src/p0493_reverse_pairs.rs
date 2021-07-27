#![allow(dead_code)]
pub struct Solution;

impl Solution {
    fn binary_search(sorted: &Vec<i64>, x: i64) -> i32 {
        let mut l = 0;
        let mut r = sorted.len();
        while l < r {
            let m = (l + r) / 2;
            if sorted[m] >= x {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l as i32 + 1
    }

    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        let mut sorted = Vec::new();
        for &x in &nums {
            sorted.push(x as i64);
            sorted.push(2 * x as i64);
        }
        sorted.sort_unstable();
        let add = |tree: &mut Vec<i32>, x: i32| {
            let mut i = x;
            while i < tree.len() as i32 {
                tree[i as usize] += 1;
                i += i & -i;
            }
        };
        let sum = |tree: &Vec<i32>, x: i32| {
            let mut res = 0;
            let mut i = x;
            while i > 0 {
                res += tree[i as usize];
                i -= i & -i;
            }
            res
        };
        let mut tree = vec![0; nums.len() * 2 + 1];
        let mut pairs = 0;
        for &x in &nums {
            let m = Solution::binary_search(&sorted, 2 * x as i64);
            pairs += sum(&tree, tree.len() as i32 - 1) - sum(&tree, m);
            add(&mut tree, Solution::binary_search(&sorted, x as i64));
        }
        pairs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_pairs() {
        assert_eq!(Solution::reverse_pairs(vec![1, 3, 2, 3, 1]), 2);
        assert_eq!(Solution::reverse_pairs(vec![2, 4, 3, 5, 1]), 3);
    }
}
