pub struct Solution;

impl Solution {
    pub fn smallest_range_ii(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        let mut ans = nums[n-1]-nums[0];
        for i in 1..n {
            let mx = (nums[i-1]+k).max(nums[n-1]-k);
            let mn = (nums[i]-k).min(nums[0]+k);
            ans = ans.min(mx-mn);
        }
        return ans;
    }
}