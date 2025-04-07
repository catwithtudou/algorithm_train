pub struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let s = nums.iter().sum::<i32>();
        if s % 2 != 0 {
            return false;
        }
        let n = nums.len();
        let mut memo = vec![vec![-1; s as usize / 2 + 1]; n];
        fn dfs(nums: &Vec<i32>, i: usize, j: usize, memo: &mut Vec<Vec<i32>>) -> bool {
            if i == 0 {
                return j == 0;
            }

            if memo[i][j] != -1 {
                return memo[i][j] == 1;
            }

            let res = j as i32 >= nums[i] && dfs(nums, i - 1, j - nums[i] as usize, memo)
                || dfs(nums, i - 1, j, memo);
            if res {
                memo[i][j] = 1;
            } else {
                memo[i][j] = 0;
            }
            res
        }
        dfs(&nums, n - 1, s as usize / 2, &mut memo)
    }
}
