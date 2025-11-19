pub struct Solution;

impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        let mut mask = 0;

        for x in nums {
            let k = x / original;
            if x % original == 0 && (k & (k - 1)) == 0 {
                mask |= k;
            }
        }

        mask = !mask;

        original * (mask & (-mask))
    }
}
