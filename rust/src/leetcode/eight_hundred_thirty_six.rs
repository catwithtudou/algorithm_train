pub struct Solution;

impl Solution {
    fn is_interval_overlap(l1: i32, r1: i32, l2: i32, r2: i32) -> bool {
        l1.max(l2) < r1.min(r2)
    }

    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        Self::is_interval_overlap(rec1[0], rec1[2], rec2[0], rec2[2]) &&
        Self::is_interval_overlap(rec1[1], rec1[3], rec2[1], rec2[3])
    }

}