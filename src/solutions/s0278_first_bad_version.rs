#![allow(dead_code)]
#![allow(non_snake_case)]
pub struct Solution {
    bad_version: i32,
}

impl Solution {
    pub fn isBadVersion(&self, version: i32) -> bool {
        version >= self.bad_version
    }
}

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let (mut l, mut r) = (0, n);
        while l < r {
            let m = l + (r - l) / 2;
            if self.isBadVersion(m) {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_bad_version() {
        let s = Solution { bad_version: 4 };
        assert_eq!(s.first_bad_version(5), 4);
        let s = Solution { bad_version: 1 };
        assert_eq!(s.first_bad_version(1), 1);
    }
}
