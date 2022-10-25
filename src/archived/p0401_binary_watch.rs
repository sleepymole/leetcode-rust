#![allow(dead_code)]
pub struct Solution;

impl Solution {
    fn count_bits(x: i32) -> i32 {
        let mut x = x;
        let mut n = 0;
        while x > 0 {
            if (x & 1) > 0 {
                n += 1;
            }
            x >>= 1;
        }
        n
    }

    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let mut times = Vec::new();
        for i in 0..12 {
            for j in 0..60 {
                if Solution::count_bits(i) + Solution::count_bits(j) == turned_on {
                    times.push(format!("{i}:{j:02}"));
                }
            }
        }
        times
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_binary_watch() {
        assert_eq!(
            Solution::read_binary_watch(1),
            vec!["0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00"]
        );
        assert_eq!(Solution::read_binary_watch(9), Vec::<String>::new());
    }
}
