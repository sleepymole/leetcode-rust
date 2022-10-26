#![allow(dead_code)]
pub struct Solution;

impl Solution {
    fn valid_ipv4(ip: &Vec<char>) -> bool {
        let fields: Vec<&[char]> = ip.split(|&c| c == '.').collect();
        if fields.len() != 4 {
            return false;
        }
        for field in fields {
            if field.is_empty() || field.len() > 1 && field[0] == '0' || field.len() > 3 {
                return false;
            }
            let mut x = 0;
            for &c in field {
                x = x * 10 + c as i32 - '0' as i32;
            }
            if x >= 256 {
                return false;
            }
        }
        true
    }

    fn valid_ipv6(ip: &Vec<char>) -> bool {
        let fields: Vec<&[char]> = ip.split(|&c| c == ':').collect();
        if fields.len() != 8 {
            return false;
        }
        for field in fields {
            if field.is_empty() || field.len() > 4 {
                return false;
            }
        }
        true
    }

    pub fn valid_ip_address(ip: String) -> String {
        let ip: Vec<char> = ip.chars().collect();
        if ip.iter().all(|&c| c.is_ascii_digit() || c == '.') && Solution::valid_ipv4(&ip) {
            return "IPv4".to_owned();
        } else if ip.iter().all(|&c| c.is_ascii_hexdigit() || c == ':') && Solution::valid_ipv6(&ip)
        {
            return "IPv6".to_owned();
        }
        "Neither".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ip_address() {
        assert_eq!(
            Solution::valid_ip_address("172.16.254.1".to_owned()),
            "IPv4".to_owned()
        );
        assert_eq!(
            Solution::valid_ip_address("2001:0db8:85a3:0:0:8A2E:0370:7334".to_owned()),
            "IPv6".to_owned()
        );
        assert_eq!(
            Solution::valid_ip_address("256.256.256.256".to_owned()),
            "Neither".to_owned()
        );
        assert_eq!(
            Solution::valid_ip_address("2001:0db8:85a3:0:0:8A2E:0370:7334:".to_owned()),
            "Neither".to_owned()
        );
        assert_eq!(
            Solution::valid_ip_address("1e1.4.5.6".to_owned()),
            "Neither".to_owned()
        );
    }
}
