#![allow(dead_code)]
pub struct Solution {}

use std::collections::HashSet;

impl Solution {
    fn dfs(n: i32, visited: &mut HashSet<i32>) -> bool {
        if n == 1 {
            return true;
        }
        visited.insert(n);
        let mut sum = 0;
        let mut n = n;
        while n > 0 {
            sum += (n % 10) * (n % 10);
            n /= 10;
        }
        if visited.contains(&sum) {
            return false;
        }
        Solution::dfs(sum, visited)
    }

    pub fn is_happy(n: i32) -> bool {
        let mut visited = HashSet::new();
        Solution::dfs(n, &mut visited)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_happy() {
        assert!(Solution::is_happy(19));
        assert!(!Solution::is_happy(2));
    }
}
