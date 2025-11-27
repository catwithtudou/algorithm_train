pub struct Solution;

impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut sum = vec![0];
        for x in nums {
            sum.push(sum.last().unwrap() + x as i64);
        }

        let mut min_s = vec![i64::MAX / 2; k as usize];

        let mut ans = i64::MIN;
        for (j, &s) in sum.iter().enumerate() {
            let i = (j as i32 % k) as usize;
            ans = ans.max(s - min_s[i]);
            min_s[i] = min_s[i].min(s);
        }

        ans
    }
}
