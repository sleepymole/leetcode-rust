#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        if s.is_empty() {
            return vec![];
        }
        let mut vs = vec!['#'];
        for c in s.chars() {
            vs.push(c);
            vs.push('#');
        }
        let mut f = vec![0; vs.len()];
        let mut pr = (0, 0);
        for i in 1..vs.len() {
            if pr.0 + pr.1 <= i {
                let mut j = 0;
                while i > j && i + j + 1 < vs.len() && vs[i - j - 1] == vs[i + j + 1] {
                    j += 1;
                }
                pr = (i, j);
                f[i] = j;
            } else {
                let mut j = f[2 * pr.0 - i].min(pr.0 + pr.1 - i);
                while i > j && i + j + 1 < vs.len() && vs[i - j - 1] == vs[i + j + 1] {
                    j += 1;
                }
                f[i] = j;
                if pr.0 + pr.1 < i + j {
                    pr = (i, j);
                }
            }
        }
        let s: Vec<char> = s.chars().collect();
        let mut ss: Vec<Vec<Vec<String>>> = vec![vec![]; s.len()];
        for i in 0..s.len() {
            for j in 0..=i {
                if f[i + j + 1] < i - j {
                    continue;
                }
                if j == 0 {
                    ss[i].push(vec![s[..=i].iter().collect()]);
                } else {
                    let mut ssi = Vec::new();
                    for ps in &ss[j - 1] {
                        let mut psc = ps.clone();
                        psc.push(s[j..=i].iter().collect());
                        ssi.push(psc);
                    }
                    ss[i].append(&mut ssi);
                }
            }
        }
        let mut ans = ss[s.len() - 1].clone();
        ans.sort();
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition() {
        assert_eq!(
            Solution::partition("aab".to_owned()),
            vec![
                vec!["a".to_owned(), "a".to_owned(), "b".to_owned()],
                vec!["aa".to_owned(), "b".to_owned()]
            ]
        );
        assert_eq!(
            Solution::partition("a".to_owned()),
            vec![vec!["a".to_owned()]]
        );
    }
}
