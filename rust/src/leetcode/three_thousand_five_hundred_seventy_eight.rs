use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn count_partitions(nums: Vec<i32>, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let n = nums.len();
        let mut f = vec![0_i64; n + 1];
        f[0] = 1;

        let mut min_q: VecDeque<usize> = VecDeque::new();
        let mut max_q: VecDeque<usize> = VecDeque::new();
        let mut sum_f = 0_i64;
        let mut left = 0_usize;

        for i in 0..n {
            sum_f = (sum_f + f[i]) % MOD;
            let x = nums[i];

            while let Some(&idx) = min_q.back() {
                if x <= nums[idx] {
                    min_q.pop_back();
                } else {
                    break;
                }
            }
            min_q.push_back(i);

            while let Some(&idx) = max_q.back() {
                if x >= nums[idx] {
                    max_q.pop_back();
                } else {
                    break;
                }
            }
            max_q.push_back(i);

            while nums[*max_q.front().unwrap()] - nums[*min_q.front().unwrap()] > k {
                sum_f = (sum_f - f[left] + MOD) % MOD;
                left += 1;
                if *min_q.front().unwrap() < left {
                    min_q.pop_front();
                }
                if *max_q.front().unwrap() < left {
                    max_q.pop_front();
                }
            }

            f[i + 1] = sum_f;
        }

        (f[n] % MOD) as i32
    }
}
