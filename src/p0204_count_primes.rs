#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let n = n as usize;
        let mut primes = vec![0; n];
        let mut count = 0;
        for i in 2..n {
            if primes[i] == 0 {
                primes[count] = i;
                count += 1;
            }
            for j in 0..count {
                if i > (n - 1) / primes[j] {
                    break;
                }
                let m = i * primes[j];
                primes[m] = 1;
                if i % primes[j] == 0 {
                    break;
                }
            }
        }
        count as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_primes() {
        assert_eq!(Solution::count_primes(10), 4);
        assert_eq!(Solution::count_primes(0), 0);
        assert_eq!(Solution::count_primes(1), 0);
    }
}
