#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn compress(chars: &mut Vec<char>) -> i32 {
        let mut i = 0;
        let mut n = 0;
        while i < chars.len() {
            let mut j = i + 1;
            while j < chars.len() && chars[j] == chars[j - 1] {
                j += 1;
            }
            chars[n] = chars[i];
            n += 1;
            if j - i > 1 {
                for c in (j - i).to_string().chars() {
                    chars[n] = c;
                    n += 1;
                }
            }
            i = j;
        }
        chars.truncate(n);
        n as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress() {
        let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        assert_eq!(Solution::compress(&mut chars), 6);
        assert_eq!(chars, vec!['a', '2', 'b', '2', 'c', '3']);
        let mut chars = vec!['a'];
        assert_eq!(Solution::compress(&mut chars), 1);
        assert_eq!(chars, vec!['a']);
        let mut chars = vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
        ];
        assert_eq!(Solution::compress(&mut chars), 4);
        assert_eq!(chars, vec!['a', 'b', '1', '2']);
        let mut chars = vec!['a', 'a', 'a', 'b', 'b', 'a', 'a'];
        assert_eq!(Solution::compress(&mut chars), 6);
        assert_eq!(chars, vec!['a', '3', 'b', '2', 'a', '2']);
    }
}
