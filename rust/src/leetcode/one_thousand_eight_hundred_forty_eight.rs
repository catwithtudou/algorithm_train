pub struct Solution;

impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        let mut ans = i32::MAX;

        for (i, &v) in nums.iter().enumerate() {
            if v == target {
                ans = ans.min((i as i32 - start).abs());
            }
        }

        ans
    }
}