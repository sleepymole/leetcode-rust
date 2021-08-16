#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    fn is_subsequence(a: &str, b: &str) -> bool {
        let mut b = b.chars();
        for c1 in a.chars() {
            let mut matched = false;
            while let Some(c2) = b.next() {
                if c1 == c2 {
                    matched = true;
                    break;
                }
            }
            if !matched {
                return false;
            }
        }
        true
    }

    pub fn find_lu_slength(strs: Vec<String>) -> i32 {
        let mut res = -1;
        for i in 0..strs.len() {
            let mut ok = true;
            for j in 0..strs.len() {
                if i != j && Solution::is_subsequence(&strs[i], &strs[j]) {
                    ok = false;
                    break;
                }
            }
            if ok {
                res = res.max(strs[i].len() as i32);
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_lu_slength() {
        assert_eq!(
            Solution::find_lu_slength(vec!["aba".to_owned(), "cdc".to_owned(), "eae".to_owned()]),
            3
        );
        assert_eq!(
            Solution::find_lu_slength(vec!["aaa".to_owned(), "aaa".to_owned(), "aa".to_owned()]),
            -1
        );
    }
}
