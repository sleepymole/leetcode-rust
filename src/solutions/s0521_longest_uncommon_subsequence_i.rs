#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        if a != b {
            a.len().max(b.len()) as i32
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_lu_slength() {
        assert_eq!(
            Solution::find_lu_slength("aba".to_owned(), "cdc".to_owned()),
            3
        );
        assert_eq!(
            Solution::find_lu_slength("aaa".to_owned(), "bbb".to_owned()),
            3
        );
        assert_eq!(
            Solution::find_lu_slength("aaa".to_owned(), "aaa".to_owned()),
            -1
        );
    }
}
