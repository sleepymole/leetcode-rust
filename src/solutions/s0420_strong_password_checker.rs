#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn strong_password_checker(password: String) -> i32 {
        let chars: Vec<char> = password.chars().collect();
        let mut has_lower = false;
        let mut has_upper = false;
        let mut has_digit = false;
        let mut missing = 3;
        for &c in &chars {
            if !has_lower && c.is_ascii_lowercase() {
                missing -= 1;
                has_lower = true;
            } else if !has_upper && c.is_ascii_uppercase() {
                missing -= 1;
                has_upper = true;
            } else if !has_digit && c.is_ascii_digit() {
                missing -= 1;
                has_digit = true;
            }
        }
        let mut need = 6 - chars.len().min(6);
        let mut remove = chars.len().max(20) - 20;
        missing -= missing.min(need);
        let mut steps = 0;
        let mut i = 0;
        let mut repeats = Vec::new();
        while i < chars.len() {
            let mut j = i + 1;
            while j < chars.len() && chars[j] == chars[i] {
                j += 1;
            }
            if j - i >= 3 {
                let mut r = j - i;
                while r >= 3 && need > 0 {
                    r -= 2;
                    need -= 1;
                    steps += 1;
                }
                while r >= 3 && missing > 0 {
                    r -= 3;
                    missing -= 1;
                    steps += 1;
                }
                if r >= 3 {
                    if remove > 0 {
                        repeats.push(r);
                    } else {
                        steps += r / 3;
                    }
                }
            }
            i = j;
        }
        if remove > 0 {
            if repeats.is_empty() {
                steps += missing + remove;
            } else {
                assert!(missing == 0);
                while remove > 0 {
                    let mut target = repeats.len();
                    for i in 0..repeats.len() {
                        if repeats[i] < 3 {
                            continue;
                        }
                        if target == repeats.len() {
                            target = i;
                            continue;
                        }
                        if repeats[i] % 3 < repeats[target] % 3 {
                            target = i;
                        }
                    }
                    if target == repeats.len() {
                        break;
                    }
                    repeats[target] -= 1;
                    remove -= 1;
                    steps += 1;
                }
                for i in 0..repeats.len() {
                    steps += repeats[i] / 3;
                }
                steps += remove;
            }
        } else {
            steps += missing + need;
        }
        steps as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strong_password_checker() {
        assert_eq!(Solution::strong_password_checker("a".to_owned()), 5);
        assert_eq!(Solution::strong_password_checker("aA1".to_owned()), 3);
        assert_eq!(Solution::strong_password_checker("1337C0d3".to_owned()), 0);
        assert_eq!(Solution::strong_password_checker("aaa111".to_owned()), 2);
        assert_eq!(
            Solution::strong_password_checker("1111111111".to_owned()),
            3
        );
        assert_eq!(
            Solution::strong_password_checker("bbaaaaaaaaaaaaaaacccccc".to_owned()),
            8
        );
        assert_eq!(
            Solution::strong_password_checker("aaaabbbbccccddeeddeeddeedd".to_owned()),
            8
        );
    }
}
