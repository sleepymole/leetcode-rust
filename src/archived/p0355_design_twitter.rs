#![allow(dead_code)]

use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

#[derive(Debug, Default, Eq)]
struct Tweet {
    id: i32,
    ts: i32,
}

impl Ord for Tweet {
    fn cmp(&self, other: &Self) -> Ordering {
        other.ts.cmp(&self.ts)
    }
}

impl PartialOrd for Tweet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Tweet {
    fn eq(&self, other: &Self) -> bool {
        self.ts == other.ts
    }
}

impl Clone for Tweet {
    fn clone(&self) -> Self {
        Tweet {
            id: self.id,
            ts: self.ts,
        }
    }
}

#[derive(Default)]
struct Twitter {
    now: i32,
    tweets: HashMap<i32, VecDeque<Tweet>>,
    followees: HashMap<i32, HashSet<i32>>,
}

impl Twitter {
    fn new() -> Self {
        Default::default()
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.now += 1;
        self.tweets
            .entry(user_id)
            .or_insert_with(VecDeque::new)
            .push_back(Tweet {
                id: tweet_id,
                ts: self.now,
            })
    }

    fn get_recent_tweets(&self, user_id: i32, only_self: bool) -> Vec<Tweet> {
        let mut recents = BinaryHeap::new();
        if let Some(tweets) = self.tweets.get(&user_id) {
            for t in tweets.iter().rev().take(10) {
                recents.push(t.clone());
            }
        }
        if only_self {
            return recents.into_sorted_vec();
        }
        if let Some(followees) = self.followees.get(&user_id) {
            for &id in followees {
                for t in self.get_recent_tweets(id, true) {
                    if recents.len() < 10 || t.cmp(recents.peek().unwrap()) == Ordering::Less {
                        recents.push(t);
                        if recents.len() > 10 {
                            recents.pop();
                        }
                    }
                }
            }
        }
        recents.into_sorted_vec()
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        self.get_recent_tweets(user_id, false)
            .iter()
            .map(|t| t.id)
            .collect()
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.followees
            .entry(follower_id)
            .or_insert_with(HashSet::new)
            .insert(followee_id);
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(followees) = self.followees.get_mut(&follower_id) {
            followees.remove(&followee_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_twitter() {
        let mut t = Twitter::new();
        t.post_tweet(1, 5);
        assert_eq!(t.get_news_feed(1), vec![5]);
        t.follow(1, 2);
        t.post_tweet(2, 6);
        assert_eq!(t.get_news_feed(1), vec![6, 5]);
        t.unfollow(1, 2);
        assert_eq!(t.get_news_feed(1), vec![5]);
    }
}
