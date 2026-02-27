pub struct Solution;

use std::collections::{BTreeSet, VecDeque};

impl Solution {
    pub fn min_operations(s: String, k: i32) -> i32 {
        let n = s.len() as i32;
        let k = k as i32;

        // 使用 BTreeSet 代替红黑树，存储未访问的位置
        let mut not_vis = [BTreeSet::new(), BTreeSet::new()];

        // 初始化两个集合：偶数位置和奇数位置
        for m in 0..2 {
            let mut i = m;
            while i <= n {
                not_vis[m as usize].insert(i);
                i += 2;
            }
            not_vis[m as usize].insert(n + 1);
        }

        // 计算初始状态（'0' 的数量）
        let start = s.chars().filter(|&c| c == '0').count() as i32;
        not_vis[(start % 2) as usize].remove(&start);

        let mut q = VecDeque::new();
        q.push_back(start);
        let mut ans = 0;

        while !q.is_empty() {
            let tmp: Vec<i32> = q.drain(..).collect();

            for z in tmp {
                if z == 0 {
                    return ans;
                }

                // 计算可达范围
                let mn = z + k - 2 * k.min(z);
                let mx = z + k - 2 * 0.max(k - n + z);

                let t = &mut not_vis[(mn % 2) as usize];

                // 找到所有在 [mn, mx] 范围内的未访问节点
                let keys_to_remove: Vec<i32> = t.range(mn..=mx).copied().collect();

                for key in keys_to_remove {
                    q.push_back(key);
                    t.remove(&key);
                }
            }

            ans += 1;
        }

        -1
    }
}