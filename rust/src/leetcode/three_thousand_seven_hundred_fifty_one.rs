pub struct Solution;

impl Solution {
    pub fn total_waviness(num1: i32, num2: i32) -> i32 {
        fn get_common_wave(x: i32) -> i32 {
            let s = x.to_string();
            let bytes = s.as_bytes();

            if bytes.len() < 3 {
                return 0;
            }

            let mut wave = 0;
            for i in 1..bytes.len() - 1 {
                let is_peak = bytes[i] > bytes[i - 1] && bytes[i] > bytes[i + 1];
                let is_bottom = bytes[i] < bytes[i - 1] && bytes[i] < bytes[i + 1];

                if is_peak || is_bottom {
                    wave += 1;
                }
            }

            wave
        }

        let mut ans = 0;
        for i in num1..=num2 {
            ans += get_common_wave(i);
        }

        ans
    }
}