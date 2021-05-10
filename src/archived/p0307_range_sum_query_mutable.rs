#![allow(dead_code)]

struct NumArray {
    nums: Vec<i32>,
    sums: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut sums = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            let mut j = i as i32 + 1;
            while j <= nums.len() as i32 {
                sums[j as usize] += nums[i];
                j += j & -j;
            }
        }
        NumArray { nums, sums }
    }

    fn update(&mut self, index: i32, val: i32) {
        let i = index as usize;
        let add = val - self.nums[i];
        let mut j = index + 1;
        while j <= self.nums.len() as i32 {
            self.sums[j as usize] += add;
            j += j & -j;
        }
        self.nums[i] = val;
    }

    fn sum(&self, n: i32) -> i32 {
        let mut j = n + 1;
        let mut sum = 0;
        while j > 0 {
            sum += self.sums[j as usize];
            j -= j & -j;
        }
        sum
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.sum(right) - self.sum(left - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_array() {
        let mut arr = NumArray::new(vec![1, 3, 5]);
        assert_eq!(arr.sum_range(0, 2), 9);
        arr.update(1, 2);
        assert_eq!(arr.sum_range(0, 2), 8);
    }
}
