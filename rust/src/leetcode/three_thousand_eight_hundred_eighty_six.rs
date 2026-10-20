pub struct Solution;

impl Solution {
    pub fn sortable_integers(nums: Vec<i32>) -> i64 {
        let n = nums.len();

        // next_dec[i] = 从 i 往右第一个 p 满足 nums[p] > nums[p+1]，没有则为 n
        let mut next_dec = vec![n; n];
        let mut p = n;
        for i in (0..n.saturating_sub(1)).rev() {
            if nums[i] > nums[i + 1] {
                p = i;
            }
            next_dec[i] = p;
        }

        // 判断块长 k 是否可行
        let check = |k: usize| -> bool {
            let mut last_max = i32::MIN;
            let mut r = k - 1;
            while r < n {
                let l = r + 1 - k;
                let m = next_dec[l];
                if m >= r {
                    // 本身有序
                    if nums[l] < last_max {
                        return false;
                    }
                    last_max = nums[r];
                } else {
                    // 恰好一个下降点 m，且首尾能接上
                    if next_dec[m + 1] < r || nums[m + 1] < last_max || nums[r] > nums[l] {
                        return false;
                    }
                    last_max = nums[m];
                }
                r += k;
            }
            true
        };

        let mut ans: i64 = 0;
        let mut k = 1;
        while k * k <= n {
            if n % k == 0 {
                if check(k) {
                    ans += k as i64;
                }
                if k * k < n && check(n / k) {
                    ans += (n / k) as i64;
                }
            }
            k += 1;
        }
        ans
    }
}