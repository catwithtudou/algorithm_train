pub struct Solution;

impl Solution {
    pub fn sum_digit_differences( nums: Vec<i32>) -> i64 {
        let mut res: i64 = 0;
        let mut cnt = vec![vec![0; 10]; nums[0].to_string().len()];
        for (k, &n) in nums.iter().enumerate() {
            let mut i = 0;
            let mut n_temp = n;
            while n_temp > 0 {
                let d = n_temp % 10;
                res += (k as i32 - cnt[i][d as usize]) as i64;
                cnt[i][d as usize] += 1;
                n_temp /= 10;
                i += 1;
            }
        }
        res
    }
}
