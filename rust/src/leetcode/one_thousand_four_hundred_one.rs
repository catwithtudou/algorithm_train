pub struct Solution;

impl Solution {
    pub fn check_overlap(
        radius: i32,
        x_center: i32,
        y_center: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> bool {
        let x = x1.max(x_center.min(x2));
        let y = y1.max(y_center.min(y2));

        (x - x_center) * (x - x_center) + (y - y_center) * (y - y_center) <= radius * radius
    }
}
