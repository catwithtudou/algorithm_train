pub struct Solution;

impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mx = *nums.iter().max().unwrap();
        let mut has = vec![false; (mx + 1) as usize];
        let mut ans = 0;
        let mut left = 0;
        let mut s = 0;
        for &x in &nums {
            while has[x as usize] {
                has[nums[left] as usize] = false;
                s -= nums[left];
                left += 1;
            }
            has[x as usize] = true;
            s += x;
            ans = ans.max(s);
        }

        ans
    }
}
