pub struct Solution;

impl Solution {
    // 记忆搜索
    pub fn min_path_cost(grid: Vec<Vec<i32>>, move_cost: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut memo = vec![vec![0; n]; m];
        fn dfs(i: usize, j: usize, memo: &mut Vec<Vec<i32>>, grid: &Vec<Vec<i32>>, move_cost: &Vec<Vec<i32>>) -> i32 {
            if i == grid.len() - 1 {
                return grid[i][j];
            }

            if memo[i][j] != 0 {
                return memo[i][j];
            }

            let mut res = i32::MAX;
            for (k, c) in move_cost[grid[i][j] as usize].iter().enumerate() {
                res = res.min(dfs(i + 1, k, memo, grid, move_cost) + *c);
            }
            res += grid[i][j];
            memo[i][j] = res;
            res
        }

        let mut ans = i32::MAX;
        for j in 0..n {
            ans = ans.min(dfs(0, j, &mut memo, &grid, &move_cost));
        }

        ans
    }

    // 堆循环
    pub fn min_path_cost_heap(grid: Vec<Vec<i32>>, move_cost: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut f = vec![vec![0; n]; m];
        f[m - 1] = grid[m - 1].clone();
        for i in (0..m - 1).rev() {
            for (j, &g) in grid[i].iter().enumerate() {
                f[i][j] = g + f[i + 1].iter()
                    .zip(move_cost[g as usize].iter())
                    .map(|(&v, &c)| v + c)
                    .min().unwrap();
            }
        }

        *f[0].iter().min().unwrap()
    }
}