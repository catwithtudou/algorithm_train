pub struct Solution;

impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        target[0] + target.windows(2).map(|w| 0.max(w[1] - w[0])).sum::<i32>()
    }
}
