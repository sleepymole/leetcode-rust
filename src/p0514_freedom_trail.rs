#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let n = ring.len();
        let ring: Vec<char> = ring.chars().collect();
        let mut f = vec![vec![i32::MAX; n]; 2];
        f[0][0] = 0;
        let mut k = 0;
        for c in key.chars() {
            let k2 = k ^ 1;
            for i in 0..n {
                f[k2][i] = i32::MAX;
            }
            for i in 0..n {
                if f[k][i] == i32::MAX {
                    continue;
                }
                for j in 0..n {
                    if ring[j] != c {
                        continue;
                    }
                    let mut d = (i + n - j) % n;
                    d = d.min(n - d);
                    f[k2][j] = f[k2][j].min(f[k][i] + d as i32);
                }
            }
            k = k2;
        }
        key.len() as i32 + f[k].iter().min().cloned().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_rotate_steps() {
        assert_eq!(
            Solution::find_rotate_steps("godding".to_owned(), "gd".to_owned()),
            4
        );
        assert_eq!(
            Solution::find_rotate_steps("godding".to_owned(), "godding".to_owned()),
            13
        );
    }
}
