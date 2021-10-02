#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    fn get_fa(fa: &mut [usize], i: usize) -> usize {
        if fa[i] == i {
            i
        } else {
            fa[i] = Solution::get_fa(fa, fa[i]);
            fa[i]
        }
    }

    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let n = is_connected.len();
        let mut fa = vec![0; n];
        for i in 0..n {
            fa[i] = i;
        }
        let mut num = n;
        for i in 0..n {
            for j in i + 1..n {
                if is_connected[i][j] == 1 {
                    let a = Solution::get_fa(&mut fa, i);
                    let b = Solution::get_fa(&mut fa, j);
                    if a != b {
                        fa[b] = a;
                        num -= 1;
                    }
                }
            }
        }
        num as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_circle_num() {
        assert_eq!(
            Solution::find_circle_num(vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]]),
            2
        );
        assert_eq!(
            Solution::find_circle_num(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]),
            3
        );
    }
}
