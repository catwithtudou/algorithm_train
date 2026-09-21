pub struct Solution;

impl Solution {
    pub fn result_array(nums: Vec<i32>, k: i32) -> Vec<i64> {
        let k = k as usize;
        let mut ans = vec![0i64; k];
        let mut dp = vec![0i64; k];

        for v in nums {
            let v = v.rem_euclid(k as i32) as usize;
            let mut next = vec![0i64; k];

            next[v] = 1;

            for (y, &count) in dp.iter().enumerate() {
                next[y * v % k] += count;
            }

            for (x, &count) in next.iter().enumerate() {
                ans[x] += count;
            }

            dp = next;
        }

        ans
    }
}