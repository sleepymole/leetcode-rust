#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut stk = Vec::new();
        let names: Vec<&str> = path.split('/').collect();
        for name in names {
            if name == "" || name == "." {
                continue;
            }
            if name == ".." {
                if !stk.is_empty() {
                    stk.pop();
                }
            } else {
                stk.push(name);
            }
        }
        let mut ans = String::new();
        for s in stk.into_iter() {
            ans.push('/');
            ans.push_str(s);
        }
        if ans.len() == 0 {
            ans.push('/');
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simplify_path() {
        assert_eq!(
            Solution::simplify_path("/home/".to_owned()),
            "/home".to_owned()
        );
        assert_eq!(Solution::simplify_path("/../".to_owned()), "/".to_owned());
        assert_eq!(
            Solution::simplify_path("/home//foo/".to_owned()),
            "/home/foo".to_owned()
        );
        assert_eq!(
            Solution::simplify_path("/a/./b/../../c/".to_owned()),
            "/c".to_owned()
        );
    }
}
