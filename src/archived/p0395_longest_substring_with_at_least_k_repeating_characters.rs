#![allow(dead_code)]
pub struct Solution;

impl Solution {
    fn search(s: &[usize], k: i32) -> i32 {
        if (s.len() as i32) < k {
            return 0;
        }
        let mut counts = [0; 26];
        for &c in s {
            counts[c] += 1;
        }
        for (c, &r) in counts.iter().enumerate() {
            if r > 0 && r < k {
                let mut longest = 0;
                let mut i = 0;
                while i < s.len() {
                    if s[i] == c {
                        i += 1;
                        continue;
                    }
                    let mut j = i + 1;
                    while j < s.len() && s[j] != c {
                        j += 1;
                    }
                    longest = longest.max(Solution::search(&s[i..j], k));
                    i = j;
                }
                return longest;
            }
        }
        s.len() as i32
    }

    pub fn longest_substring(s: String, k: i32) -> i32 {
        let s: Vec<usize> = s.chars().map(|c| c as usize - 'a' as usize).collect();
        Solution::search(&s, k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_substring() {
        assert_eq!(Solution::longest_substring("aaabb".to_owned(), 3), 3);
        assert_eq!(Solution::longest_substring("ababbc".to_owned(), 2), 5);
    }
}
