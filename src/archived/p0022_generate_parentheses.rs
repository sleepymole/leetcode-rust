#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let n = n * 2;
        let mut s = String::new();
        for _ in 0..n / 2 {
            s.push('(');
        }
        for _ in 0..n / 2 {
            s.push(')');
        }
        let mut ss: Vec<String> = Vec::new();
        ss.push(s);
        loop {
            let bs = ss.last().unwrap().as_bytes();
            let (mut l, mut r) = (0, 0);
            let mut p: Option<usize> = None;
            for i in 0..bs.len() {
                if bs[i] == b'(' {
                    if l > r {
                        p = Some(i);
                    }
                    l += 1;
                } else {
                    r += 1;
                }
            }
            if p.is_none() {
                break;
            }
            let p = p.unwrap();
            let mut s = String::new();
            l = 0;
            r = 0;
            for i in 0..p {
                let c = bs[i] as char;
                if c == '(' {
                    l += 1;
                } else {
                    r += 1;
                }
                s.push(bs[i] as char);
            }
            s.push(')');
            r += 1;
            for _ in l..n / 2 {
                s.push('(');
            }
            for _ in r..n / 2 {
                s.push(')');
            }
            ss.push(s);
        }
        ss
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sort(mut v: Vec<String>) -> Vec<String> {
        v.sort();
        v
    }

    #[test]
    fn test_generate_parenthesis() {
        assert_eq!(
            sort(Solution::generate_parenthesis(3)),
            sort(vec![
                "((()))".to_owned(),
                "(()())".to_owned(),
                "(())()".to_owned(),
                "()(())".to_owned(),
                "()()()".to_owned()
            ])
        );
        assert_eq!(
            sort(Solution::generate_parenthesis(1)),
            sort(vec!["()".to_owned()])
        );
    }
}
