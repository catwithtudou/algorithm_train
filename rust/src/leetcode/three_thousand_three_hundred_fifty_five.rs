pub struct Solution;

impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let mut diff = vec![0; nums.len() + 1];
        for q in queries {
            diff[q[0] as usize] += 1;
            diff[(q[1] + 1) as usize] -= 1;
        }
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += diff[i];
            if nums[i] > sum {
                return false;
            }
        }
        true
    }
}
