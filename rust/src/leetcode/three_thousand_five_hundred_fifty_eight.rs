pub struct Solution;

impl Solution {
    pub fn assign_edge_weights(edges: Vec<Vec<i32>>) -> i32 {
        let n = edges.len() + 1;

        let mut g = vec![Vec::<usize>::new(); n + 1];

        for e in edges {
            let x = e[0] as usize;
            let y = e[1] as usize;

            g[x].push(y);
            g[y].push(x);
        }

        fn dfs(x: usize, fa: usize, g: &Vec<Vec<usize>>) -> i32 {
            let mut d = 0;

            for &y in &g[x] {
                if y != fa {
                    d = d.max(dfs(y, x, g) + 1);
                }
            }

            d
        }

        fn pow_common(mut x: i64, mut n: i32) -> i32 {
            const MOD: i64 = 1_000_000_007;

            let mut res = 1_i64;

            while n > 0 {
                if n % 2 > 0 {
                    res = res * x % MOD;
                }

                x = x * x % MOD;
                n /= 2;
            }

            res as i32
        }

        let k = dfs(1, 0, &g);

        pow_common(2, k - 1)
    }
}