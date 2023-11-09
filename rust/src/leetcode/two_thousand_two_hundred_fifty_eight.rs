pub struct Solution;

impl Solution {
    pub fn maximum_minutes(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let check = |mut t: i32| -> bool{
            let mut on_fire = vec![vec![false; n]; m];
            let mut f = Vec::new();
            for (i, row) in grid.iter().enumerate() {
                for (j, &x) in row.iter().enumerate() {
                    if x == 1 {
                        on_fire[i][j] = true;
                        f.push((i, j));
                    }
                }
            }

            while t > 0 && !f.is_empty() {
                let mut tmp = Vec::new();
                for &(i, j) in &f {
                    for &(x, y) in &[(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
                        if 0 <= x && x < m && 0 <= y && y < n {
                            if !on_fire[x][y] && grid[x][y] == 0 {
                                on_fire[x][y] = true;
                                tmp.push((x, y));
                            }
                        }
                    }
                }
                f = tmp;
                t -= 1
            }

            if on_fire[0][0] {
                return false;
            }

            let mut vis = vec![vec![false; n]; m];
            vis[0][0] = true;
            let mut q = vec![(0, 0)];
            while !q.is_empty() {
                let mut tmp = Vec::new();
                for &(i, j) in &q {
                    if on_fire[i][j] {
                        continue;
                    }

                    for &(x, y) in &[(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
                        if x >= 0 && x < m && y >= 0 && y < n {
                            if !on_fire[x][y] && !vis[x][y] && grid[x][y] == 0 {
                                if x == m - 1 && y == n - 1 {
                                    return true;
                                }
                                vis[x][y] = true;
                                tmp.push((x, y));
                            }
                        }
                    }
                }
                q = tmp;

                tmp = Vec::new();
                for &(i, j) in &f {
                    for &(x, y) in &[(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
                        if 0 <= x && x < m && 0 <= y && y < n {
                            if !on_fire[x][y] && grid[x][y] == 0 {
                                on_fire[x][y] = true;
                                tmp.push((x, y));
                            }
                        }
                    }
                }
                f = tmp;
            }
            false
        };

        let mut left = -1;
        let mut right = (m * n) as i32 + 1;
        while left + 1 < right {
            let mid = (left + right) / 2;
            if check(mid) {
                left = mid;
            } else {
                right = mid
            }
        }

        if left < (m * n) as i32 { left } else { 1_000_000_000 }
    }
}