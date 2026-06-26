pub struct Solution;

impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i32 {
        let mut ans = 0;

        for i in 0..nums.len() {
            let mut cnt = 0;
            for j in i..nums.len() {
                if nums[j] == target {
                    cnt += 1;
                } else {
                    cnt -= 1;
                }
                if cnt > 0 {
                    ans += 1;
                }
            }
        }

        ans
    }
}
