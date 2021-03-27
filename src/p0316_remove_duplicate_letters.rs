#![allow(dead_code)]
pub struct Solution;

use std::char;

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut cnt = vec![0; 26];
        for c in s.chars() {
            cnt[c as usize - 'a' as usize] += 1;
        }
        let mut vis = vec![false; 26];
        let mut ans = vec![];
        for c in s.chars() {
            let x = c as usize - 'a' as usize;
            cnt[x] -= 1;
            if vis[x] {
                continue;
            }
            while let Some(&v) = ans.last() {
                if x < v && cnt[v] > 0 {
                    ans.pop();
                    vis[v] = false;
                } else {
                    break;
                }
            }
            vis[x] = true;
            ans.push(x);
        }
        ans.iter()
            .map(|&x| char::from_u32(x as u32 + 'a' as u32).unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicate_letters() {
        assert_eq!(
            Solution::remove_duplicate_letters("bcabc".to_owned()),
            "abc".to_owned()
        );
        assert_eq!(
            Solution::remove_duplicate_letters("cbacdcbc".to_owned()),
            "acdb".to_owned()
        );
        assert_eq!(
            Solution::remove_duplicate_letters("leetcode".to_owned()),
            "letcod".to_owned()
        );
        assert_eq!(
            Solution::remove_duplicate_letters("bbcaac".to_owned()),
            "bac".to_owned()
        );
    }
}
