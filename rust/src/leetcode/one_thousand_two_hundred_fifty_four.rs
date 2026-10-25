pub struct Solution;

impl Solution {
    pub fn closed_island(mut grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(grid: &mut [Vec<i32>], x: usize, y: usize) {
            let m = grid.len();
            let n = grid[0].len();

            if grid[x][y] != 0 {
                return;
            }

            let mut stack = vec![(x, y)];

            while let Some((x, y)) = stack.pop() {
                if grid[x][y] != 0 {
                    continue;
                }

                grid[x][y] = 1;

                if x > 0 {
                    stack.push((x - 1, y));
                }
                if x + 1 < m {
                    stack.push((x + 1, y));
                }
                if y > 0 {
                    stack.push((x, y - 1));
                }
                if y + 1 < n {
                    stack.push((x, y + 1));
                }
            }
        }

        let m = grid.len();
        let n = grid[0].len();

        if m < 3 || n < 3 {
            return 0;
        }

        // 淹没所有与边界相连的陆地
        for i in 0..m {
            dfs(&mut grid, i, 0);
            dfs(&mut grid, i, n - 1);
        }

        for j in 0..n {
            dfs(&mut grid, 0, j);
            dfs(&mut grid, m - 1, j);
        }

        // 统计剩余的封闭岛屿
        let mut ans = 0;

        for i in 1..m - 1 {
            for j in 1..n - 1 {
                if grid[i][j] == 0 {
                    ans += 1;
                    dfs(&mut grid, i, j);
                }
            }
        }

        ans
    }
}