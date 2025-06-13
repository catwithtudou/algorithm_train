pub struct Solution;

impl Solution {
    pub fn minimize_max(mut nums: Vec<i32>, p: i32) -> i32 {
        nums.sort();
        let n = nums.len();

        let (mut left, mut right) = (0, nums[n - 1] - nums[0]);

        while left < right {
            let mid = left + (right - left) / 2;
            let (mut cnt, mut i) = (0, 0);

            while i < n - 1 {
                if nums[i + 1] - nums[i] <= mid {
                    cnt += 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }

            if cnt >= p {
                right = mid
            } else {
                left = mid + 1
            }
        }

        left
    }
}
