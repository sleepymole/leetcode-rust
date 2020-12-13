#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if t.is_empty() {
            return "".to_owned();
        }
        let mut epexct: Vec<i32> = Vec::new();
        epexct.resize(128, 0);
        for c in t.chars() {
            epexct[c as usize] += 1;
        }
        let n = t.len();
        let s = s.into_bytes();
        let mut actual: Vec<i32> = Vec::new();
        actual.resize(128, 0);
        let (mut i, mut j, mut matched) = (0, 0, 0);
        let mut ans: Option<String> = None;
        while j < s.len() {
            actual[s[j] as usize] += 1;
            if actual[s[j] as usize] <= epexct[s[j] as usize] {
                matched += 1;
            }
            if matched == n && (ans.is_none() || ans.as_ref().unwrap().len() > j - i + 1) {
                let mut ss = String::new();
                for k in i..=j {
                    ss.push(s[k] as char);
                }
                ans = Some(ss);
            }
            while matched == n || i <= j && actual[s[i] as usize] > epexct[s[i] as usize] {
                if actual[s[i] as usize] <= epexct[s[i] as usize] {
                    matched -= 1;
                }
                actual[s[i] as usize] -= 1;
                i += 1;
            }
            j += 1;
        }
        ans.unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_window() {
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".to_owned(), "ABC".to_owned()),
            "BANC".to_owned()
        );
        assert_eq!(
            Solution::min_window("a".to_owned(), "a".to_owned()),
            "a".to_owned()
        );
        assert_eq!(
            Solution::min_window("aa".to_owned(), "aa".to_owned()),
            "aa".to_owned()
        );
        assert_eq!(
            Solution::min_window("bba".to_owned(), "ab".to_owned()),
            "ba".to_owned()
        );
        assert_eq!(
            Solution::min_window("aaaaaaaaaaaabbbbbcdd".to_owned(), "abcdd".to_owned()),
            "abbbbbcdd".to_owned()
        );
    }
}
