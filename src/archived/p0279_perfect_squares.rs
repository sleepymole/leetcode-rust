#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut f = vec![0; n + 1];
        for i in 1..=n {
            f[i] = n;
            let mut j = 1;
            while j <= i / j {
                f[i] = f[i].min(f[i - j * j] + 1);
                j += 1;
            }
        }
        f[n] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_squares() {
        assert_eq!(Solution::num_squares(12), 3);
        assert_eq!(Solution::num_squares(13), 2);
    }
}
