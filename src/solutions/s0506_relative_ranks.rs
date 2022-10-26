#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut sorted = score.clone();
        sorted.sort_unstable();
        let mut ranks = Vec::new();
        for i in 0..score.len() {
            let (mut l, mut r) = (0, score.len());
            while l < r {
                let m = (l + r) / 2;
                if sorted[m] >= score[i] {
                    r = m;
                } else {
                    l = m + 1;
                }
            }
            ranks.push(match score.len() - l {
                1 => "Gold Medal".to_owned(),
                2 => "Silver Medal".to_owned(),
                3 => "Bronze Medal".to_owned(),
                r => r.to_string(),
            });
        }
        ranks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_relative_ranks() {
        assert_eq!(
            Solution::find_relative_ranks(vec![5, 4, 3, 2, 1]),
            vec![
                "Gold Medal".to_owned(),
                "Silver Medal".to_owned(),
                "Bronze Medal".to_owned(),
                "4".to_owned(),
                "5".to_owned()
            ]
        );
        assert_eq!(
            Solution::find_relative_ranks(vec![10, 3, 8, 9, 4]),
            vec![
                "Gold Medal".to_owned(),
                "5".to_owned(),
                "Bronze Medal".to_owned(),
                "Silver Medal".to_owned(),
                "4".to_owned()
            ]
        );
    }
}
