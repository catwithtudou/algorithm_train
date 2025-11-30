pub struct Solution;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let n = nums.len();
        let mut s = vec![0; n + 1];
        for (i, &x) in nums.iter().enumerate() {
            s[i + 1] = (s[i] + x) % p;
        }

        let x = s[n];
        if x == 0 {
            return 0;
        }

        let mut ans = n;
        let mut last = std::collections::HashMap::new();
        for (i, &v) in s.iter().enumerate() {
            last.insert(v, i);
            if let Some(&j) = last.get(&((v - x + p) % p)) {
                ans = ans.min(i - j);
            }
        }

        if ans < n {
            ans as i32
        } else {
            -1
        }
    }
}
