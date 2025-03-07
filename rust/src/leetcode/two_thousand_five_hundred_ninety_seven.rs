pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = -1;
        let mut cnt: HashMap<i32, i32> = HashMap::new();

        fn dfs(i: usize, nums: &Vec<i32>, k: i32, cnt: &mut HashMap<i32, i32>, ans: &mut i32) {
            if i == nums.len() {
                *ans += 1;
                return;
            }

            // 不选
            dfs(i + 1, nums, k, cnt, ans);

            // 选
            let x = nums[i];
            if *cnt.get(&(x - k)).unwrap_or(&0) == 0 && *cnt.get(&(x + k)).unwrap_or(&0) == 0 {
                *cnt.entry(x).or_insert(0) += 1;
                dfs(i + 1, nums, k, cnt, ans);
                *cnt.get_mut(&x).unwrap() -= 1;
            }
        }

        dfs(0,&nums, k,&mut cnt,&mut ans);

        ans
    }
}
