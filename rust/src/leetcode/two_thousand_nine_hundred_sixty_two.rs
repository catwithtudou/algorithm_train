pub struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let mx = nums.iter().max().unwrap();
        let mut cnt_mx = 0;
        let mut left = 0;
        let mut ans = 0;
        for (_, &x) in nums.iter().enumerate() {
            if x == *mx {
                cnt_mx += 1;
            }
            while cnt_mx == k {
                if nums[left] == *mx {
                    cnt_mx -= 1;
                }
                left += 1;
            }
            ans += left as i64;
        }
        ans
    }
}
