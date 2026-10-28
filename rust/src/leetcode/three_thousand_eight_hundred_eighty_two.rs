pub struct Solution;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        let or_all = grid.iter().flatten().fold(0, |acc, &x| acc | x) as usize;

        // vis[i][j][xor] 压平成一维
        let w = or_all + 1;
        let mut vis = vec![false; m * n * w];
        let mut ans = i32::MAX;

        fn dfs(
            i: isize, j: isize, mut xor: i32,
            grid: &Vec<Vec<i32>>, vis: &mut Vec<bool>,
            n: usize, w: usize, ans: &mut i32,
        ) {
            if *ans == 0 || i < 0 || j < 0 {
                return;
            }
            let (iu, ju) = (i as usize, j as usize);
            let idx = (iu * n + ju) * w + xor as usize;
            if vis[idx] {
                return;
            }
            vis[idx] = true;

            xor ^= grid[iu][ju];

            if i == 0 && j == 0 {
                *ans = (*ans).min(xor);
                return;
            }

            dfs(i - 1, j, xor, grid, vis, n, w, ans);
            dfs(i, j - 1, xor, grid, vis, n, w, ans);
        }

        dfs(m as isize - 1, n as isize - 1, 0, &grid, &mut vis, n, w, &mut ans);
        ans
    }
}