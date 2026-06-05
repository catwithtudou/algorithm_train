pub struct Solution;

impl Solution {
    pub fn total_waviness(num1: i64, num2: i64) -> i64 {
        let low_s = num1.to_string();
        let high_s = num2.to_string();

        let low = low_s.as_bytes();
        let high = high_s.as_bytes();

        let n = high.len();
        let diff_lh = n - low.len();

        // memo[i][waviness][last_cmp + 1][last_digit]
        // last_cmp: -1, 0, 1，所以映射到 0, 1, 2
        let mut memo = vec![
            vec![[[0_i64; 10]; 3]; n.saturating_sub(1)];
            n
        ];

        fn dfs(
            i: usize,
            waviness: usize,
            last_cmp: i32,
            last_digit: usize,
            limit_low: bool,
            limit_high: bool,
            low: &[u8],
            high: &[u8],
            diff_lh: usize,
            memo: &mut Vec<Vec<[[i64; 10]; 3]>>,
        ) -> i64 {
            if i == high.len() {
                return waviness as i64;
            }

            if !limit_low && !limit_high {
                let cmp_idx = (last_cmp + 1) as usize;
                let v = memo[i][waviness][cmp_idx][last_digit];

                if v > 0 {
                    return v - 1;
                }
            }

            let mut lo = 0usize;
            if limit_low && i >= diff_lh {
                lo = (low[i - diff_lh] - b'0') as usize;
            }

            let mut hi = 9usize;
            if limit_high {
                hi = (high[i] - b'0') as usize;
            }

            // 前面是否已经填过真正的数字
            let is_num = !limit_low || i > diff_lh;

            let mut res = 0_i64;

            for d in lo..=hi {
                let mut c = 0_i32;

                if is_num {
                    c = if d > last_digit {
                        1
                    } else if d < last_digit {
                        -1
                    } else {
                        0
                    };
                }

                let mut w = waviness;

                // 比较方向发生变化，说明形成了峰或谷
                if c * last_cmp < 0 {
                    w += 1;
                }

                res += dfs(
                    i + 1,
                    w,
                    c,
                    d,
                    limit_low && d == lo,
                    limit_high && d == hi,
                    low,
                    high,
                    diff_lh,
                    memo,
                );
            }

            if !limit_low && !limit_high {
                let cmp_idx = (last_cmp + 1) as usize;
                memo[i][waviness][cmp_idx][last_digit] = res + 1;
            }

            res
        }

        dfs(
            0,
            0,
            0,
            0,
            true,
            true,
            low,
            high,
            diff_lh,
            &mut memo,
        )
    }
}