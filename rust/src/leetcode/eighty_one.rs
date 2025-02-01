pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let check = |i: usize, right: usize| -> bool {
            let x = nums[i];
            if x > nums[right] {
                return target > nums[right] && x >= target;
            }
            target > nums[right] || x >= target
        };

        let (mut left, mut right) = (0, nums.len() - 1);
        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] == nums[right] {
                right -= 1;
            } else if check(mid, right) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        nums[right] == target
    }
}
