pub struct Solution;

impl Solution {
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let check = |m: i32| -> bool {
            let mut cnt = 0;
            for i in 0..nums.len() {
                cnt += ((nums[i] - 1) / m) as i64;
            }
            cnt <= max_operations as i64
        };
        let mut left = 0;
        let mut right = *nums.iter().max().unwrap();
        while left + 1 < right {
            let mid = left + (right - left) / 2;
            if check(mid) {
                right = mid;
            } else {
                left = mid;
            }
        }
        right
    }
}
