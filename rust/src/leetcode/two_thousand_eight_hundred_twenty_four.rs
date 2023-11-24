pub struct Solution;

impl Solution {
    pub fn count_pairs(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let mut ans = 0;
        for i in 1..nums.len() {
            ans += nums[0..i].binary_search(&(target - nums[i])).unwrap_or_else(|x| x+1);
        }
        ans as i32
    }
}