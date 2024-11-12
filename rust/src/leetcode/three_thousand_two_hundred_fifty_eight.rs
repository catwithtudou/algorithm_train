pub struct Solution;

impl Solution {
    pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
        let mut cnt = vec![0; 2];
        let s = s.as_bytes();
        let (mut left, mut ans) = (0, 0);
        for i in 0..s.len() {
            cnt[(s[i] - b'0') as usize] += 1;
            while cnt[0] > k && cnt[1] > k {
                cnt[(s[left] - b'0') as usize] -= 1;
                left += 1;
            }
            ans += i - left + 1;
        }

        return ans as _;
    }
}
