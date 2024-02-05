pub struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut result = nums.len() + 1;
        let mut start = 0;
        let mut cur = 0;
        for end in 0..nums.len() {
            cur += nums[end];
            while cur >= target {
                result = result.min(end - start + 1);
                cur -= nums[start];
                start += 1;
            }
        }

        if result == nums.len() + 1 {
            return 0;
        }
        return result as i32;
    }
}