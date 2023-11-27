use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn sum_subarray_mins(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut left = vec![0; n];
        let mut right = vec![0; n];
        let mut mono_stk: VecDeque<usize> = VecDeque::new();

        for i in 0..n {
            while !mono_stk.is_empty() && arr[*mono_stk.back().unwrap()] >= arr[i] {
                mono_stk.pop_back();
            }
            if let Some(&top) = mono_stk.back() {
                left[i] = (i - top) as i32;
            } else {
                left[i] = (i + 1) as i32;
            }
            mono_stk.push_back(i);
        }

        mono_stk.clear();

        for i in (0..n).rev() {
            while !mono_stk.is_empty() && arr[*mono_stk.back().unwrap()] > arr[i] {
                mono_stk.pop_back();
            }
            if let Some(&top) = mono_stk.back() {
                right[i] = (top - i) as i32;
            } else {
                right[i] = (n - i) as i32;
            }
            mono_stk.push_back(i);
        }

        let MOD = 1_000_000_007;
        let mut ans: i64 = 0;
        for i in 0..n {
            ans += (left[i] as i64 * right[i] as i64 * arr[i] as i64) % MOD;
            ans %= MOD;
        }

        ans as i32
    }
}