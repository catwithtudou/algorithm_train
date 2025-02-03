pub struct Solution;

impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        fn check(mut i: usize, mut j: usize, s: &[u8]) -> bool {
            while i < j {
                if s[i] != s[j] {
                    return false;
                }
                i += 1;
                j -= 1;
            }
            return true;
        }

        let s = s.as_bytes();
        let (mut i, mut j) = (0, s.len() - 1);
        while i < j {
            if s[i] != s[j] {
                return check(i + 1, j, s) || check(i, j - 1, s);
            }
            i += 1;
            j -= 1;
        }
        return true;
    }
}
