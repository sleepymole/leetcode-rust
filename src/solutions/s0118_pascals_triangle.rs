#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut rows: Vec<Vec<_>> = Vec::new();
        for i in 1..=num_rows as usize {
            let mut row = vec![1; i];
            for j in 1..i - 1 {
                row[j] = rows[i - 2][j - 1] + rows[i - 2][j];
            }
            rows.push(row);
        }
        rows
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        assert_eq!(
            Solution::generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }
}
