pub struct Solution;

impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        // 将数字转换为字符串
        let low_s = low.to_string();
        let high_s = high.to_string();
        let n = high_s.len();
        let m = n / 2;
        let diff_lh = n - low_s.len();

        // 创建记忆化数组
        let mut memo = vec![vec![vec![-1; m * 18 + 1]; diff_lh + 1]; n];

        let low_bytes = low_s.as_bytes();
        let high_bytes = high_s.as_bytes();

        fn dfs(
            i: usize,
            start: i32,
            diff: i32,
            limit_low: bool,
            limit_high: bool,
            n: usize,
            m: usize,
            diff_lh: usize,
            memo: &mut Vec<Vec<Vec<i32>>>,
            low_bytes: &[u8],
            high_bytes: &[u8],
        ) -> i32 {
            if i == n {
                return if diff == 0 { 1 } else { 0 };
            }

            if start != -1 && !limit_low && !limit_high {
                let memo_idx = (diff + m as i32 * 9) as usize;
                if memo[i][start as usize][memo_idx] != -1 {
                    return memo[i][start as usize][memo_idx];
                }
            }

            let lo = if limit_low && i >= diff_lh {
                (low_bytes[i - diff_lh] - b'0') as i32
            } else {
                0
            };

            let hi = if limit_high {
                (high_bytes[i] - b'0') as i32
            } else {
                9
            };

            // 处理特殊情况：前面没有填数字且剩余位数为奇数
            if start < 0 && (n - i) % 2 > 0 {
                if lo > 0 {
                    return 0;
                }
                return dfs(
                    i + 1,
                    start,
                    diff,
                    true,
                    false,
                    n,
                    m,
                    diff_lh,
                    memo,
                    low_bytes,
                    high_bytes,
                );
            }

            let is_left = start < 0 || i < ((start as usize + n) / 2);
            let mut res = 0;

            // 枚举当前位可以填的数字
            for d in lo..=hi {
                let new_start = if start < 0 && d > 0 { i as i32 } else { start };

                let new_diff = if is_left { diff + d } else { diff - d };

                res += dfs(
                    i + 1,
                    new_start,
                    new_diff,
                    limit_low && d == lo,
                    limit_high && d == hi,
                    n,
                    m,
                    diff_lh,
                    memo,
                    low_bytes,
                    high_bytes,
                );
            }

             // 更新记忆化状态
             if start != -1 && !limit_low && !limit_high {
                let memo_idx = (diff + m as i32 * 9) as usize;
                memo[i][start as usize][memo_idx] = res;
            }

            res
        }

        dfs(
            0, -1, 0, true, true, n, m, diff_lh, &mut memo, low_bytes, high_bytes,
        )
    }
}
