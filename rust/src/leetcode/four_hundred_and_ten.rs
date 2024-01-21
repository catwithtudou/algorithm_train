pub struct Solution;


impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let (mut sum, mut max_num) = nums.iter()
            .fold((0, 0), |(sum, max_num), &x| (sum + x, max_num.max(x)));

        let (mut left, mut right) = (max_num, sum);
        while left < right {
            let mid = left + (right - left) / 2;
            let split_num = Solution::split_num(&nums, mid);
            if split_num > k {
                left = mid + 1;
            } else {
                right = mid;
            }
        }


        left
    }

    pub fn split_num(nums: &Vec<i32>, max_inter_num: i32) -> i32 {
        let mut cur_num = 0;
        let mut split = 1;

        for i in 0..nums.len() {
            if (cur_num + nums[i]) > max_inter_num {
                split += 1;
                cur_num = 0;
            }
            cur_num += nums[i];
        }
        split
    }
}