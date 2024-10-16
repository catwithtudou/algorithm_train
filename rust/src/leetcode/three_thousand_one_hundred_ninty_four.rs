pub struct Solution;


impl Solution {
    pub fn minimum_average(mut nums: Vec<i32>) -> f64 {
        nums.sort_unstable();
        (0..nums.len()/2).map(|i| nums[i]+nums[nums.len()-1-i]).min().unwrap() as f64 / 2.0
    }
}