pub struct Solution;

use std::collections::BTreeSet;

#[derive(Default)]
struct MedianWindow {
    // 较小的一半，包含中位数
    lower: BTreeSet<(i64, usize)>,
    upper: BTreeSet<(i64, usize)>,
    lower_sum: i64,
    upper_sum: i64,
}

impl MedianWindow {
    fn insert(&mut self, value: i64, idx: usize) {
        let item = (value, idx);

        if self.lower.last().map_or(true, |&top| item <= top) {
            self.lower.insert(item);
            self.lower_sum += value;
        } else {
            self.upper.insert(item);
            self.upper_sum += value;
        }

        self.balance();
    }

    fn remove(&mut self, value: i64, idx: usize) {
        let item = (value, idx);

        if self.lower.remove(&item) {
            self.lower_sum -= value;
        } else if self.upper.remove(&item) {
            self.upper_sum -= value;
        }

        self.balance();
    }

    fn balance(&mut self) {
        if self.lower.len() > self.upper.len() + 1 {
            let item = self.lower.pop_last().unwrap();
            self.lower_sum -= item.0;
            self.upper_sum += item.0;
            self.upper.insert(item);
        } else if self.lower.len() < self.upper.len() {
            let item = self.upper.pop_first().unwrap();
            self.upper_sum -= item.0;
            self.lower_sum += item.0;
            self.lower.insert(item);
        }
    }

    // 将窗口所有元素变成中位数所需的最小操作数
    fn cost(&self) -> i64 {
        let median = self.lower.last().unwrap().0;

        median * self.lower.len() as i64 - self.lower_sum
            + self.upper_sum - median * self.upper.len() as i64
    }
}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32, k: i32) -> i64 {
        let n = nums.len();
        let x = x as usize;
        let k = k as usize;

        // 1. 计算每个长度为 x 的窗口的最小操作数
        let mut window = MedianWindow::default();

        for i in 0..x {
            window.insert(nums[i] as i64, i);
        }

        let mut costs = Vec::with_capacity(n - x + 1);
        costs.push(window.cost());

        for i in x..n {
            window.remove(nums[i - x] as i64, i - x);
            window.insert(nums[i] as i64, i);
            costs.push(window.cost());
        }

        // 2. DP：选择 k 个互不重叠的长度为 x 的子数组
        const INF: i64 = i64::MAX / 4;

        let mut prev = vec![0i64; n + 1];
        let mut curr = vec![INF; n + 1];

        for i in 1..=k {
            curr.fill(INF);

            // 左右需要留出足够空间
            for j in i * x..=n - (k - i) * x {
                curr[j] = curr[j - 1].min(
                    prev[j - x] + costs[j - x]
                );
            }

            std::mem::swap(&mut prev, &mut curr);
        }

        prev[n]
    }
}