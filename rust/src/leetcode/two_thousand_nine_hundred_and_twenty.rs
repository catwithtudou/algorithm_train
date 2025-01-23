pub struct Solution;

impl Solution {
    pub fn maximum_points(edges: Vec<Vec<i32>>, coins: Vec<i32>, k: i32) -> i32 {
        let n = coins.len();
        let mut g:Vec<Vec<usize>> = vec![Vec::new(); n];
        for e in edges {
            let (x, y) = (e[0] as usize, e[1] as usize);
            g[x].push(y);
            g[y].push(x);
        }
        pub fn dfs(x: usize, fa: i32, g: &Vec<Vec<usize>>, coins: &Vec<i32>, k: i32) -> [i32; 14] {
            let mut s = [0; 14];

            for &y in &g[x] {
                if y as i32 == fa {
                    continue;
                }
                let fy = dfs(y, x as i32, g, coins, k);
                for j in 0..14 {
                    s[j] += fy[j];
                }
            }

            for j in 0..13 {
                s[j] = std::cmp::max((coins[x] >> j) - k + s[j], (coins[x] >> (j + 1)) + s[j + 1]);
            }
            s[13] += (coins[x] >> 13) - k;

            s
        }
        dfs(0, -1, &g, &coins, k)[0]
    }
}
