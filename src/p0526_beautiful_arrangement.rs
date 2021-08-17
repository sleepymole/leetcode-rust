#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    fn dfs(pos: i32, state: i32) -> i32 {
        if state == 0 {
            return 1;
        }
        let mut perms = 0;
        for i in 0..20 {
            if (1 << i) & state > 0 && ((i + 1) % (pos + 1) == 0 || (pos + 1) % (i + 1) == 0) {
                perms += Solution::dfs(pos + 1, state ^ (1 << i));
            }
        }
        perms
    }

    pub fn count_arrangement(n: i32) -> i32 {
        Solution::dfs(0, (1 << n) - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_arrangement() {
        assert_eq!(Solution::count_arrangement(2), 2);
        assert_eq!(Solution::count_arrangement(1), 1);
    }
}
