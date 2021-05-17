#![allow(dead_code)]
pub struct Solution;

#[derive(Default, Debug)]
struct Trie {
    terminal: Option<usize>,
    children: [Option<Box<Trie>>; 26],
    prefixed: Vec<usize>,
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, index: usize, word: &String) {
        let mut root = self;
        root.prefixed.push(index);
        for i in word.chars().map(|c| c as usize - 'a' as usize) {
            root = root.children[i].get_or_insert(Box::new(Trie::new()));
            root.prefixed.push(index);
        }
        root.terminal = Some(index);
    }

    fn search(&self, word: &String) -> (Vec<usize>, Vec<usize>) {
        let mut root = self;
        let word: Vec<usize> = word
            .chars()
            .rev()
            .map(|c| c as usize - 'a' as usize)
            .collect();
        let mut matched = Vec::new();
        let mut prefixed = Vec::new();
        if let Some(index) = root.terminal {
            if Trie::is_palindrome(&word) {
                matched.push(index);
            }
        }
        for (i, &c) in word.iter().enumerate() {
            if let Some(ref node) = root.children[c] {
                if let Some(index) = node.terminal {
                    if Trie::is_palindrome(&word[i + 1..]) {
                        matched.push(index);
                    }
                }
                root = node;
            } else {
                return (matched, prefixed);
            }
        }
        for node in root.children.iter().flatten() {
            for &p in &node.prefixed {
                prefixed.push(p);
            }
        }

        (matched, prefixed)
    }

    fn is_palindrome(s: &[usize]) -> bool {
        if s.is_empty() {
            return true;
        }
        let (mut i, mut j) = (0, s.len() - 1);
        while i < j && s[i] == s[j] {
            i += 1;
            j -= 1;
        }
        i >= j
    }
}

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut trie = Trie::new();
        let mut pairs = Vec::new();
        for (i, w) in words.iter().enumerate() {
            trie.insert(i, w);
        }
        for (i, w) in words.iter().enumerate() {
            let res = trie.search(w);
            for &j in &res.0 {
                if i != j {
                    pairs.push(vec![j as i32, i as i32]);
                }
            }
            for &j in &res.1 {
                if i != j {
                    let rw: Vec<usize> = words[j]
                        .chars()
                        .skip(words[i].len())
                        .map(|c| c as usize - 'a' as usize)
                        .collect();
                    if Trie::is_palindrome(&rw) {
                        pairs.push(vec![j as i32, i as i32]);
                    }
                }
            }
        }
        pairs.sort_unstable();
        pairs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_pairs() {
        assert_eq!(
            Solution::palindrome_pairs(vec![
                "abcd".to_owned(),
                "dcba".to_owned(),
                "lls".to_owned(),
                "s".to_owned(),
                "sssll".to_owned()
            ]),
            vec![vec![0, 1], vec![1, 0], vec![2, 4], vec![3, 2]]
        );
        assert_eq!(
            Solution::palindrome_pairs(vec!["bat".to_owned(), "tab".to_owned(), "cat".to_owned()]),
            vec![vec![0, 1], vec![1, 0]]
        );
        assert_eq!(
            Solution::palindrome_pairs(vec!["a".to_owned(), "".to_owned()]),
            vec![vec![0, 1], vec![1, 0]]
        );
    }
}
