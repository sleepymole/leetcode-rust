#![allow(dead_code)]
struct NumMatrix {
    sum: Vec<Vec<i32>>,
}

impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut sum = matrix;
        let m = sum.len();
        let n = sum[0].len();
        for i in 0..m {
            for j in 0..n {
                if i > 0 {
                    sum[i][j] += sum[i - 1][j];
                }
                if j > 0 {
                    sum[i][j] += sum[i][j - 1];
                }
                if i > 0 && j > 0 {
                    sum[i][j] -= sum[i - 1][j - 1];
                }
            }
        }
        NumMatrix { sum }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let (row1, col1, row2, col2) = (row1 as usize, col1 as usize, row2 as usize, col2 as usize);
        let mut sum = self.sum[row2][col2];
        if row1 > 0 {
            sum -= self.sum[row1 - 1][col2];
        }
        if col1 > 0 {
            sum -= self.sum[row2][col1 - 1];
        }
        if row1 > 0 && col1 > 0 {
            sum += self.sum[row1 - 1][col1 - 1];
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_matrix() {
        let m = NumMatrix::new(vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5],
        ]);
        assert_eq!(m.sum_region(2, 1, 4, 3), 8);
        assert_eq!(m.sum_region(1, 1, 2, 2), 11);
        assert_eq!(m.sum_region(1, 2, 2, 4), 12);
    }
}
