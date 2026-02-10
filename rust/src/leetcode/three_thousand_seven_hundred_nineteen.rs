pub struct Solution;

impl Solution {
    pub fn longest_balanced(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = 0;
        for i in 0..n-1 {
            let mut vis = std::collections::HashMap::new();
            let mut cnt = [0, 0];
            for j in i..n {
                if !vis.contains_key(&nums[j]) {
                    vis.insert(nums[j], true);
                    cnt[nums[j] as usize & 1] += 1;
                }
                if cnt[0] == cnt[1] {
                    ans = ans.max(j - i + 1);
                }
            }
        }
        ans as _
    }
}