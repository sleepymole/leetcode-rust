#![allow(dead_code)]

use std::collections::VecDeque;

#[derive(Default)]
struct WordDictionary {
    children: [Option<Box<WordDictionary>>; 26],
    is_terminal: bool,
}

impl WordDictionary {
    fn new() -> Self {
        Default::default()
    }

    fn get_index(c: char) -> usize {
        c as usize - 'a' as usize
    }

    fn add_word(&mut self, word: String) {
        let mut root = self;
        for i in word.chars().map(WordDictionary::get_index) {
            root = root.children[i].get_or_insert(Box::new(WordDictionary::new()));
        }
        root.is_terminal = true;
    }

    fn dfs(&self, mut word: VecDeque<char>) -> bool {
        if word.is_empty() {
            return self.is_terminal;
        }
        let c = word.pop_front().unwrap();
        if c == '.' {
            for i in 0..26 {
                if let Some(ref node) = self.children[i] {
                    if WordDictionary::dfs(node, word.clone()) {
                        return true;
                    }
                }
            }
            return false;
        }
        if let Some(ref node) = self.children[WordDictionary::get_index(c)] {
            return WordDictionary::dfs(node, word);
        }
        false
    }

    fn search(&self, word: String) -> bool {
        self.dfs(word.chars().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_dictionary() {
        let mut dict = WordDictionary::new();
        dict.add_word("bad".to_owned());
        dict.add_word("dad".to_owned());
        dict.add_word("mad".to_owned());
        assert_eq!(dict.search("pad".to_owned()), false);
        assert_eq!(dict.search("bad".to_owned()), true);
        assert_eq!(dict.search(".ad".to_owned()), true);
        assert_eq!(dict.search("b..".to_owned()), true);
    }
}
