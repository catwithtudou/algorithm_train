pub struct Solution;

impl Solution {
    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let m = m as usize;
        let n = n as usize;

        // 计算3的幂
        let mut pow3: Vec<i32> = vec![1; m];
        for i in 1..m {
            pow3[i] = pow3[i - 1] * 3;
        }

        // 找出所有有效的列着色方案
        let mut valid = Vec::new();
        'next: for color in 0..pow3[m - 1] * 3 {
            for i in 0..m - 1 {
                if color / pow3[i + 1] % 3 == color / pow3[i] % 3 {
                    // 相邻颜色相同
                    continue 'next;
                }
            }
            valid.push(color);
        }

        let nv = valid.len();

        // 计算相邻列之间的有效过渡
        let mut nxt = vec![Vec::new(); nv];
        for (i, &color1) in valid.iter().enumerate() {
            'next2: for (j, &color2) in valid.iter().enumerate() {
                for &p3 in &pow3 {
                    if color1 / p3 % 3 == color2 / p3 % 3 {
                        // 相邻颜色相同
                        continue 'next2;
                    }
                }
                nxt[i].push(j);
            }
        }

        // 动态规划求解
        let mut f = vec![vec![0; nv]; n];
        for j in 0..nv {
            f[0][j] = 1;
        }

        for i in 1..n {
            for j in 0..nv {
                for &k in &nxt[j] {
                    f[i][j] = (f[i][j] + f[i - 1][k]) % MOD;
                }
            }
        }

        // 计算总方案数
        let mut ans = 0;
        for &fv in &f[n - 1] {
            ans = (ans + fv) % MOD;
        }

        ans
    }
}
