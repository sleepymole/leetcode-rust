#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
        let mut houses = houses;
        houses.sort_unstable();
        let mut heaters = heaters;
        heaters.sort_unstable();
        let mut res = 0;
        let mut i = 0;
        for x in houses {
            while i < heaters.len() && heaters[i] < x {
                i += 1;
            }
            let mut r = i32::MAX;
            if i > 0 {
                r = r.min(x - heaters[i - 1]);
            }
            if i < heaters.len() {
                r = r.min(heaters[i] - x);
            }
            res = res.max(r)
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_radius() {
        assert_eq!(Solution::find_radius(vec![1, 2, 3], vec![2]), 1);
        assert_eq!(Solution::find_radius(vec![1, 2, 3, 4], vec![1, 4]), 1);
        assert_eq!(Solution::find_radius(vec![1, 5], vec![2]), 3);
    }
}
