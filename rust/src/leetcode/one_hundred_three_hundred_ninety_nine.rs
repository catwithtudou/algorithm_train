use std::cmp::min;

pub struct Solution;



impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let s = n.to_string();
        let m = s.len();
        let s_bytes = s.as_bytes();

        let mut memo = vec![vec![-1; m * 9 + 1]; m];

        fn dfs(
            i: usize,
            left: usize,
            limit_high: bool,
            m: usize,
            s_bytes: &[u8],
            memo: &mut Vec<Vec<i32>>,
        ) -> i32 {
            if i == m {
                return if left == 0 { 1 } else { 0 };
            }

            if !limit_high && memo[i][left] != -1 {
                return memo[i][left];
            }

            let hi = if limit_high {( s_bytes[i] - b'0') as usize } else { 9 };

            let mut res = 0;
            for d in 0..=min(hi, left) {
                res+=dfs(i+1,left-d,limit_high&&d==hi,m,s_bytes,memo);
            }

            if !limit_high {
                memo[i][left] = res;
            }

            res
        }

        let mut max_cnt = 0;
        let mut ans = 0;

        for target in 1..=m * 9 {
            let cnt = dfs(0, target, true, m, s_bytes, &mut memo);
            if cnt > max_cnt {
                max_cnt = cnt;
                ans = 1;
            } else if cnt == max_cnt {
                ans += 1;
            }
        }

        ans
    }
}
