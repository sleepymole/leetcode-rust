#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        let points = vec![p1, p2, p3, p4];
        let mut d2 = Vec::new();
        for i in 0..4 {
            for j in i + 1..4 {
                let a = points[j][0] - points[i][0];
                let b = points[j][1] - points[i][1];
                d2.push(a * a + b * b);
            }
        }
        d2.sort_unstable();
        d2[0..4].iter().all(|&v| v > 0 && v == d2[0]) && d2[4] == d2[5] && 2 * d2[0] == d2[4]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_square() {
        assert!(Solution::valid_square(
            vec![0, 0],
            vec![1, 1],
            vec![1, 0],
            vec![0, 1]
        ));
        assert!(!Solution::valid_square(
            vec![0, 0],
            vec![1, 1],
            vec![1, 0],
            vec![0, 12]
        ));
        assert!(Solution::valid_square(
            vec![1, 0],
            vec![-1, 0],
            vec![0, 1],
            vec![0, -1]
        ));
    }
}
