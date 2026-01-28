pub struct Solution;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut points = vec![];
        for i in 0..m {
            for j in 0..n {
                points.push((i, j));
            }
        }
        points.sort_by_key(|&(i, j)| grid[i][j]);
        let mut costs = vec![vec![i32::MAX; n]; m];
        for _ in 0..=k {
            let mut min_cost = i32::MAX;
            let mut i = 0;
            let mut j = 0;
            while i < points.len() {
                min_cost = min_cost.min(costs[points[i].0][points[i].1]);
                if i + 1 < points.len()
                    && grid[points[i].0][points[i].1] == grid[points[i + 1].0][points[i + 1].1]
                {
                    i += 1;
                    continue;
                }
                for r in j..=i {
                    let p = points[r];
                    costs[p.0][p.1] = min_cost;
                }
                j = i + 1;
                i += 1;
            }
            for i in (0..m).rev() {
                for j in (0..n).rev() {
                    if i == m - 1 && j == n - 1 {
                        costs[i][j] = 0;
                        continue;
                    }
                    if i != m - 1 {
                        costs[i][j] = costs[i][j].min(costs[i + 1][j] + grid[i + 1][j]);
                    }
                    if j != n - 1 {
                        costs[i][j] = costs[i][j].min(costs[i][j + 1] + grid[i][j + 1]);
                    }
                }
            }
        }
        costs[0][0]
    }
}
