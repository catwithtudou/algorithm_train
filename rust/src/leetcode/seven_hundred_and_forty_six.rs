pub struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let (mut f0, mut f1) = (0, 0);
        for i in 1..cost.len() {
            let new_f = (f1 + cost[i]).min(f0 + cost[i - 1]);
            f0 = f1;
            f1 = new_f;
        }

        f1
    }
}