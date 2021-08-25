#![allow(dead_code)]
pub struct Solution {}

use std::num::ParseIntError;
use std::ops::{Add, Sub};
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TimePoint(i32);

impl Add for TimePoint {
    type Output = Self;

    fn add(self, rhs: TimePoint) -> Self::Output {
        TimePoint(self.0 + rhs.0)
    }
}

impl Sub for TimePoint {
    type Output = Self;

    fn sub(self, rhs: TimePoint) -> Self::Output {
        TimePoint(self.0 - rhs.0)
    }
}

impl FromStr for TimePoint {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = s.splitn(2, ':').collect();
        let h = v[0].parse::<i32>()?;
        let m = v[1].parse::<i32>()?;
        Ok(TimePoint(h * 60 + m))
    }
}

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut time_points: Vec<TimePoint> = time_points
            .into_iter()
            .map(|s| s.parse::<TimePoint>().unwrap())
            .collect();
        time_points.sort_unstable();
        let n = time_points.len();
        let mut min_diff = TimePoint(1440) - time_points[n - 1] + time_points[0];
        for i in 1..time_points.len() {
            min_diff = min_diff.min(time_points[i] - time_points[i - 1]);
        }
        min_diff.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min_difference() {
        assert_eq!(
            Solution::find_min_difference(vec!["23:59".to_owned(), "00:00".to_owned()]),
            1
        );
        assert_eq!(
            Solution::find_min_difference(vec![
                "00:00".to_owned(),
                "23:59".to_owned(),
                "00:00".to_owned()
            ]),
            0
        );
    }
}
