#![allow(dead_code)]
pub struct Solution;

impl Solution {
    fn search_circular(nums: &Vec<i32>, flag: i32) -> bool {
        let mut vis = vec![-1; nums.len()];
        let find_next = |i: usize| {
            ((i as i32 + nums.len() as i32 + nums[i] % nums.len() as i32) % nums.len() as i32)
                as usize
        };
        for i in 0..nums.len() {
            if vis[i] >= 0 {
                continue;
            }
            vis[i] = i as i32;
            if nums[i] * flag < 0 {
                continue;
            }
            let mut k = 1;
            let mut cur = i;
            let mut next = find_next(i);
            while next != i && nums[next] * flag > 0 && vis[next] < 0 {
                cur = next;
                vis[cur] = i as i32;
                next = find_next(next);
                k += 1;
            }
            if k > 1 && next != cur && vis[next] == i as i32 {
                return true;
            }
        }
        false
    }

    pub fn circular_array_loop(nums: Vec<i32>) -> bool {
        Solution::search_circular(&nums, 1) || Solution::search_circular(&nums, -1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular_array_loop() {
        assert!(Solution::circular_array_loop(vec![2, -1, 1, 2, 2]));
        assert!(!Solution::circular_array_loop(vec![-1, 2]));
        assert!(!Solution::circular_array_loop(vec![-2, 1, -1, -2, -2]));
        assert!(!Solution::circular_array_loop(vec![-1, -2, -3, -4, -5]));
    }
}
