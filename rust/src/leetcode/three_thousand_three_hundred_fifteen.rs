pub struct Solution;

impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        nums.iter().map(|&p| {
            if p % 2 == 0 {
                return -1;
            }
            let p_long = p as i64;
            let a = (p_long & (p_long + 1)) | ((p_long & !(p_long + 1)) >> 1);
            if a < p_long && ((a | (a + 1)) == p_long) {
                a as i32
            } else {
                -1
            }
        }).collect()
    }
}