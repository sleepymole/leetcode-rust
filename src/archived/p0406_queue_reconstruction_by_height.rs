#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn reconstruct_queue(people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut people = people;
        people.sort_unstable_by(|a, b| {
            if a[0] != b[0] {
                a[0].cmp(&b[0])
            } else {
                b[1].cmp(&a[1])
            }
        });
        let n = people.len();
        let mut sum = vec![0; people.len() + 1];
        let mut res = vec![vec![]; people.len()];
        for p in people {
            let (mut l, mut r) = (0, n);
            while l < r {
                let m = (l + r) / 2;
                let mut count = 0;
                let mut x = m + 1;
                while x > 0 {
                    count += sum[x];
                    x -= (x as i32 & -(x as i32)) as usize;
                }
                count = (m + 1) - count;
                if count > p[1] as usize {
                    r = m;
                } else {
                    l = m + 1;
                }
            }
            res[l] = p;
            let mut x = l + 1;
            while x <= n {
                sum[x] += 1;
                x += (x as i32 & -(x as i32)) as usize;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reconstruct_queue() {
        assert_eq!(
            Solution::reconstruct_queue(vec![
                vec![7, 0],
                vec![4, 4],
                vec![7, 1],
                vec![5, 0],
                vec![6, 1],
                vec![5, 2]
            ]),
            vec![
                vec![5, 0],
                vec![7, 0],
                vec![5, 2],
                vec![6, 1],
                vec![4, 4],
                vec![7, 1]
            ]
        );
        assert_eq!(
            Solution::reconstruct_queue(vec![
                vec![6, 0],
                vec![5, 0],
                vec![4, 0],
                vec![3, 2],
                vec![2, 2],
                vec![1, 4]
            ]),
            vec![
                vec![4, 0],
                vec![5, 0],
                vec![2, 2],
                vec![3, 2],
                vec![1, 4],
                vec![6, 0]
            ]
        );
    }
}
