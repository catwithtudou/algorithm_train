pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn smallest_number(num: String, t: i64) -> String {
        fn gcd(mut a: i64, mut b: i64) -> i64 {
            while b != 0 {
                (a, b) = (b, a % b);
            }
            a
        }

        fn dfs(
            i: usize,
            t: i64,
            is_limit: bool,
            cnt: usize,
            num: &[u8],
            ans: &mut [u8],
            vis: &mut HashSet<(usize, i64)>,
        ) -> bool {
            if i == num.len() {
                return t == 1;
            }

            // 非贴着下界时做记忆化
            if !is_limit {
                let state = (i, t);

                if vis.contains(&state) {
                    return false;
                }

                vis.insert(state);
            }

            // 前面人为补的 0 可以直接跳过
            if is_limit && i < cnt {
                if dfs(i + 1, t, true, cnt, num, ans, vis) {
                    return true;
                }
            }

            let low = if is_limit {
                (num[i] - b'0') as i64
            } else {
                0
            };

            // 最终答案不能包含 0，所以数字从 max(low, 1) 开始
            for d in low.max(1)..=9 {
                let next_t = t / gcd(t, d);

                if dfs(
                    i + 1,
                    next_t,
                    is_limit && d == low,
                    cnt,
                    num,
                    ans,
                    vis,
                ) {
                    ans[i] = b'0' + d as u8;
                    return true;
                }
            }

            false
        }

        // t 必须只能由 2、3、5、7 组成
        // 因为十进制非零数字的质因数只可能来自这几个数
        let mut tmp = t;
        let mut factor_count = 0usize;

        for p in [2_i64, 3, 5, 7] {
            while tmp % p == 0 {
                tmp /= p;
                factor_count += 1;
            }
        }

        if tmp > 1 {
            return "-1".to_string();
        }

        // 多补一些前导 0，为构造更长的答案预留空间
        let cnt = (
            factor_count as isize
                - num.len() as isize
                + 1
        )
        .max(1) as usize;

        let mut padded = Vec::with_capacity(cnt + num.len());
        padded.extend(std::iter::repeat(b'0').take(cnt));
        padded.extend_from_slice(num.as_bytes());

        let n = padded.len();
        let mut ans = vec![b'0'; n];
        let mut vis = HashSet::new();

        dfs(
            0,
            t,
            true,
            cnt,
            &padded,
            &mut ans,
            &mut vis,
        );

        // 前面跳过的位置仍然是 '0'，把这些补位删掉
        let start = ans
            .iter()
            .rposition(|&c| c == b'0')
            .map_or(0, |i| i + 1);

        String::from_utf8(ans[start..].to_vec()).unwrap()
    }
}