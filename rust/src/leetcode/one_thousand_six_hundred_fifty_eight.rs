pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let target = nums.iter().map(|&x| x as i64).sum::<i64>() - x as i64;

        if target < 0 {
            return -1;
        }

        let mut left = 0;
        let mut sum = 0i64;
        let mut max_len = -1;

        for (right, &num) in nums.iter().enumerate() {
            sum += num as i64;

            while sum > target {
                sum -= nums[left] as i64;
                left += 1;
            }

            if sum == target {
                max_len = max_len.max((right - left + 1) as i32);
            }
        }

        if max_len < 0 {
            -1
        } else {
            nums.len() as i32 - max_len
        }
    }
}