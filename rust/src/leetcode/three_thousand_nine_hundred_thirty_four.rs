pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn smallest_unique_subarray(nums: Vec<i32>) -> i32 {
        const BASE: i64 = 911_382_323;
        const MODS: [i64; 2] = [1_000_000_007, 1_000_000_009];

        let n = nums.len();
        let mut pow_base = vec![[1i64; 2]; n + 1];
        let mut pre_hash = vec![[0i64; 2]; n + 1];

        // 预处理前缀哈希和基数幂
        for (i, &x) in nums.iter().enumerate() {
            for (j, &m) in MODS.iter().enumerate() {
                pow_base[i + 1][j] = pow_base[i][j] * BASE % m;
                pre_hash[i + 1][j] =
                    (pre_hash[i][j] * BASE + x as i64).rem_euclid(m);
            }
        }

        // 计算子数组 nums[l..r] 的双哈希值
        let sub_hash = |l: usize, r: usize| -> [i64; 2] {
            std::array::from_fn(|i| {
                (pre_hash[r][i] - pre_hash[l][i] * pow_base[r - l][i])
                    .rem_euclid(MODS[i])
            })
        };

        // 判断是否存在长度为 length 的唯一子数组
        let check = |length: usize| -> bool {
            let mut cnt: HashMap<[i64; 2], usize> = HashMap::new();

            for i in length..=n {
                let hash = sub_hash(i - length, i);
                *cnt.entry(hash).or_default() += 1;
            }

            cnt.values().any(|&c| c == 1)
        };

        let mut left = 0;
        let mut right = n;

        while left < right {
            let mid = left + (right - left) / 2;

            if check(mid) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left as i32
    }
}