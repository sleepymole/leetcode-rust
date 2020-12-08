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
                let t = nums[j];
                nums[j] = nums[i - 1];
                nums[i - 1] = t;
                break;
            }
        }
        let mut j = nums.len() - 1;
        while i < j {
            let t = nums[j];
            nums[j] = nums[i];
            nums[i] = t;
            i += 1;
            j -= 1;
        }
        return true;
    }

    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut p = Vec::new();
        for i in 0..n {
            p.push(i);
        }
        let mut ans = Vec::new();
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
                let mut res = Vec::new();
                for i in 0..n {
                    let mut s = String::new();
                    for j in 0..n {
                        if j == p[i as usize] {
                            s.push('Q');
                        } else {
                            s.push('.')
                        }
                    }
                    res.push(s);
                }
                ans.push(res);
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
        assert_eq!(
            Solution::solve_n_queens(4),
            vec![
                vec![
                    ".Q..".to_owned(),
                    "...Q".to_owned(),
                    "Q...".to_owned(),
                    "..Q.".to_owned()
                ],
                vec![
                    "..Q.".to_owned(),
                    "Q...".to_owned(),
                    "...Q".to_owned(),
                    ".Q..".to_owned()
                ]
            ]
        );
    }
}
