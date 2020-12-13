#![allow(dead_code)]
pub struct Solution;

impl Solution {
    fn parse(s: &Vec<i32>, i: usize, j: usize) -> Option<i32> {
        if s[i] == 0 {
            if i + 1 == j {
                return Some(0);
            }
            return None;
        }
        let mut n = 0;
        for k in i..j {
            n = n * 10 + s[k];
        }
        if n > 255 {
            return None;
        }
        Some(n)
    }

    pub fn restore_ip_addresses(s: String) -> Vec<String> {
        let s: Vec<i32> = s
            .into_bytes()
            .into_iter()
            .map(|x| x as i32 - '0' as i32)
            .collect();
        if s.len() < 4 || s.len() > 12 {
            return vec![];
        }
        let mut ans = Vec::new();
        for i in 1..s.len() {
            
            for j in i + 1..s.len() {
                for k in j + 1..s.len() {
                    if let Some(a) = Solution::parse(&s, 0, i) {
                        if let Some(b) = Solution::parse(&s, i, j) {
                            if let Some(c) = Solution::parse(&s, j, k) {
                                if let Some(d) = Solution::parse(&s, k, s.len()) {
                                    let mut ip = String::new();
                                    ip.push_str(a.to_string().as_str());
                                    ip.push('.');
                                    ip.push_str(b.to_string().as_str());
                                    ip.push('.');
                                    ip.push_str(c.to_string().as_str());
                                    ip.push('.');
                                    ip.push_str(d.to_string().as_str());
                                    ans.push(ip);
                                }
                            }
                        }
                    }
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_restore_ip_addresses() {
        assert_eq!(
            Solution::restore_ip_addresses("25525511135".to_owned()),
            vec!["255.255.11.135".to_owned(), "255.255.111.35".to_owned()],
        );
        assert_eq!(
            Solution::restore_ip_addresses("0000".to_owned()),
            vec!["0.0.0.0".to_owned()]
        );
        assert_eq!(
            Solution::restore_ip_addresses("1111".to_owned()),
            vec!["1.1.1.1".to_owned()]
        );
    }
}
