#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        if points.is_empty() {
            return 0;
        }
        let mut points = points;
        points.sort_unstable_by(|p1, p2| p1[1].cmp(&p2[1]));
        let mut pivot = points[0][1];
        let mut shots = 1;
        for i in 1..points.len() {
            if points[i][0] > pivot {
                shots += 1;
                pivot = points[i][1];
            }
        }
        shots
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min_arrow_shots() {
        assert_eq!(
            Solution::find_min_arrow_shots(vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]]),
            2
        );
        assert_eq!(
            Solution::find_min_arrow_shots(vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]]),
            4
        );
        assert_eq!(
            Solution::find_min_arrow_shots(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]),
            2
        );
    }
}
