pub struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn count_of_pairs(nums: Vec<i32>) -> i32 {
        let mut memo = HashMap::new();
        fn get_total_cnt(
            idx: usize,
            prev: i32,
            memo: &mut HashMap<(usize, i32), i32>,
            nums: &Vec<i32>,
        ) -> i32 {
            let mod1 = 1_000_000_007;
            if let Some(&v) = memo.get(&(idx, prev)) {
                return v;
            }

            if idx == nums.len() {
                return 1;
            }

            let mut cnt = 0;
            for i in (prev as usize..=nums[idx] as usize).rev() {
                if idx != 0 {
                    let prev = nums[idx - 1] - prev;
                    if nums[idx] - i as i32 > prev {
                        break;
                    }
                }
                cnt += get_total_cnt(idx + 1, i as i32, memo, nums);
                cnt %= mod1;
            }
            memo.insert((idx, prev), cnt);
            cnt
        }

        get_total_cnt(0, 0, &mut memo, &nums)
    }
}
