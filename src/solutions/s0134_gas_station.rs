#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut i = 0;
        while i < gas.len() {
            let mut sum = gas[i] - cost[i];
            let mut j = i + 1;
            while sum >= 0
                && j - i < gas.len()
                && sum + gas[j % gas.len()] - cost[j % gas.len()] >= 0
            {
                sum += gas[j % gas.len()] - cost[j % gas.len()];
                j += 1;
            }
            if sum >= 0 && j - i == gas.len() {
                return i as i32;
            }
            i = j
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_complete_circuit() {
        assert_eq!(
            Solution::can_complete_circuit(vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2]),
            3
        );
        assert_eq!(
            Solution::can_complete_circuit(vec![2, 3, 4], vec![3, 4, 3]),
            -1
        );
    }
}
