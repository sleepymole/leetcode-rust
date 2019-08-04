#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut chars = vec!['#'];
        for c in s.chars() {
            chars.push(c);
            chars.push('#');
        }
        let mut radius = Vec::with_capacity(chars.len());
        radius.resize(chars.len(), 0);
        assert_eq!(radius.len(), chars.len());
        let (mut center, mut right, mut max_center, mut max_radius) = (0, 0, 0, 0);
        for i in 0..chars.len() {
            if i < right {
                radius[i] = radius[2 * center - i];
                if i + radius[i] > right {
                    radius[i] = right - i;
                }
            }
            while i >= radius[i] + 1
                && i + radius[i] + 1 < chars.len()
                && chars[i - radius[i] - 1] == chars[i + radius[i] + 1]
            {
                radius[i] += 1;
            }
            if i + radius[i] > right {
                right = i + radius[i];
                center = i;
            }
            if radius[i] > max_radius {
                max_center = i;
                max_radius = radius[i];
            }
        }
        s[max_center / 2 - max_radius / 2..(max_center + 1) / 2 + max_radius / 2].to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        let p1 = Solution::longest_palindrome(String::from("babad"));
        assert!(valid_palindrome(&p1));
        assert_eq!(p1.len(), 3);

        let p2 = Solution::longest_palindrome(String::from("cbbd"));
        assert!(valid_palindrome(&p2));
        assert_eq!(p2.len(), 2);
    }

    fn valid_palindrome(s: &String) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let (mut i, mut j) = (0, 0);
        while i < j {
            if chars[i] != chars[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}
