pub struct Solution;

impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let zero = zero as usize;
        let one = one as usize;
        let limit = limit as usize;

        let mut memo = vec![vec![[-1i64; 2]; one + 1]; zero + 1];

        fn dfs(i: usize, j: usize, k: usize, limit: usize, memo: &mut Vec<Vec<[i64; 2]>>) -> i64 {
            const MOD: i64 = 1_000_000_007;

            if i == 0 {
                return if k == 1 && j <= limit { 1 } else { 0 };
            }

            if j == 0 {
                return if k == 0 && i <= limit { 1 } else { 0 };
            }

            if memo[i][j][k] != -1 {
                return memo[i][j][k];
            }

            let res;
            if k == 0 {
                res = {
                    let mut r =
                        (dfs(i - 1, j, 0, limit, memo) + dfs(i - 1, j, 1, limit, memo)) % MOD;
                    if i > limit {
                        r = (r - dfs(i - limit - 1, j, 1, limit, memo) + MOD) % MOD;
                    }
                    r
                };
            } else {
                res = {
                    let mut r =
                        (dfs(i, j - 1, 0, limit, memo) + dfs(i, j - 1, 1, limit, memo)) % MOD;
                    if j > limit {
                        r = (r - dfs(i, j - limit - 1, 0, limit, memo) + MOD) % MOD;
                    }
                    r
                };
            }

            memo[i][j][k] = res;
            res
        }

        let ans = (dfs(zero, one, 0, limit, &mut memo) + dfs(zero, one, 1, limit, &mut memo)) % MOD;

        ans as i32
    }
}
