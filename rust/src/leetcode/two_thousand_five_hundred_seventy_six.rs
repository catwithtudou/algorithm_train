pub struct Solution;

impl Solution {
    pub fn max_num_of_marked_indices(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut i = 0;
        for &j in &nums[(nums.len() + 1) / 2..] {
            if nums[i] * 2 <= j {
                i += 1;
            }
        }
        (i * 2) as _
    }
}
