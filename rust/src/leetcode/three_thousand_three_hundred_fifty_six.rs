pub struct Solution;

impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let q = queries.len();
        let n = nums.len();

        let mut left = 0;
        let mut right = q + 1;

        while left < right {
            let i = left + (right - left) / 2;
            let mut diff = vec![0; n + 1];
            for q in queries.iter().take(i) {
                diff[q[0] as usize] += q[2];
                diff[(q[1] + 1) as usize] -= q[2];
            }
            let mut sum = 0;
            let mut valid = true;
            for i in 0..nums.len() {
                sum += diff[i];
                if nums[i] > sum {
                    valid = false;
                }
            }
            if valid {
                right = i;
            } else {
                left = i + 1;
            }
        }

        if left > q {
            -1
        } else {
            left as i32
        }
    }
}
