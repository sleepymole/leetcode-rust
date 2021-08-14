#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn find_min_moves(machines: Vec<i32>) -> i32 {
        if machines.is_empty() {
            return 0;
        }
        let sum: i32 = machines.iter().sum();
        if sum % machines.len() as i32 != 0 {
            return -1;
        }
        let avg = sum / machines.len() as i32;
        let mut moves = 0;
        let mut delta = 0;
        for m in machines {
            if delta < 0 && m + delta > avg {
                moves = moves.max(-delta + m + delta - avg);
            } else {
                moves = moves.max(delta.abs().max((m + delta - avg).abs()));
            }
            delta = m + delta - avg;
        }
        moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min_moves() {
        assert_eq!(Solution::find_min_moves(vec![1, 0, 5]), 3);
        // assert_eq!(Solution::find_min_moves(vec![0, 3, 0]), 3);
    }
}
