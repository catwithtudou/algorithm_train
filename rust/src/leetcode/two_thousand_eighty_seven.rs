pub struct Solution;

impl Solution {
    pub fn min_cost(start_pos: Vec<i32>, home_pos: Vec<i32>, row_costs: Vec<i32>, col_costs: Vec<i32>) -> i32 {
        let x0 = start_pos[0] as usize;
        let y0 = start_pos[1] as usize;
        let x1 = home_pos[0] as usize;
        let y1 = home_pos[1] as usize;

        let mut ans = -row_costs[x0] - col_costs[y0];

        ans += row_costs[x0.min(x1)..=x0.max(x1)].iter().sum::<i32>();
        ans += col_costs[y0.min(y1)..=y0.max(y1)].iter().sum::<i32>();

        ans
    }
}

