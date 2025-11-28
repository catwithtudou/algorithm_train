pub struct Solution;

impl Solution {
    pub fn max_k_divisible_components(
        n: i32,
        edges: Vec<Vec<i32>>,
        values: Vec<i32>,
        k: i32,
    ) -> i32 {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for e in edges {
            g[e[0] as usize].push(e[1] as usize);
            g[e[1] as usize].push(e[0] as usize);
        }

        fn dfs(
            x: usize,
            fa: usize,
            g: &[Vec<usize>],
            values: &[i32],
            k: i64,
            ans: &mut i32,
        ) -> i64 {
            let mut s = values[x] as i64;
            for &y in &g[x] {
                if y != fa {
                    s += dfs(y, x, g, values, k, ans);
                }
            }
            if s % k == 0 {
                *ans += 1;
            }
            s
        }

        let mut ans = 0;
        dfs(0, 0, &g, &values, k as i64, &mut ans);
        ans
    }
}
