#![allow(dead_code)]
pub struct Solution;

impl Solution {
    pub fn max_envelopes(envelopes: Vec<Vec<i32>>) -> i32 {
        let mut envelopes = envelopes;
        envelopes.sort_unstable();
        let mut f = Vec::new();
        let mut i = 0;
        while i < envelopes.len() {
            let mut updates = Vec::new();
            let mut j = i;
            while j < envelopes.len() && envelopes[i][0] == envelopes[j][0] {
                let x = envelopes[j][1];
                let (mut l, mut r) = (0, f.len());
                while l < r {
                    let m = (l + r) / 2;
                    if f[m] >= x {
                        r = m;
                    } else {
                        l = m + 1;
                    }
                }
                updates.push((l, x));
                j += 1;
            }
            i = j;
            for (i, x) in updates {
                if i == f.len() {
                    f.push(x);
                } else {
                    f[i] = f[i].min(x);
                }
            }
        }
        f.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_envelopes() {
        assert_eq!(
            Solution::max_envelopes(vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]]),
            3
        );
        assert_eq!(
            Solution::max_envelopes(vec![vec![1, 1], vec![1, 1], vec![1, 1]]),
            1
        );
        assert_eq!(
            Solution::max_envelopes(vec![
                vec![16, 1],
                vec![6, 9],
                vec![16, 12],
                vec![8, 7],
                vec![18, 16],
                vec![8, 10],
                vec![13, 19],
                vec![16, 7],
                vec![7, 20],
                vec![13, 6],
                vec![9, 11],
                vec![10, 13],
                vec![15, 19],
                vec![8, 11]
            ]),
            5
        );
    }
}
