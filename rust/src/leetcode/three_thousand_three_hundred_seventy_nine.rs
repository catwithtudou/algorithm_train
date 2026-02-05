pub struct Solution;

impl Solution {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let m = n as i32;
        let mut result = vec![0; n];
        for i in 0..n {
            let j = ((i as i32 + nums[i]) % m + m) % m; // 保证结果在 [0,n-1] 中
            result[i] = nums[j as usize];
        }
        result
    }
}
