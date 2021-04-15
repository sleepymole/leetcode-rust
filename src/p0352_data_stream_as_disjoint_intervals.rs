#![allow(dead_code)]

use std::collections::BTreeMap;

#[derive(Default)]
struct SummaryRanges {
    ranges: BTreeMap<i32, i32>,
}

impl SummaryRanges {
    fn new() -> Self {
        Default::default()
    }

    fn add_num(&mut self, val: i32) {
        let (mut l, mut r) = (val, val);
        if let Some((&start, &end)) = self.ranges.range(val + 1..).next() {
            if val + 1 == start {
                r = end;
                self.ranges.remove(&start);
            } else if val >= start && val <= end {
                return;
            }
        }
        if let Some((&start, &end)) = self.ranges.range(..=val).rev().next() {
            if end + 1 == val {
                l = start;
                self.ranges.remove(&start);
            } else if val >= start && val <= end {
                return;
            }
        }
        self.ranges.insert(l, r);
    }

    fn get_intervals(&self) -> Vec<Vec<i32>> {
        self.ranges.iter().map(|(&k, &v)| vec![k, v]).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summary_ranges() {
        let mut sr = SummaryRanges::new();
        sr.add_num(1);
        assert_eq!(sr.get_intervals(), vec![vec![1, 1]]);
        sr.add_num(3);
        assert_eq!(sr.get_intervals(), vec![vec![1, 1], vec![3, 3]]);
        sr.add_num(7);
        assert_eq!(sr.get_intervals(), vec![vec![1, 1], vec![3, 3], vec![7, 7]]);
        sr.add_num(2);
        assert_eq!(sr.get_intervals(), vec![vec![1, 3], vec![7, 7]]);
        sr.add_num(6);
        assert_eq!(sr.get_intervals(), vec![vec![1, 3], vec![6, 7]]);
    }
}
