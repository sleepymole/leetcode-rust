#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn smallest_good_base(n: String) -> String {
        let n: i64 = n.parse().unwrap();
        for b in 2..n {
            let mut x = 0;
            let mut k = 0;
            while x <= (n - 1) / b {
                x = x * b + 1;
                k += 1;
            }
            if x == n {
                return b.to_string();
            }
            if k <= 5 {
                break;
            }
        }

        // r^4 + r^3 + r^2 + r + 1  = n
        let mut r = (n as f64).powf(0.25);
        for _ in 0..100 {
            r = r
                - (r.powi(4) + r.powi(3) + r.powi(2) + r + 1.0 - n as f64)
                    / (4.0 * r.powi(3) + 3.0 * r.powi(2) + 2.0 * r + 1.0);
        }
        let r = ((r - 1.0) as i64).max(2);
        for b in r..r + 3 {
            if (b.pow(4) + b.pow(3) + b.pow(2) + b + 1) == n {
                return b.to_string();
            }
        }

        // r^3 + r^2 + r + 1  = n
        let mut r = (n as f64).powf(1.0 / 3.0);
        for _ in 0..100 {
            r = r
                - (r.powi(3) + r.powi(2) + r + 1.0 - n as f64) / (3.0 * r.powi(2) + 2.0 * r + 1.0);
        }
        let r = ((r - 1.0) as i64).max(2);
        for b in r..r + 3 {
            if (b.pow(3) + b.pow(2) + b + 1) == n {
                return b.to_string();
            }
        }

        //  r^2 + r + 1  = n
        let mut r = (n as f64).powf(0.5);
        for _ in 0..100 {
            r = r - (r.powi(2) + r + 1.0 - n as f64) / (2.0 * r + 1.0);
        }
        let r = ((r - 1.0) as i64).max(2);
        for b in r..r + 3 {
            if (b.pow(2) + b + 1) == n {
                return b.to_string();
            }
        }

        (n - 1).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_good_base() {
        assert_eq!(
            Solution::smallest_good_base("13".to_owned()),
            "3".to_owned()
        );
        assert_eq!(
            Solution::smallest_good_base("4681".to_owned()),
            "8".to_owned()
        );
        assert_eq!(
            Solution::smallest_good_base("1000000000000000000".to_owned()),
            "999999999999999999".to_owned()
        );
        assert_eq!(
            Solution::smallest_good_base("3541".to_owned()),
            "59".to_owned()
        );
    }
}
