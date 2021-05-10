#![allow(dead_code)]
struct NumArray {
    sum: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut sum = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            sum[i + 1] = sum[i] + nums[i];
        }
        NumArray { sum }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.sum[right as usize + 1] - self.sum[left as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_array() {
        let arr = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
        assert_eq!(arr.sum_range(0, 2), 1);
        assert_eq!(arr.sum_range(2, 5), -1);
        assert_eq!(arr.sum_range(0, 5), -3);
    }
}
