pub struct Solution;

impl Solution {
    pub fn min_sum_of_lengths(arr: Vec<i32>, target: i32) -> i32 {
        let n = arr.len();
        let inf = n + 1;

        let mut suf_min = vec![inf; n];
        let mut min_len = inf;
        let mut sum = 0;
        let mut right = n;

        for left in (1..n).rev() {
            sum += arr[left];

            while sum > target {
                sum -= arr[right - 1];
                right -= 1;
            }

            if sum == target {
                min_len = min_len.min(right - left);
            }

            suf_min[left] = min_len;
        }

        let mut ans = inf;
        let mut sum = 0;
        let mut left = 0;

        for right in 0..n.saturating_sub(1) {
            sum += arr[right];

            while sum > target {
                sum -= arr[left];
                left += 1;
            }

            if sum == target {
                ans = ans.min(right - left + 1 + suf_min[right + 1]);
            }
        }

        if ans > n {
            -1
        } else {
            ans as i32
        }
    }
}
