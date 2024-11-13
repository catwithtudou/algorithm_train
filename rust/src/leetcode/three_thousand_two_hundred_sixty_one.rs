pub struct Solution;

impl Solution {
    pub fn count_k_constraint_substrings(s: String, k: i32, queries: Vec<Vec<i32>>) -> Vec<i64> {
        let n = s.len();
        let mut right: Vec<usize> = vec![n; n];
        let mut sum = vec![0 as i64; n + 1];
        let mut cnt = vec![0; 2];
        let s = s.as_bytes();
        let mut l = 0;
        for i in 0..n {
            cnt[(s[i] - b'0') as usize] += 1;
            while cnt[0] > k && cnt[1] > k {
                cnt[(s[l] - b'0') as usize] -= 1;
                right[l] = i;
                l += 1;
            }
            sum[i + 1] = sum[i] + (i - l + 1) as i64;
        }

        let mut ans: Vec<i64> = Vec::with_capacity(queries.len());
        for query in queries {
            let (l, r) = (query[0] as usize, query[1] as usize);
            let i = std::cmp::min(right[l], r + 1);
            ans.push(((i - l + 1) * (i - l) / 2) as i64 + (sum[r + 1] - sum[i]));
        }

        ans
    }
}
