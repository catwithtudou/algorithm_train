use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn max_partitions_after_operations(s: String, k: i32) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut memo: HashMap<(usize, u32, bool), i32> = HashMap::new();

        fn dfs(
            i: usize,
            mask: u32,
            changed: bool,
            bytes: &[u8],
            n: usize,
            k: i32,
            memo: &mut HashMap<(usize, u32, bool), i32>,
        ) -> i32 {
            if i == n {
                return 1;
            }

            let key = (i, mask, changed);
            if let Some(&res) = memo.get(&key) {
                return res;
            }

            let bit = 1u32 << ((bytes[i] - b'a') as u32);
            let new_mask = mask | bit;

            let mut res = if new_mask.count_ones() as i32 <= k {
                dfs(i + 1, new_mask, changed, bytes, n, k, memo)
            } else {
                1 + dfs(i + 1, bit, changed, bytes, n, k, memo)
            };

            if !changed {
                for j in 0..26u32 {
                    let new_mask2: u32 = mask | (1u32 << j);
                    if (new_mask2.count_ones() as i32) > k {
                        res = res.max(dfs(i + 1, 1u32 << j, true, bytes, n, k, memo) + 1);
                    } else {
                        res = res.max(dfs(i + 1, new_mask2, true, bytes, n, k, memo));
                    }
                }
            }

            memo.insert(key, res);
            res
        }

        dfs(0, 0u32, false, bytes, n, k, &mut memo)
    }
}
