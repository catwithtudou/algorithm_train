pub struct Solution;

impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort_unstable();
        let mut ans = 0;
        let (mut l, mut r) = (nums.len(), nums.len());

        for (j, &x) in nums.iter().enumerate() {
            while r > 0 && nums[r - 1] > upper - x {
                r -= 1;
            }

            while l > 0 && nums[l - 1] >= lower - x {
                l -= 1;
            }

            ans += r.min(j) - l.min(j);
        }

        ans as _
    }
}
