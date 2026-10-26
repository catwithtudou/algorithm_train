pub struct Solution;

impl Solution {
    pub fn count_islands(mut grid: Vec<Vec<i32>>, k: i32) -> i32 {
        const DIRS: [(isize, isize); 4] = [
            (0, -1),
            (0, 1),
            (-1, 0),
            (1, 0),
        ];

        let m = grid.len();
        let n = grid[0].len();
        let mut ans = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 {
                    continue;
                }

                let mut sum = 0i64;
                let mut stack = vec![(i, j)];

                while let Some((x, y)) = stack.pop() {
                    if grid[x][y] == 0 {
                        continue;
                    }

                    sum += grid[x][y] as i64;
                    grid[x][y] = 0;

                    for (dx, dy) in DIRS {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;

                        if nx >= 0 && nx < m as isize
                            && ny >= 0 && ny < n as isize
                            && grid[nx as usize][ny as usize] > 0
                        {
                            stack.push((nx as usize, ny as usize));
                        }
                    }
                }

                if sum % k as i64 == 0 {
                    ans += 1;
                }
            }
        }

        ans
    }
}