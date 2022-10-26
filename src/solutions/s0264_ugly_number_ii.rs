#![allow(dead_code)]
pub struct Solution {}

use std::i32;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut l = 1;
        let mut r = i32::MAX;
        while l < r {
            let m = l + (r - l) / 2;
            let mut a = 1;
            let mut count = 0;
            loop {
                let mut b = 1;
                loop {
                    let mut c = 1;
                    loop {
                        count += 1;
                        if a * b * c > m / 5 {
                            break;
                        }
                        c *= 5;
                    }
                    if a * b > m / 3 {
                        break;
                    }
                    b *= 3;
                }
                if a > m / 2 {
                    break;
                }
                a *= 2;
            }
            if count >= n {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_ugly_number() {
        assert_eq!(Solution::nth_ugly_number(10), 12);
    }
}
