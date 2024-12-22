pub struct Solution;

use std::cmp::Ordering;
use std::collections::HashMap;

impl Solution {
    pub fn get_kth(lo: i32, hi: i32, k: i32) -> i32 {
        let mut memo: HashMap<i32, i32> = HashMap::new();

        fn dfs(i: i32, memo: &mut HashMap<i32, i32>) -> i32 {
            if i == 1 {
                return 0;
            }

            if let Some(&res) = memo.get(&i) {
                return res;
            }

            let result = if i % 2 == 0 {
                dfs(i / 2, memo) + 1
            } else {
                dfs((i * 3 + 1) / 2, memo) + 2
            };
            memo.insert(i, result);
            result
        }

        let mut nums: Vec<i32> = (lo..=hi).collect();

        nums.sort_by(|&x, &y| {
            let dx = dfs(x, &mut memo);
            let dy = dfs(y, &mut memo);
            match dx.cmp(&dy) {
                Ordering::Equal => x.cmp(&y),
                ord => ord,
            }
        });

        nums[k as usize - 1]
    }
}
