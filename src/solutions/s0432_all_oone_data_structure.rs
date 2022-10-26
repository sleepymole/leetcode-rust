#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

#[derive(Debug, Default)]
struct Seat {
    keys: HashSet<String>,
    prev: usize,
    next: usize,
}

impl Seat {
    fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug)]
struct AllOne {
    keys: HashMap<String, i32>,
    seats: Vec<Seat>,
}

impl AllOne {
    fn new() -> Self {
        AllOne {
            keys: HashMap::new(),
            seats: vec![Seat::new()],
        }
    }

    fn inc(&mut self, key: String) {
        let count = {
            let v = self.keys.entry(key.clone()).or_insert(0);
            *v += 1;
            *v as usize
        };
        if count >= self.seats.len() {
            self.seats.push(Seat::new());
        }
        self.seats[count].keys.insert(key.clone());
        self.seats[count - 1].keys.remove(&key);
        if self.seats[count].keys.len() == 1 {
            let next = self.seats[count - 1].next;
            assert!(next != count);
            self.seats[count].next = next;
            if next > 0 {
                self.seats[next].prev = count;
            }
        }
        if count > 1 && self.seats[count - 1].keys.is_empty() {
            let prev = self.seats[count - 1].prev;
            self.seats[count - 1].prev = 0;
            self.seats[count - 1].next = 0;
            self.seats[count].prev = prev;
            self.seats[prev].next = count;
        } else {
            self.seats[count].prev = count - 1;
            self.seats[count - 1].next = count;
        }
    }

    fn dec(&mut self, key: String) {
        let count;
        if let Some(v) = self.keys.get_mut(&key) {
            *v -= 1;
            count = *v as usize;
            if count == 0 {
                self.keys.remove(&key);
            }
        } else {
            return;
        }
        if count > 0 {
            self.seats[count].keys.insert(key.clone());
        }
        self.seats[count + 1].keys.remove(&key);
        if self.seats[count].keys.len() == 1 {
            let prev = self.seats[count + 1].prev;
            assert!(prev != count);
            self.seats[count].prev = prev;
            self.seats[prev].next = count;
        }
        if self.seats[count + 1].keys.is_empty() {
            let next = self.seats[count + 1].next;
            self.seats[count + 1].prev = 0;
            self.seats[count + 1].next = 0;
            self.seats[count].next = next;
            if next == 0 {
                self.seats.truncate(count + 1);
            } else {
                self.seats[next].prev = count;
            }
        } else {
            self.seats[count].next = count + 1;
            self.seats[count + 1].prev = count;
        }
    }

    fn get_max_key(&self) -> String {
        if self.seats.len() <= 1 {
            return "".to_owned();
        }
        let last = self.seats.last().unwrap();
        last.keys.iter().next().unwrap().clone()
    }

    fn get_min_key(&self) -> String {
        if self.seats.len() <= 1 {
            return "".to_owned();
        }
        let min_idx = self.seats[0].next;
        self.seats[min_idx].keys.iter().next().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_one() {
        let mut all_one = AllOne::new();
        all_one.inc("hello".to_owned());
        all_one.inc("hello".to_owned());
        assert_eq!(all_one.get_max_key(), "hello".to_owned());
        assert_eq!(all_one.get_min_key(), "hello".to_owned());
        all_one.inc("leet".to_owned());
        assert_eq!(all_one.get_max_key(), "hello".to_owned());
        assert_eq!(all_one.get_min_key(), "leet".to_owned());
    }
}
