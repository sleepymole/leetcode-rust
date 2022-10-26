#![allow(dead_code)]
pub struct Solution;

impl Solution {
    fn dfs(n: i32, x: i32, nums: &mut Vec<i32>, vis: &mut Vec<bool>) -> bool {
        nums.push(x);
        vis[x as usize] = true;
        if nums.len() == 1 << n {
            return true;
        }
        for i in 0..n {
            let nx = x ^ (1 << i);
            if !vis[nx as usize] && Solution::dfs(n, nx, nums, vis) {
                return true;
            }
        }
        vis[x as usize] = false;
        nums.pop();
        false
    }

    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut vis = Vec::new();
        vis.resize(1 << n, false);
        let mut nums = Vec::new();
        Solution::dfs(n, 0, &mut nums, &mut vis);
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gray_code() {
        assert_eq!(Solution::gray_code(2), vec![0, 1, 3, 2]);
        assert_eq!(Solution::gray_code(0), vec![0]);
    }
}
