pub struct Solution;

impl Solution {
    pub fn min_changes(n: i32, k: i32) -> i32 {
        if n|k!=n {
            return -1;
        }
        (n^k).count_ones() as _
    }
}