#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) -> bool {
        if nums.len() <= 1 {
            return false;
        }
        let mut i = nums.len() - 1;
        while i > 0 && nums[i - 1] >= nums[i] {
            i -= 1;
        }
        if i == 0 {
            return false;
        }
        for j in (i..nums.len()).rev() {
            if nums[j] > nums[i - 1] {
                nums.swap(j, i - 1);
                break;
            }
        }
        let mut j = nums.len() - 1;
        while i < j {
            nums.swap(j, i);
            i += 1;
            j -= 1;
        }
        true
    }

    pub fn total_n_queens(n: i32) -> i32 {
        let mut p = Vec::new();
        for i in 0..n {
            p.push(i);
        }
        let mut ans = 0;
        loop {
            let mut valid = true;
            for i in 0..n {
                for j in i + 1..n {
                    if i - p[i as usize] == j - p[j as usize]
                        || i + p[i as usize] == j + p[j as usize]
                    {
                        valid = false;
                        break;
                    }
                }
                if !valid {
                    break;
                }
            }
            if valid {
                ans += 1;
            }
            if !Solution::next_permutation(&mut p) {
                break;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve_n_queens() {
        assert_eq!(Solution::total_n_queens(4), 2);
        assert_eq!(Solution::total_n_queens(1), 1);
    }
}
