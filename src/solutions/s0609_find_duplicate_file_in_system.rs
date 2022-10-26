#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        let mut map = std::collections::HashMap::new();
        for path in paths {
            let mut iter = path.split_whitespace();
            let dir = iter.next().unwrap();
            for file in iter {
                let mut iter = file.split('(');
                let name = iter.next().unwrap();
                let content = iter.next().unwrap();
                let content = &content[..content.len() - 1];
                map.entry(content.to_owned())
                    .or_insert_with(Vec::new)
                    .push(format!("{}/{}", dir, name));
            }
        }
        map.into_iter()
            .filter(|(_, v)| v.len() > 1)
            .map(|(_, v)| v)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicate() {
        assert_eq!(
            Solution::find_duplicate(vec![
                "root/a 1.txt(abcd) 2.txt(efgh)".to_owned(),
                "root/c 3.txt(abcd)".to_owned(),
                "root/c/d 4.txt(efgh)".to_owned(),
                "root 4.txt(efgh)".to_owned()
            ]),
            vec![
                vec![
                    "root/a/2.txt".to_owned(),
                    "root/c/d/4.txt".to_owned(),
                    "root/4.txt".to_owned()
                ],
                vec!["root/a/1.txt".to_owned(), "root/c/3.txt".to_owned()]
            ]
        );
        assert_eq!(
            Solution::find_duplicate(vec![
                "root/a 1.txt(abcd) 2.txt(efgh)".to_owned(),
                "root/c 3.txt(abcd)".to_owned(),
                "root/c/d 4.txt(efgh)".to_owned()
            ]),
            vec![
                vec!["root/a/2.txt".to_owned(), "root/c/d/4.txt".to_owned()],
                vec!["root/a/1.txt".to_owned(), "root/c/3.txt".to_owned()]
            ]
        );
    }
}
