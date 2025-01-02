pub struct Solution;

impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        let mut cnt = [0; 3];
        for c in s.bytes() {
            cnt[(c - b'a') as usize] += 1;
        }

        if cnt[0] < k || cnt[1] < k || cnt[2] < k {
            return -1;
        }

        let (mut mx, mut left) = (0, 0);
        let s = s.as_bytes();
        for (right, &c) in s.iter().enumerate() {
            let c = (c - b'a') as usize;
            cnt[c] -= 1;
            while cnt[c] < k {
                cnt[(s[left] - b'a') as usize] += 1;
                left += 1;
            }
            mx = mx.max(right - left + 1);
        }

        (s.len() - mx) as _
    }
}
