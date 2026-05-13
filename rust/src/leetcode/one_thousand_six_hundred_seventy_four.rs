pub struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        let n = nums.len();
        let size = (limit * 2 + 2) as usize;
        let mut diff = vec![0; size];

        for (i, &x) in nums.iter().take(n / 2).enumerate() {
            let y = nums[n - 1 - i];

            let l = x.min(y) + 1;
            let r = x.max(y) + limit;

            diff[2] += 2;
            diff[l as usize] -= 2;

            diff[l as usize] += 1;
            diff[(r + 1) as usize] -= 1;

            diff[(x + y) as usize] -= 1;
            diff[(x + y + 1) as usize] += 1;

            diff[(r + 1) as usize] += 2;
        }

        let mut ans = i32::MAX;
        let mut sum_d = 0;

        for &v in &diff[2..=(limit * 2) as usize] {
            sum_d += v;
            ans = ans.min(sum_d);
        }

        ans
    }
}