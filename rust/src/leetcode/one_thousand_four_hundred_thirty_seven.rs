pub struct Solution;

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut last1 = -k - 1;
        for (i, x) in nums.into_iter().enumerate() {
            if x != 1 {
                continue;
            }
            if i as i32 - last1 <= k {
                return false;
            }
            last1 = i as i32;
        }
        true
    }
}
