#![allow(dead_code)]
pub struct Solution;

impl Solution {
    fn format(line: &Vec<String>, spaces: usize) -> String {
        let mut s = String::new();
        s.push_str(line[0].as_str());
        let mut remain = spaces;
        for i in 1..line.len() {
            let mut n = spaces / (line.len() - 1);
            if spaces % (line.len() - 1) >= i {
                n += 1;
            }
            for _ in 0..n {
                s.push(' ');
            }
            remain -= n;
            s.push_str(line[i].as_str());
        }
        for _ in 0..remain {
            s.push(' ');
        }
        s
    }

    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let mut ans = Vec::new();
        let mut line = Vec::new();
        let mut sum = 0;
        for w in words.into_iter() {
            if sum + line.len() + w.len() > max_width as usize {
                ans.push(Solution::format(&line, max_width as usize - sum));
                line.clear();
                sum = 0;
            }
            sum += w.len();
            line.push(w);
        }
        if sum > 0 {
            let mut s = String::new();
            s.push_str(line[0].as_str());
            for i in 1..line.len() {
                s.push(' ');
                s.push_str(line[i].as_str());
            }
            while s.len() < max_width as usize {
                s.push(' ');
            }
            ans.push(s);
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_justify() {
        assert_eq!(
            Solution::full_justify(
                vec![
                    "This".to_owned(),
                    "is".to_owned(),
                    "an".to_owned(),
                    "example".to_owned(),
                    "of".to_owned(),
                    "text".to_owned(),
                    "justification.".to_owned()
                ],
                16
            ),
            vec![
                "This    is    an".to_owned(),
                "example  of text".to_owned(),
                "justification.  ".to_owned()
            ]
        );
        assert_eq!(
            Solution::full_justify(
                vec![
                    "What".to_owned(),
                    "must".to_owned(),
                    "be".to_owned(),
                    "acknowledgment".to_owned(),
                    "shall".to_owned(),
                    "be".to_owned()
                ],
                16
            ),
            vec![
                "What   must   be".to_owned(),
                "acknowledgment  ".to_owned(),
                "shall be        ".to_owned()
            ]
        );
        assert_eq!(
            Solution::full_justify(
                vec![
                    "Science".to_owned(),
                    "is".to_owned(),
                    "what".to_owned(),
                    "we".to_owned(),
                    "understand".to_owned(),
                    "well".to_owned(),
                    "enough".to_owned(),
                    "to".to_owned(),
                    "explain".to_owned(),
                    "to".to_owned(),
                    "a".to_owned(),
                    "computer.".to_owned(),
                    "Art".to_owned(),
                    "is".to_owned(),
                    "everything".to_owned(),
                    "else".to_owned(),
                    "we".to_owned(),
                    "do".to_owned()
                ],
                20
            ),
            [
                "Science  is  what we".to_owned(),
                "understand      well".to_owned(),
                "enough to explain to".to_owned(),
                "a  computer.  Art is".to_owned(),
                "everything  else  we".to_owned(),
                "do                  ".to_owned()
            ]
        );
    }
}
