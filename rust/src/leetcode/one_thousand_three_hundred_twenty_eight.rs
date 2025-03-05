pub struct Solution;

impl Solution {
    pub fn break_palindrome(mut palindrome: String) -> String {
        let n = palindrome.len();
        if n <= 1 {
            return "".to_string();
        }

        let s = unsafe { palindrome.as_bytes_mut() };
        for i in 0..n / 2 {
            if s[i] != b'a' {
                s[i] = b'a';
                return palindrome;
            }
        }
        s[n - 1] = b'b';
        palindrome
    }
}
