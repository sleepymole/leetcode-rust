#![allow(dead_code)]
pub struct Solution {}

impl Solution {
    fn match_cdata(s: &[char]) -> Option<usize> {
        if s.len() < 9 {
            return None;
        }
        if s[0] != '<'
            || s[1] != '!'
            || s[2] != '['
            || s[3] != 'C'
            || s[4] != 'D'
            || s[5] != 'A'
            || s[6] != 'T'
            || s[7] != 'A'
            || s[8] != '['
        {
            return None;
        }
        let l = Solution::match_cdata_content(&s[9..]);
        if s.len() < l + 12 {
            return None;
        }
        if s[l + 9] != ']' || s[l + 10] != ']' || s[l + 11] != '>' {
            return None;
        }
        Some(l + 12)
    }

    fn match_cdata_content(s: &[char]) -> usize {
        let mut i = 0;
        while i + 3 <= s.len() {
            if s[i] == ']' && s[i + 1] == ']' && s[i + 2] == '>' {
                return i;
            }
            i += 1;
        }
        s.len()
    }

    fn match_content(s: &[char]) -> Option<usize> {
        let mut i = 0;
        while i < s.len() {
            if s[i] != '<' {
                i += 1;
                continue;
            }
            if i + 1 == s.len() {
                return None;
            }
            if s[i + 1] == '/' {
                if Solution::match_end_tag(&s[i..]).is_some() {
                    return Some(i);
                } else {
                    return None;
                }
            } else if s[i + 1] == '!' {
                if let Some(l) = Solution::match_cdata(&s[i..]) {
                    i += l;
                } else {
                    return None;
                }
            } else if Solution::match_start_tag(&s[i..]).is_some() {
                if let Some(l) = Solution::match_closed_tag(&s[i..]) {
                    i += l;
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
        None
    }

    fn match_start_tag(s: &[char]) -> Option<(String, usize)> {
        if s.len() < 2 || s[0] != '<' {
            return None;
        }
        let mut i = 1;
        while i < s.len() && s[i] != '>' {
            i += 1;
        }
        if i == 1 || i > 10 || i >= s.len() {
            return None;
        }
        if s[1..i].iter().any(|c| !c.is_ascii_uppercase()) {
            return None;
        }
        let name: String = s[1..i].iter().collect();
        Some((name, i + 1))
    }

    fn match_end_tag(s: &[char]) -> Option<(String, usize)> {
        if s.len() < 3 || s[0] != '<' || s[1] != '/' {
            return None;
        }
        let mut i = 2;
        while i < s.len() && s[i] != '>' {
            i += 1;
        }
        if i == 2 || i > 11 || i >= s.len() {
            return None;
        }
        if s[2..i].iter().any(|c| !c.is_ascii_uppercase()) {
            return None;
        }
        let name: String = s[2..i].iter().collect();
        Some((name, i + 1))
    }

    fn match_closed_tag(mut s: &[char]) -> Option<usize> {
        let mut matched = 0;
        let left = if let Some((name, l)) = Solution::match_start_tag(s) {
            matched += l;
            s = &s[l..];
            name
        } else {
            return None;
        };
        if let Some(l) = Solution::match_content(s) {
            matched += l;
            s = &s[l..];
        } else {
            return None;
        }
        let right = if let Some((name, l)) = Solution::match_end_tag(s) {
            matched += l;
            name
        } else {
            return None;
        };
        if left != right {
            return None;
        }
        Some(matched)
    }

    pub fn is_valid(code: String) -> bool {
        let s: Vec<char> = code.chars().collect();
        matches!(Solution::match_closed_tag(&s),Some(l)  if l == s.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        assert!(Solution::is_valid(
            "<DIV>This is the first line <![CDATA[<div>]]></DIV>".to_owned()
        ));
        assert!(Solution::is_valid(
            "<DIV>>>  ![cdata[]] <![CDATA[<div>]>]]>]]>>]</DIV>".to_owned()
        ));
        assert!(!Solution::is_valid("<A>  <B> </A>   </B>".to_owned()));
        assert!(!Solution::is_valid(
            "<DIV>  div tag is not closed  <DIV>".to_owned()
        ));
        assert!(!Solution::is_valid(
            "<DIV> illegal tag name  <b>123</b> 123456 </abcde>  </DIV>".to_owned()
        ));
        assert!(!Solution::is_valid("<DIV><></></DIV>".to_owned()));
        assert!(!Solution::is_valid("<AAAAAAAAAA></AAAAAAAAAA>".to_owned()));
    }
}
