#![allow(dead_code)]
pub struct Solution;

impl Solution {
    fn add(a: &[u8], b: &[u8]) -> Vec<u8> {
        let mut c = Vec::new();
        let mut carry = 0;
        for i in 0..a.len().min(b.len()) {
            let x = a[a.len() - 1 - i] + b[b.len() - 1 - i] + carry;
            c.push(x % 10);
            carry = x / 10;
        }
        if a.len() > b.len() {
            for i in (0..a.len() - b.len()).rev() {
                let x = a[i] + carry;
                c.push(x % 10);
                carry = x / 10;
            }
        } else {
            for i in (0..b.len() - a.len()).rev() {
                let x = b[i] + carry;
                c.push(x % 10);
                carry = x / 10;
            }
        }
        if carry > 0 {
            c.push(carry);
        }
        c.reverse();
        c
    }

    pub fn is_additive_number(num: String) -> bool {
        let digits: Vec<u8> = num.chars().map(|c| c as u8 - b'0').collect();
        for i in 1..digits.len() {
            for j in i + 1..digits.len() {
                let mut a = &digits[0..i];
                let mut b = &digits[i..j];
                if a.len() > 1 && a[0] == 0 {
                    continue;
                }
                if b.len() > 1 && b[0] == 0 {
                    continue;
                }
                let mut off = j;
                while off < digits.len() {
                    let c = Solution::add(a, b);
                    if off + c.len() <= digits.len() && &digits[off..off + c.len()] == c.as_slice()
                    {
                        a = b;
                        b = &digits[off..off + c.len()];
                        off += c.len();
                    } else {
                        break;
                    }
                }
                if off == digits.len() {
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_additive_number() {
        assert_eq!(Solution::is_additive_number("112358".to_owned()), true);
        assert_eq!(Solution::is_additive_number("199100199".to_owned()), true);
    }
}
