#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    fn dfs(chars: &[char]) -> Vec<i32> {
        if chars
            .iter()
            .skip(1)
            .filter(|&&c| c == '+' || c == '-' || c == '*')
            .count()
            == 0
        {
            return vec![chars.iter().collect::<String>().parse().unwrap()];
        }
        let mut ans = vec![];
        let mut i = 1;
        while i < chars.len() {
            if chars[i] == '+' || chars[i] == '-' || chars[i] == '*' {
                let left = Solution::dfs(&chars[0..i]);
                let right = Solution::dfs(&chars[i + 1..]);
                for x in left {
                    for y in &right {
                        match chars[i] {
                            '+' => ans.push(x + y),
                            '-' => ans.push(x - y),
                            '*' => ans.push(x * y),
                            _ => unreachable!(),
                        }
                    }
                }
                i += 1;
            }
            i += 1;
        }
        ans
    }

    pub fn diff_ways_to_compute(input: String) -> Vec<i32> {
        let chars: Vec<char> = input.chars().collect();
        let mut ans = Solution::dfs(&chars);
        ans.sort_unstable();
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff_ways_to_compute() {
        assert_eq!(
            Solution::diff_ways_to_compute("2-1-1".to_owned()),
            vec![0, 2]
        );
        assert_eq!(
            Solution::diff_ways_to_compute("2*3-4*5".to_owned()),
            vec![-34, -14, -10, -10, 10]
        );
    }
}
