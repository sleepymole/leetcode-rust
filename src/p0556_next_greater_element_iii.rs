#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut s: Vec<char> = n.to_string().chars().collect();
        let mut i = s.len() - 1;
        while i > 0 && s[i - 1] >= s[i] {
            i -= 1;
        }
        if i == 0 {
            return -1;
        }
        let mut j = i;
        while j + 1 < s.len() && s[j + 1] > s[i - 1] {
            j += 1;
        }
        s.swap(i - 1, j);
        j = s.len() - 1;
        while i < j {
            s.swap(i, j);
            i += 1;
            j -= 1;
        }
        let res = s
            .into_iter()
            .map(|c| c as i64 - '0' as i64)
            .fold(0, |a, b| a * 10 + b);
        if res > (i32::MAX) as i64 {
            return -1;
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_greater_element() {
        assert_eq!(Solution::next_greater_element(12), 21);
        assert_eq!(Solution::next_greater_element(21), -1);
    }
}
