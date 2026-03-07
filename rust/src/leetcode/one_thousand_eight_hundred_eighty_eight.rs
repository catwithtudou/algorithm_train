pub struct Solution;

impl Solution {
    pub fn min_flips(s: String) -> i32 {
        let n = s.len();
        let bytes = s.as_bytes();
        let mut ans = n as i32;
        let mut cnt: i32 = 0;

        for i in 0..(2 * n - 1) {
            if (bytes[i % n] % 2) as i32 != (i % 2) as i32 {
                cnt += 1;
            }
            let left = i as i32 - n as i32 + 1;
            if left < 0 {
                continue;
            }
            ans = ans.min(cnt.min(n as i32 - cnt));
            if (bytes[left as usize] % 2) as i32 != (left % 2) {
                cnt -= 1;
            }
        }
        ans
    }
}
