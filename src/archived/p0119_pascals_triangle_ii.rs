#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut rows: Vec<Vec<_>> = Vec::new();
        for i in 0..=row_index as usize {
            let mut row = vec![1; i + 1];
            for j in 1..i {
                row[j] = rows[i - 1][j - 1] + rows[i - 1][j];
            }
            rows.push(row);
        }
        rows.last().cloned().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_row() {
        assert_eq!(Solution::get_row(3), vec![1, 3, 3, 1]);
        assert_eq!(Solution::get_row(0), vec![1]);
        assert_eq!(Solution::get_row(1), vec![1, 1]);
    }
}
