pub struct Solution;

impl Solution {
    pub fn max_active_sections_after_trade(s: String) -> i32 {
        let bytes = s.as_bytes();

        let mut ans = 0_i32;
        let mut max_gain = 0_i32;
        let mut previous_zero_count = i32::MIN;
        let mut count = 0_i32;

        for i in 0..bytes.len() {
            count += 1;

            // 当前连续段结束
            if i + 1 == bytes.len() || bytes[i] != bytes[i + 1] {
                if bytes[i] == b'1' {
                    ans += count;
                } else {
                    max_gain = max_gain.max(previous_zero_count + count);
                    previous_zero_count = count;
                }

                count = 0;
            }
        }

        ans + max_gain
    }
}