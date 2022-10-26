#![allow(dead_code)]
pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
        let s: Vec<char> = s.chars().collect();
        let (mut l, mut r, mut o) = (0, 0, 0);
        for &c in &s {
            if c == '(' {
                l += 1;
            } else if c == ')' {
                if l > r {
                    r += 1;
                }
            } else {
                o += 1;
            }
        }
        let target = 2 * r + o;
        let mut ans = HashSet::new();
        let n = s.iter().filter(|&&c| c == '(' || c == ')').count();
        for i in 0..(1 << n) {
            let mut t = String::new();
            let mut j = 0;
            for &c in &s {
                if c == '(' || c == ')' {
                    if i & (1 << j) > 0 {
                        t.push(c);
                    }
                    j += 1;
                } else {
                    t.push(c);
                }
            }
            if t.len() != target {
                continue;
            }
            let (mut l, mut r, mut o) = (0, 0, 0);
            for c in t.chars() {
                if c == '(' {
                    l += 1;
                } else if c == ')' {
                    r += 1;
                } else {
                    o += 1;
                }
                if l < r {
                    break;
                }
            }
            if l == r && l + r + o == target {
                ans.insert(t);
            }
        }
        ans.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sorted(mut ss: Vec<String>) -> Vec<String> {
        ss.sort_unstable();
        ss
    }

    #[test]
    fn test_remove_invalid_parentheses() {
        assert_eq!(
            sorted(Solution::remove_invalid_parentheses("()())()".to_owned())),
            sorted(vec!["(())()".to_owned(), "()()()".to_owned()])
        );
        assert_eq!(
            sorted(Solution::remove_invalid_parentheses("(a)())()".to_owned())),
            sorted(vec!["(a())()".to_owned(), "(a)()()".to_owned()])
        );
        assert_eq!(
            sorted(Solution::remove_invalid_parentheses(")(".to_owned())),
            vec!["".to_owned()]
        );
    }
}
