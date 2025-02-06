pub struct Solution;

impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        fn dfs(
            i: usize,
            nums: &[i32],
            path: &mut Vec<i32>,
            vis: &mut Vec<bool>,
            ans: &mut Vec<Vec<i32>>,
        ) {
            if i == nums.len() {
                ans.push(path.clone());
                return;
            }

            for j in 0..nums.len() {
                if vis[j] {
                    continue;
                }
                if j > 0 && nums[j] == nums[j - 1] && !vis[j - 1] {
                    continue;
                }

                path[i] = nums[j];
                vis[j] = true;
                dfs(i + 1, nums, path, vis, ans);
                vis[j] = false;
            }
        }

        let n = nums.len();
        let mut ans = vec![];
        let (mut path, mut vis) = (vec![0; n], vec![false; n]);
        dfs(0, &mut nums, &mut path, &mut vis, &mut ans);
        ans
    }
}
