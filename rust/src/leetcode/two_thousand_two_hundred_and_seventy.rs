pub struct Solution;

impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let mut total = nums.iter().map(|&x| x as i64).sum();
        let mut ans = 0;
        let mut s = 0;
        for i in 0..nums.len() - 1 {
            s += nums[i] as i64;
            if s * 2 >= total {
                ans += 1;
            }
        }
        ans
    }
}
