pub struct Solution;

impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        fn dfs(i: usize, nums: &[i32], ans: &mut Vec<Vec<i32>>, path: &mut Vec<i32>) {
            ans.push(path.clone());

            for j in i..nums.len() {
                if j > i && nums[j] == nums[j - 1] {
                    continue;
                }
                path.push(nums[j]);
                dfs(j + 1, nums, ans, path);
                path.pop();
            }
        }

        let mut ans = vec![];
        let mut path = vec![];
        dfs(0, &nums, &mut ans, &mut path);
        ans
    }
}
