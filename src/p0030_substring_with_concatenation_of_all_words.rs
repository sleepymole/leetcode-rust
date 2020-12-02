#![allow(dead_code)]
pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if words.is_empty() {
            return vec![];
        }
        let mut expect: HashMap<String, i32> = HashMap::new();
        for w in words.iter() {
            expect.insert(w.clone(), expect.get(w).map_or(0, |x| *x) + 1);
        }
        let wlen = words[0].len();
        let mut ans: Vec<i32> = Vec::new();
        for i in 0..wlen {
            let mut actual: HashMap<String, i32> = HashMap::new();
            let (mut i, mut j, mut matched) = (i, i, 0);
            while j + wlen <= s.len() {
                let right: String = s.chars().skip(j).take(wlen).collect();
                if !expect.contains_key(&right) {
                    actual.clear();
                    j += wlen;
                    i = j;
                    matched = 0;
                    continue;
                }
                actual.insert(right.clone(), actual.get(&right).map_or(0, |x| *x) + 1);
                matched += 1;
                if matched == words.len() && actual[&right] == expect[&right] {
                    ans.push(i as i32);
                }
                while matched == words.len()
                    || actual.get(&right).map_or(0, |x| *x) > expect.get(&right).map_or(0, |x| *x)
                {
                    let left: String = s.chars().skip(i).take(wlen).collect();
                    let cnt = actual.get(&left).map_or(0, |x| *x);
                    if cnt == 1 {
                        actual.remove(&left);
                    } else {
                        actual.insert(left, cnt - 1);
                    }
                    matched -= 1;
                    i += wlen;
                }
                j += wlen;
            }
        }
        ans.sort();
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_substring() {
        assert_eq!(
            Solution::find_substring(
                "barfoothefoobarman".to_owned(),
                vec!["foo".to_owned(), "bar".to_owned()]
            ),
            vec![0, 9]
        );
        assert_eq!(
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_owned(),
                vec![
                    "word".to_owned(),
                    "good".to_owned(),
                    "best".to_owned(),
                    "word".to_owned()
                ]
            ),
            vec![]
        );
        assert_eq!(
            Solution::find_substring(
                "barfoofoobarthefoobarman".to_owned(),
                vec!["bar".to_owned(), "foo".to_owned(), "the".to_owned()]
            ),
            vec![6, 9, 12]
        );
        assert_eq!(
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_owned(),
                vec![
                    "word".to_owned(),
                    "good".to_owned(),
                    "best".to_owned(),
                    "good".to_owned()
                ]
            ),
            vec![8]
        );
        assert_eq!(
            Solution::find_substring("a".to_owned(), vec!["a".to_owned()]),
            vec![0]
        )
    }
}
