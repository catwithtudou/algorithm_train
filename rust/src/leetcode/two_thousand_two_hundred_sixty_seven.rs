pub struct Solution;

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();

        // 路径长度为 m + n - 1，合法括号串长度必须为偶数
        if (m + n) % 2 == 0
            || grid[0][0] == ')'
            || grid[m - 1][n - 1] == '('
        {
            return false;
        }

        let max_balance = (m + n + 1) / 2;
        let mut visited = vec![vec![vec![false; max_balance]; n]; m];

        fn dfs(
            grid: &[Vec<char>],
            visited: &mut [Vec<Vec<bool>>],
            x: usize,
            y: usize,
            balance: i32,
        ) -> bool {
            let m = grid.len();
            let n = grid[0].len();

            // 先处理当前格子
            let balance = match grid[x][y] {
                '(' => balance + 1,
                ')' => balance - 1,
                _ => unreachable!(),
            };

            // 任何前缀都不能出现右括号更多的情况
            if balance < 0 {
                return false;
            }

            // 还剩多少个格子可以用于消掉当前的左括号
            let remaining = (m - 1 - x) + (n - 1 - y);

            if balance as usize > remaining {
                return false;
            }

            if x == m - 1 && y == n - 1 {
                return balance == 0;
            }

            let balance = balance as usize;

            if visited[x][y][balance] {
                return false;
            }
            visited[x][y][balance] = true;

            (x + 1 < m && dfs(grid, visited, x + 1, y, balance as i32))
                || (y + 1 < n && dfs(grid, visited, x, y + 1, balance as i32))
        }

        dfs(&grid, &mut visited, 0, 0, 0)
    }
}