pub struct Solution;

impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        if nums[0] >= nums[1] {
            return false;
        }

        let mut cnt = 1;

        for i in 2..nums.len() {
           if nums[i-1] == nums[i] {
            return false;
           }
           if (nums[i-2]< nums[i-1]) != (nums[i-1] < nums[i]) {
            cnt += 1;
           }
        }

        cnt==3
    }
}