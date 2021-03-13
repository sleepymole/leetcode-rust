#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let mut chars = vec!['#'];
        for c in s.chars().rev() {
            chars.push(c);
            chars.push('#');
        }
        let mut radius = vec![0; chars.len()];
        let (mut center, mut right) = (0, 0);
        for i in 0..chars.len() {
            if i < right {
                radius[i] = radius[2 * center - i];
                if i + radius[i] > right {
                    radius[i] = right - i;
                }
            }
            while i > radius[i]
                && i + radius[i] + 1 < chars.len()
                && chars[i - radius[i] - 1] == chars[i + radius[i] + 1]
            {
                radius[i] += 1;
            }
            if i + radius[i] > right {
                right = i + radius[i];
                center = i;
            }
        }
        for i in (0..center - radius[center]).rev() {
            chars.push(chars[i]);
        }
        chars.iter().filter(|&&c| c != '#').collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_palindrome() {
        assert_eq!(
            Solution::shortest_palindrome("aacecaaa".to_owned()),
            "aaacecaaa".to_owned()
        );
        assert_eq!(
            Solution::shortest_palindrome("abcd".to_owned()),
            "dcbabcd".to_owned()
        );
    }
}
