#![allow(dead_code)]

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

use std::collections::VecDeque;

trait Iterator {
    fn next(&mut self) -> i32;
    fn has_next(&mut self) -> bool;
}

struct IntIterator {
    val: i32,
    iterated: bool,
}

impl IntIterator {
    fn new(val: i32) -> Self {
        IntIterator {
            val,
            iterated: false,
        }
    }
}

impl Iterator for IntIterator {
    fn next(&mut self) -> i32 {
        if self.iterated {
            panic!("iterate to end");
        }
        self.iterated = true;
        self.val
    }

    fn has_next(&mut self) -> bool {
        !self.iterated
    }
}

struct ListIterator {
    list: Vec<NestedInteger>,
    iterators: VecDeque<Box<dyn Iterator>>,
}

impl ListIterator {
    fn new(list: Vec<NestedInteger>) -> Self {
        ListIterator {
            list,
            iterators: VecDeque::new(),
        }
    }

    fn lazy_init(&mut self) {
        if self.list.is_empty() {
            return;
        }
        for item in self.list.drain(..) {
            match item {
                NestedInteger::Int(val) => {
                    self.iterators.push_back(Box::new(IntIterator::new(val)));
                }
                NestedInteger::List(list) => {
                    self.iterators.push_back(Box::new(ListIterator::new(list)));
                }
            }
        }
    }
}

impl Iterator for ListIterator {
    fn next(&mut self) -> i32 {
        self.lazy_init();
        while !self.iterators.is_empty() {
            if self.iterators[0].has_next() {
                return self.iterators[0].next();
            }
            self.iterators.pop_front();
        }
        panic!("iterate to end");
    }

    fn has_next(&mut self) -> bool {
        self.lazy_init();
        for iter in &mut self.iterators {
            if iter.has_next() {
                return true;
            }
        }
        false
    }
}

struct NestedIterator {
    inner: Box<dyn Iterator>,
}

impl NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        NestedIterator {
            inner: Box::new(ListIterator::new(nested_list)),
        }
    }

    fn next(&mut self) -> i32 {
        self.inner.next()
    }

    fn has_next(&mut self) -> bool {
        self.inner.has_next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nested_iterator() {
        let mut iter = NestedIterator::new(vec![
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
            NestedInteger::Int(2),
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
        ]);
        assert_eq!(iter.next(), 1);
        assert_eq!(iter.next(), 1);
        assert_eq!(iter.next(), 2);
        assert_eq!(iter.next(), 1);
        assert_eq!(iter.next(), 1);
        assert_eq!(iter.has_next(), false);

        let mut iter = NestedIterator::new(vec![
            NestedInteger::Int(1),
            NestedInteger::List(vec![
                NestedInteger::Int(4),
                NestedInteger::List(vec![NestedInteger::Int(6)]),
            ]),
        ]);
        assert_eq!(iter.next(), 1);
        assert_eq!(iter.next(), 4);
        assert_eq!(iter.next(), 6);
        assert_eq!(iter.has_next(), false);
    }
}
