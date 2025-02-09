pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut stack_size = 2;
        for i in 2..nums.len() {
            if nums[i] != nums[stack_size - 2] {
                nums[stack_size] = nums[i];
                stack_size += 1;
            }
        }
        stack_size.min(nums.len()) as _
    }
}
