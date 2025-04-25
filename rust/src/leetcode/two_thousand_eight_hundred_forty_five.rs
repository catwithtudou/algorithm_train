pub struct Solution;

impl Solution {
    pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
        let n = nums.len();
        let mut sum = vec![0; n + 1];
        for (i, x) in nums.into_iter().enumerate() {
            sum[i + 1] = sum[i];
            if x % modulo == k {
                sum[i + 1] += 1;
            }
        }

        let mut ans = 0;
        let mut cnt = vec![0; (n + 1).min(modulo as usize)];

        for s in sum {
            if s >= k {
                ans += cnt[((s - k) % modulo) as usize] as i64;
            }
            cnt[(s % modulo) as usize] += 1;
        }

        ans
    }
}
