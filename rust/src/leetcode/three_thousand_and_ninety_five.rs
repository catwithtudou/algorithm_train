pub struct Solution;

impl Solution {
    pub fn minimum_subarray_length(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = usize::MAX;
        let mut bottom = 0;
        let mut left = 0;
        let mut right_or = 0;
        for right in 0..nums.len() {
            right_or |= nums[right];
            while left <= right && (nums[left] | right_or) >= k {
                ans = ans.min(right - left + 1);
                left += 1;
                if bottom < left {
                    for i in (left..right).rev() {
                        nums[i] |= nums[i + 1];
                    }
                    bottom = right;
                    right_or = 0;
                }
            }
        }

        if ans == usize::MAX {
            return -1;
        } else {
            ans as _
        }
    }
}
