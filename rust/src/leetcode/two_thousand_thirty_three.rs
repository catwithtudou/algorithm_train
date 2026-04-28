pub struct Solution;

impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let k = grid.len() * grid[0].len();
        let mut a = Vec::with_capacity(k);
        let target = grid[0][0] % x;

        for row in grid {
            for v in row {
                if v % x != target {
                    return -1;
                }
                a.push(v);
            }
        }

        let median = *a.select_nth_unstable(k / 2).1;

        a.into_iter().map(|v| (v - median).abs()).sum::<i32>() / x
    }
}
