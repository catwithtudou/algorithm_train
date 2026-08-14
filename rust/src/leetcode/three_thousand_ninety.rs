pub struct Solution;

impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let bytes = s.as_bytes();

        let mut cnt = [0_i32; 26];
        let mut left = 0usize;
        let mut ans = 0usize;

        for (right, &b) in bytes.iter().enumerate() {
            let idx = (b - b'a') as usize;
            cnt[idx] += 1;

            while cnt[idx] > 2 {
                let left_idx = (bytes[left] - b'a') as usize;
                cnt[left_idx] -= 1;
                left += 1;
            }

            ans = ans.max(right - left + 1);
        }

        ans as i32
    }
}