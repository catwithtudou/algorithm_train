pub struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn count_non_decreasing_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut right_bound = vec![n; n];
        let mut stack: Vec<usize> = Vec::new();

        for (i, &x) in nums.iter().enumerate() {
            while let Some(&j) = stack.last() {
                if nums[j] > x {
                    break;
                }

                stack.pop();
                right_bound[j] = i;
            }

            if let Some(&j) = stack.last() {
                graph[j].push(i);
            }

            stack.push(i);
        }

        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut left = 0;
        let mut cost = 0i64;
        let mut ans = 0i64;
        let k = k as i64;

        for (right, &x) in nums.iter().enumerate() {
            while let Some(&j) = queue.back() {
                if nums[j] > x {
                    break;
                }
                queue.pop_back();
            }

            queue.push_back(right);
            cost += nums[*queue.front().unwrap()] as i64 - x as i64;

            while cost > k {
                let out = nums[left] as i64;

                for &i in &graph[left] {
                    if i > right {
                        break;
                    }

                    cost -= (out - nums[i] as i64)
                        * (right_bound[i].min(right + 1) - i) as i64;
                }

                left += 1;

                if queue.front().is_some_and(|&i| i < left) {
                    queue.pop_front();
                }
            }

            ans += (right - left + 1) as i64;
        }

        ans
    }
}