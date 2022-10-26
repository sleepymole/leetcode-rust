#![allow(dead_code)]
#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_terminal: bool,
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, word: String) {
        let mut root = self;
        for i in word.chars().map(|c| c as usize - 'a' as usize) {
            root = root.children[i].get_or_insert(Box::new(Trie::new()));
        }
        root.is_terminal = true;
    }

    fn search(&self, word: String) -> bool {
        let mut root = self;
        for i in word.chars().map(|c| c as usize - 'a' as usize) {
            match root.children[i] {
                Some(ref node) => {
                    root = node;
                }
                None => {
                    return false;
                }
            }
        }
        root.is_terminal
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut root = self;
        for i in prefix.chars().map(|c| c as usize - 'a' as usize) {
            match root.children[i] {
                Some(ref node) => {
                    root = node;
                }
                None => {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie() {
        let mut trie = Trie::new();
        trie.insert("apple".to_owned());
        assert!(trie.search("apple".to_owned()));
        assert!(!trie.search("app".to_owned()));
        assert!(trie.starts_with("app".to_owned()));
        trie.insert("app".to_owned());
        assert!(trie.search("app".to_owned()));
    }
}
