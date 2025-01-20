pub struct Solution;

impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        *nums.iter().min_by_key(|&x| (x.abs(), x)).unwrap()
    }
}
