pub struct Solution;

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let mut ans = 0;
        let mut cnt = [0, 0, 0];
        let mut left = 0;
        for i in 0..s.len() {
            cnt[(s[i] - b'a') as usize] += 1;
            while cnt[0] > 0 && cnt[1] > 0 && cnt[2] > 0 {
                cnt[(s[left] - b'a') as usize] -= 1;
                left += 1;
            }
            ans += left as i32;
        }
        ans
    }
}