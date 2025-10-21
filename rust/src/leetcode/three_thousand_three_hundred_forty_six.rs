pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        let mut cnt: HashMap<i32, i32> = HashMap::new();
        let mut diff: HashMap<i32, i32> = HashMap::new();

        // 统计每个数出现次数，并构造差分数组（用 HashMap 存）
        for &x in &nums {
            *cnt.entry(x).or_insert(0) += 1;

            // 确保 x 出现在 diff 的 key 集合里
            diff.entry(x).or_insert(0);

            // 区间 [x-k, x+k] 做差分 +1/-1
            *diff.entry(x - k).or_insert(0) += 1;
            *diff.entry(x + k + 1).or_insert(0) -= 1;
        }

        // 将 diff 的键取出来排序（等价于 slices.Sorted(maps.Keys(diff))）
        let mut keys: Vec<i32> = diff.keys().cloned().collect();
        keys.sort_unstable();

        // 前缀和扫描 + 取 min/max
        let mut sum_d = 0;
        let mut ans = 0;
        for x in keys {
            sum_d += diff.get(&x).copied().unwrap_or(0);
            let cap = cnt.get(&x).copied().unwrap_or(0) + num_operations;
            ans = ans.max(sum_d.min(cap));
        }

        ans
    }
}
