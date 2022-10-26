#![allow(dead_code)]
pub struct Solution;

use std::collections::HashSet;

impl Solution {
    fn search(w: &Vec<char>, matched: usize, s: &HashSet<&str>) -> bool {
        if matched == w.len() {
            return true;
        }
        let mut sw = String::new();
        for i in matched..w.len() {
            if matched == 0 && i + 1 == w.len() {
                break;
            }
            sw.push(w[i]);
            if s.contains(sw.as_str()) && Solution::search(w, matched + sw.len(), s) {
                return true;
            }
        }
        false
    }

    pub fn find_all_concatenated_words_in_a_dict(words: Vec<String>) -> Vec<String> {
        let words: Vec<String> = words.into_iter().filter(|s| !s.is_empty()).collect();
        let mut s: HashSet<&str> = HashSet::new();
        for w in &words {
            s.insert(w);
        }
        let mut res = Vec::new();
        for sw in &words {
            let w = sw.chars().collect();
            if Solution::search(&w, 0, &s) {
                res.push(sw.clone());
            }
        }
        res.sort_unstable();
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_all_concatenated_words_in_a_dict() {
        assert_eq!(
            Solution::find_all_concatenated_words_in_a_dict(vec![
                "cat".to_owned(),
                "cats".to_owned(),
                "catsdogcats".to_owned(),
                "dog".to_owned(),
                "dogcatsdog".to_owned(),
                "hippopotamuses".to_owned(),
                "rat".to_owned(),
                "ratcatdogcat".to_owned()
            ]),
            vec![
                "catsdogcats".to_owned(),
                "dogcatsdog".to_owned(),
                "ratcatdogcat".to_owned()
            ]
        );
        assert_eq!(
            Solution::find_all_concatenated_words_in_a_dict(vec![
                "cat".to_owned(),
                "dog".to_owned(),
                "catdog".to_owned()
            ],),
            vec!["catdog".to_owned()]
        );
    }
}
