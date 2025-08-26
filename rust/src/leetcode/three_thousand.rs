pub struct Solution;

impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        let mut mx = (0, 0);
        for d in dimensions {
            let x = d[0];
            let y = d[1];
            mx = mx.max((x * x + y * y, x * y))
        }
        mx.1
    }
}
