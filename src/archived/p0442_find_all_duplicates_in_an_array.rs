#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let n = nums.len();
        for i in 0..n {
            while nums[i] > 0 && nums[i] <= n as i32 && nums[i] != (i + 1) as i32 {
                let v = nums[i];
                if nums[v as usize - 1] == v {
                    nums[v as usize - 1] += n as i32;
                    nums[i] = 0;
                    break;
                }
                nums.swap(i, v as usize - 1);
            }
        }
        let mut i = 0;
        for j in 0..n {
            if nums[j] > n as i32 {
                nums[i] = nums[j] - n as i32;
                i += 1;
            }
        }
        nums.truncate(i);
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicates() {
        assert_eq!(
            Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![2, 3]
        );
        assert_eq!(Solution::find_duplicates(vec![1, 1, 2]), vec![1]);
        assert_eq!(Solution::find_duplicates(vec![1]), Vec::<i32>::new());
    }
}
