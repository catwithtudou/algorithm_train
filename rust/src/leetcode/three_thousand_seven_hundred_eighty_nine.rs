pub struct Solution;

impl Solution {
    pub fn minimum_cost(cost1: i32, cost2: i32, cost_both: i32, need1: i32, need2: i32) -> i64 {
        let (c1, c2, cb) = (cost1 as i64, cost2 as i64, cost_both as i64);
        let (n1, n2) = (need1 as i64, need2 as i64);

        // 方案 1：分别购买
        let res1 = c1 * n1 + c2 * n2;

        // 方案 2：全部购买组合
        let res2 = cb * n1.max(n2);

        // 方案 3：组合购买 + 单独购买
        let res3 = cb * n1.min(n2)
            + if n1 > n2 {
                c1 * (n1 - n2)
            } else {
                c2 * (n2 - n1)
            };

        res1.min(res2).min(res3)
    }
}