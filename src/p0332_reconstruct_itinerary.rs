#![allow(dead_code)]
pub struct Solution;

use std::collections::{HashMap, VecDeque};
use std::iter::FromIterator;

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut graph = HashMap::new();
        for t in &tickets {
            graph
                .entry(t[0].as_str())
                .or_insert_with(VecDeque::new)
                .push_back(t[1].as_str());
        }
        for v in graph.values_mut() {
            let mut vec = Vec::from_iter(v.clone());
            vec.sort_unstable();
            *v = VecDeque::from_iter(vec);
        }
        let mut paths = Vec::new();
        let mut trace = vec!["JFK"];
        while !trace.is_empty() {
            let mut advanced = false;
            let &from = trace.last().unwrap();
            if let Some(tos) = graph.get_mut(from) {
                if let Some(to) = tos.pop_front() {
                    trace.push(to);
                    advanced = true;
                }
            }
            if !advanced {
                paths.push(trace.pop().unwrap().to_owned());
            }
        }
        paths.reverse();
        paths
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_itinerary() {
        assert_eq!(
            Solution::find_itinerary(vec![
                vec!["MUC".to_owned(), "LHR".to_owned()],
                vec!["JFK".to_owned(), "MUC".to_owned()],
                vec!["SFO".to_owned(), "SJC".to_owned()],
                vec!["LHR".to_owned(), "SFO".to_owned()]
            ]),
            vec![
                "JFK".to_owned(),
                "MUC".to_owned(),
                "LHR".to_owned(),
                "SFO".to_owned(),
                "SJC".to_owned()
            ]
        );
        assert_eq!(
            Solution::find_itinerary(vec![
                vec!["JFK".to_owned(), "SFO".to_owned()],
                vec!["JFK".to_owned(), "ATL".to_owned()],
                vec!["SFO".to_owned(), "ATL".to_owned()],
                vec!["ATL".to_owned(), "JFK".to_owned()],
                vec!["ATL".to_owned(), "SFO".to_owned()]
            ]),
            vec![
                "JFK".to_owned(),
                "ATL".to_owned(),
                "JFK".to_owned(),
                "SFO".to_owned(),
                "ATL".to_owned(),
                "SFO".to_owned()
            ]
        );
    }
}
