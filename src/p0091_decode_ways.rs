#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if s.is_empty() {
            return 1;
        }
        let s = s.into_bytes();
        let mut f = Vec::new();
        f.resize(s.len(), 0);
        if s[0] > '0' as u8 {
            f[0] = 1;
        }
        for i in 1..s.len() {
            if s[i] > '0' as u8 {
                f[i] = f[i - 1]
            }
            if s[i - 1] == '0' as u8 {
                continue;
            }
            let x = (s[i - 1] - '0' as u8) * 10 + s[i] - '0' as u8;
            if x > 0 && x <= 26 {
                if i >= 2 {
                    f[i] += f[i - 2];
                } else {
                    f[i] += 1;
                }
            }
        }
        f[s.len() - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_decodings() {
        assert_eq!(Solution::num_decodings("12".to_owned()), 2);
        assert_eq!(Solution::num_decodings("226".to_owned()), 3);
        assert_eq!(Solution::num_decodings("0".to_owned()), 0);
        assert_eq!(Solution::num_decodings("1".to_owned()), 1);
        assert_eq!(Solution::num_decodings("2101".to_owned()), 1);
    }
}
