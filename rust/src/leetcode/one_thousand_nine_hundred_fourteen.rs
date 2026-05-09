pub struct Solution;

impl Solution {
    pub fn rotate_grid(mut grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let dirs: [(isize, isize); 4] = [
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0),
        ];

        let m0 = grid.len();
        let n0 = grid[0].len();

        let mut a: Vec<i32> = Vec::with_capacity((m0 + n0 - 2) * 2);

        for i in 0..m0.min(n0) / 2 {
            let mut m = m0 - i * 2;
            let mut n = n0 - i * 2;

            let mut x = i as isize;
            let mut y = i as isize;

            a.clear();

            for &(dx, dy) in &dirs {
                for _ in 0..n - 1 {
                    a.push(grid[x as usize][y as usize]);
                    x += dx;
                    y += dy;
                }

                std::mem::swap(&mut m, &mut n);
            }

            let shift = (k as usize) % a.len();
            a.rotate_left(shift);

            let mut j = 0;

            for &(dx, dy) in &dirs {
                for _ in 0..n - 1 {
                    grid[x as usize][y as usize] = a[j];
                    j += 1;

                    x += dx;
                    y += dy;
                }

                std::mem::swap(&mut m, &mut n);
            }
        }

        grid
    }
}