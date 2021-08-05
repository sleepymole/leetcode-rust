#![allow(dead_code)]
pub struct Solution;

use std::char;

impl Solution {
    pub fn frequency_sort(s: String) -> String {
        let f = |c: char| {
            if ('0'..='9').contains(&c) {
                c as usize - '0' as usize
            } else if ('A'..='Z').contains(&c) {
                c as usize - 'A' as usize + 10
            } else {
                c as usize - 'a' as usize + 36
            }
        };
        let g = |x: usize| {
            if x < 10 {
                char::from_u32((x + '0' as usize) as u32).unwrap()
            } else if x < 36 {
                char::from_u32((x - 10 + 'A' as usize) as u32).unwrap()
            } else {
                char::from_u32((x - 36 + 'a' as usize) as u32).unwrap()
            }
        };
        let mut a = vec![0; 62];
        for c in s.chars() {
            a[f(c)] += 1;
        }
        let mut b = vec![0; 62];
        for i in 0..62 {
            b[i] = i;
        }
        b.sort_unstable_by(|&i, &j| {
            if a[i] != a[j] {
                a[j].cmp(&a[i])
            } else {
                i.cmp(&j)
            }
        });
        let mut s = String::new();
        for i in b {
            for _ in 0..a[i] {
                s.push(g(i));
            }
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_sort() {
        assert_eq!(
            Solution::frequency_sort("tree".to_owned()),
            "eert".to_owned()
        );
        assert_eq!(
            Solution::frequency_sort("cccaaa".to_owned()),
            "aaaccc".to_owned()
        );
        assert_eq!(
            Solution::frequency_sort("Aabb".to_owned()),
            "bbAa".to_owned()
        );
    }
}
