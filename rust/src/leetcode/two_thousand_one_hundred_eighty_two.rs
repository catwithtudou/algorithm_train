pub struct Solution;

impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut cnt = vec![0; 26];
        s.bytes().for_each(|c| {
            cnt[(c - b'a') as usize] += 1;
        });
        let mut ans = String::new();
        let (mut i, mut j) = (25, 24);
        let mut repeat = 0;
        while i < 26 && j < 25 {
            if cnt[i] == 0 {
                i -= 1;
                repeat = 0;
            } else if repeat < repeat_limit {
                cnt[i] -= 1;
                repeat += 1;
                ans.push((b'a' + i as u8) as char);
            } else if j >= i || cnt[j] == 0 {
                j -= 1;
            } else {
                cnt[j] -= 1;
                repeat = 0;
                ans.push((b'a' + j as u8) as char);
            }
        }
        ans
    }
}