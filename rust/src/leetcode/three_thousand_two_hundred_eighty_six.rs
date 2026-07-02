pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
        let m = grid.len();
        let n = grid[0].len();

        let dirs: [(isize, isize); 4] = [
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
        ];

        let mut dis = vec![vec![i32::MAX / 2; n]; m];

        dis[0][0] = grid[0][0];

        let mut q = VecDeque::new();
        q.push_back((0usize, 0usize));

        while let Some((i, j)) = q.pop_front() {
            for &(dx, dy) in &dirs {
                let x = i as isize + dx;
                let y = j as isize + dy;

                if x >= 0 && x < m as isize && y >= 0 && y < n as isize {
                    let x = x as usize;
                    let y = y as usize;

                    let cost = grid[x][y];

                    if dis[i][j] + cost < dis[x][y] {
                        dis[x][y] = dis[i][j] + cost;

                        if cost == 0 {
                            q.push_front((x, y));
                        } else {
                            q.push_back((x, y));
                        }
                    }
                }
            }
        }

        dis[m - 1][n - 1] < health
    }
}