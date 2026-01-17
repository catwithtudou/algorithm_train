pub struct Solution;

impl Solution {
    pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
        let mut max_side = 0;

        for i in 0..bottom_left.len() {
            let b1 = &bottom_left[i];
            let t1 = &top_right[i];
            if t1[0] - b1[0] <= max_side || t1[1] - b1[1] <= max_side {
                continue;
            }
            for j in 0..i {
                let b2 = &bottom_left[j];
                let t2 = &top_right[j];
                let width = (t1[0].min(t2[0])) - (b1[0].max(b2[0]));
                let height = (t1[1].min(t2[1])) - (b1[1].max(b2[1]));
                max_side = max_side.max(width.min(height));
            }
        }

        (max_side as i64) * (max_side as i64)
    }
}
