#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn magical_string(n: i32) -> i32 {
        if n <= 3 {
            return 1;
        }
        let n = n as usize;
        let mut f = vec![0; n];
        f[0] = 1;
        f[1] = 2;
        f[2] = 2;
        let mut i = 3;
        let mut j = 2;
        while i < n {
            f[i] = f[i - 1] ^ 3;
            if i + 1 < n && f[j] == 2 {
                f[i + 1] = f[i];
                i += 1;
            }
            i += 1;
            j += 1;
        }
        f.into_iter().filter(|&x| x == 1).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magical_string() {
        assert_eq!(Solution::magical_string(6), 3);
        assert_eq!(Solution::magical_string(1), 1);
    }
}
