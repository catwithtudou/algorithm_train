pub struct Solution;


impl Solution {
    pub fn find_the_longest_balanced_substring(s: String) -> i32 {
        let mut ans: i32 = 0;
        let mut left: i32 = 0;
        let mut right: i32 = 0;

        for i in 0..s.len() {
            if s.as_bytes()[i] == b'0' {
                left += 1;
            } else {
                right += 1;
            }
            if left == right {
                ans = ans.max(left * 2);
                left = 0;
                right = 0;
            } else if left < right {
                left = 0;
                right = 0;
            } else if left > right && right > 0 {
                ans = ans.max(right * 2);
                if i + 1 < s.len() && s.as_bytes()[i + 1] == b'0' {
                    left = 0;
                    right = 0;
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod two_thousand_six_hundred_and_nine_test {
    use super::*;

    #[test]
    fn two_thousand_six_hundred_and_nine() {
        assert_eq!(Solution::find_the_longest_balanced_substring(String::from("01000111")), 6);
    }
}