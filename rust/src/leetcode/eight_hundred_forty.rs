pub struct Solution;

impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut ans = 0;

        for i in 0..m.saturating_sub(2) {
            for j in 0..n.saturating_sub(2) {
                if grid[i + 1][j + 1] != 5 {
                    continue;
                }

                let mut mask = 0;
                let mut r_sum = [0; 3];
                let mut c_sum = [0; 3];

                for (r, row) in grid[i..i + 3].iter().enumerate() {
                    for (c, &x) in row[j..j + 3].iter().enumerate() {
                        mask |= 1 << x;
                        r_sum[r] += x;
                        c_sum[c] += x;
                    }
                }

                if mask != (1 << 10) - 2 {
                    continue;
                }

                if r_sum[0] == 15 && r_sum[1] == 15 && c_sum[0] == 15 && c_sum[1] == 15 {
                    ans += 1;
                }
            }
        }
        ans
    }
}
