pub struct Solution;

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let mut ans = 0;

        let mut max_or = 0;

        for &v in nums.iter() {
            max_or |= v;
        }

        pub fn dfs(i: usize, or: i32, nums: &Vec<i32>, max_or: i32, ans: &mut i32) {
            if i == nums.len() {
                if or == max_or {
                    *ans += 1;
                }
                return;
            }
            dfs(i + 1, or | nums[i], nums, max_or, ans);
            dfs(i + 1, or, nums, max_or, ans);
        }

        dfs(0, 0, &nums, max_or, &mut ans);

        ans
    }
}
