#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stk = Vec::new();
        for token in tokens.into_iter() {
            if let Ok(x) = token.parse::<i32>() {
                stk.push(x);
            } else {
                let right = stk.pop().unwrap();
                let left = stk.pop().unwrap();
                match token.as_str() {
                    "+" => stk.push(left + right),
                    "-" => stk.push(left - right),
                    "*" => stk.push(left * right),
                    "/" => stk.push(left / right),
                    _ => unreachable!(),
                };
            }
        }
        stk.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_rpn() {
        assert_eq!(
            Solution::eval_rpn(vec![
                "2".to_owned(),
                "1".to_owned(),
                "+".to_owned(),
                "3".to_owned(),
                "*".to_owned()
            ]),
            9
        );
        assert_eq!(
            Solution::eval_rpn(vec![
                "4".to_owned(),
                "13".to_owned(),
                "5".to_owned(),
                "/".to_owned(),
                "+".to_owned()
            ]),
            6
        );
        assert_eq!(
            Solution::eval_rpn(vec![
                "10".to_owned(),
                "6".to_owned(),
                "9".to_owned(),
                "3".to_owned(),
                "+".to_owned(),
                "-11".to_owned(),
                "*".to_owned(),
                "/".to_owned(),
                "*".to_owned(),
                "17".to_owned(),
                "+".to_owned(),
                "5".to_owned(),
                "+".to_owned()
            ]),
            22
        );
    }
}
