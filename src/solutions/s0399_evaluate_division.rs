#![allow(dead_code)]
pub struct Solution;

use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut graph: HashMap<&str, HashMap<&str, f64>> = HashMap::new();
        for i in 0..equations.len() {
            graph
                .entry(&equations[i][0])
                .or_insert_with(HashMap::new)
                .insert(&equations[i][1], 1.0 / values[i]);
            graph
                .entry(&equations[i][1])
                .or_insert_with(HashMap::new)
                .insert(&equations[i][0], values[i]);
        }
        let mut score: HashMap<&str, (f64, i32)> = HashMap::new();
        let mut group: HashMap<&str, i32> = HashMap::new();
        let mut gid = 0;
        for &a in graph.keys() {
            if score.contains_key(&a) {
                continue;
            }
            let mut q = VecDeque::new();
            q.push_back(a);
            score.insert(a, (1.0, gid));
            group.insert(a, gid);
            while let Some(a) = q.pop_front() {
                let &(s, _) = score.get(&a).unwrap();
                if let Some(m) = graph.get(&a) {
                    for (&b, &div) in m {
                        if score.contains_key(&b) {
                            continue;
                        }
                        score.insert(b, (s * div, gid));
                        q.push_back(b);
                    }
                }
            }
            gid += 1;
        }
        let mut res = Vec::new();
        for q in queries {
            res.push(match (score.get(q[0].as_str()), score.get(q[1].as_str())) {
                (Some(&(a, g1)), Some(&(b, g2))) => {
                    if g1 == g2 {
                        a / b
                    } else {
                        -1.0
                    }
                }
                _ => -1.0,
            });
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_equation() {
        assert_eq!(
            Solution::calc_equation(
                vec![
                    vec!["a".to_owned(), "b".to_owned()],
                    vec!["b".to_owned(), "c".to_owned()]
                ],
                vec![2.0, 3.0],
                vec![
                    vec!["a".to_owned(), "c".to_owned()],
                    vec!["b".to_owned(), "a".to_owned()],
                    vec!["a".to_owned(), "e".to_owned()],
                    vec!["a".to_owned(), "a".to_owned()],
                    vec!["x".to_owned(), "x".to_owned()]
                ]
            ),
            vec![6.00000, 0.50000, -1.00000, 1.00000, -1.00000]
        );
        assert_eq!(
            Solution::calc_equation(
                vec![
                    vec!["a".to_owned(), "b".to_owned()],
                    vec!["b".to_owned(), "c".to_owned()],
                    vec!["bc".to_owned(), "cd".to_owned()]
                ],
                vec![1.5, 2.5, 5.0],
                vec![
                    vec!["a".to_owned(), "c".to_owned()],
                    vec!["c".to_owned(), "b".to_owned()],
                    vec!["bc".to_owned(), "cd".to_owned()],
                    vec!["cd".to_owned(), "bc".to_owned()]
                ]
            ),
            vec![3.75000, 0.40000, 5.00000, 0.20000]
        );
        assert_eq!(
            Solution::calc_equation(
                vec![vec!["a".to_owned(), "b".to_owned()]],
                vec![0.5],
                vec![
                    vec!["a".to_owned(), "b".to_owned()],
                    vec!["b".to_owned(), "a".to_owned()],
                    vec!["a".to_owned(), "c".to_owned()],
                    vec!["x".to_owned(), "y".to_owned()]
                ]
            ),
            vec![0.50000, 2.00000, -1.00000, -1.00000]
        );
    }
}
