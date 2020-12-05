#![allow(dead_code)]
pub struct Solution;

use std::char;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut res = vec!['1'];
        for _ in 0..n - 1 {
            let mut tmp = vec![];
            let mut i = 0;
            while i < res.len() {
                let mut j = i + 1;
                while j < res.len() && res[j] == res[i] {
                    j += 1;
                }
                tmp.push(char::from_digit((j - i) as u32, 10).unwrap());
                tmp.push(res[i]);
                i = j;
            }
            res = tmp;
        }
        res.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_and_say() {
        assert_eq!(Solution::count_and_say(1), "1");
        assert_eq!(Solution::count_and_say(2), "11");
        assert_eq!(Solution::count_and_say(3), "21");
        assert_eq!(Solution::count_and_say(4), "1211");
        assert_eq!(Solution::count_and_say(5), "111221");
    }
}
