pub struct Solution;

impl Solution {
    const DIRS_STRE: [[[i32; 2]; 2]; 7] = [
        [[0, 0], [0, 0]],
        [[0, -1], [0, 1]],  // 街道 1：左、右
        [[-1, 0], [1, 0]],  // 街道 2：上、下
        [[0, -1], [1, 0]],  // 街道 3：左、下
        [[0, 1], [1, 0]],   // 街道 4：右、下
        [[0, -1], [-1, 0]], // 街道 5：左、上
        [[0, 1], [-1, 0]],  // 街道 6：右、上
    ];

    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let mut visited = vec![vec![false; n]; m];

        fn contains_street(street: i32, dir: [i32; 2]) -> bool {
            let dirs = Solution::DIRS_STRE[street as usize];
            dirs[0] == dir || dirs[1] == dir
        }

        fn dfs(
            x: usize,
            y: usize,
            grid: &[Vec<i32>],
            visited: &mut [Vec<bool>],
        ) -> bool {
            let m = grid.len();
            let n = grid[0].len();

            if x == m - 1 && y == n - 1 {
                return true;
            }

            visited[x][y] = true;

            for [dx, dy] in Solution::DIRS_STRE[grid[x][y] as usize] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx < 0 || nx >= m as i32 || ny < 0 || ny >= n as i32 {
                    continue;
                }

                let nx = nx as usize;
                let ny = ny as usize;

                if visited[nx][ny] {
                    continue;
                }

                if !contains_street(grid[nx][ny], [-dx, -dy]) {
                    continue;
                }

                if dfs(nx, ny, grid, visited) {
                    return true;
                }
            }

            false
        }

        dfs(0, 0, &grid, &mut visited)
    }
}