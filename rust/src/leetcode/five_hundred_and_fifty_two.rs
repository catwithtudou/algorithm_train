pub struct Solution;

const MOD: i64 = 1_000_000_007;
const MX: usize = 100_001;

impl Solution {
    pub fn check_record(n: i32) -> i32 {
        let mut f = vec![vec![vec![0 as i64; 3]; 2]; MX];
        f[0][0] = [1, 1, 1].to_vec();
        f[0][1] = [1, 1, 1].to_vec();
        for i in 1..MX {
            for j in 0..2 {
                for k in 0..3 {
                    let mut res = f[i - 1][j][0];
                    if j == 0 {
                        res += f[i - 1][1][0];
                    }
                    if k < 2 {
                        res += f[i - 1][j][k + 1];
                    }
                    f[i][j][k] = res % MOD;
                }
            }
        }

        f[n as usize][0][0] as i32
    }
}
