pub struct Solution;

impl Solution {
    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        let m = grid.len();
        if m == 0 {
            return false;
        }

        let n = grid[0].len();
        if n == 0 {
            return false;
        }

        let mut visited = vec![vec![false; n]; m];

        fn dfs(
            x: usize,
            y: usize,
            parent: Option<(usize, usize)>,
            grid: &[Vec<char>],
            visited: &mut [Vec<bool>],
        ) -> bool {
            visited[x][y] = true;

            let dirs: [(isize, isize); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
            let m = grid.len() as isize;
            let n = grid[0].len() as isize;

            for (dx, dy) in dirs {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx < 0 || nx >= m || ny < 0 || ny >= n {
                    continue;
                }

                let nx = nx as usize;
                let ny = ny as usize;

                if Some((nx, ny)) == parent {
                    continue;
                }

                if grid[nx][ny] != grid[x][y] {
                    continue;
                }

                if visited[nx][ny] || dfs(nx, ny, Some((x, y)), grid, visited) {
                    return true;
                }
            }

            false
        }

        for i in 0..m {
            for j in 0..n {
                if !visited[i][j] && dfs(i, j, None, &grid, &mut visited) {
                    return true;
                }
            }
        }

        false
    }
}