pub struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    fn pow(x: i64, y: i64, m: i64) -> i64 {
        let mut res = 1;
        let mut x = x;
        let mut y = y;
        while y > 0 {
            if y % 2 == 1 {
                res = (res * x) % m;
            }
            x = (x * x) % m;
            y /= 2;
        }
        res
    }

    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        if multiplier == 1 {
            return nums;
        }

        let n = nums.len();
        let m = 1_000_000_007;
        let mx: i64 = *nums.iter().max().unwrap() as i64;

        let mut v: BinaryHeap<Reverse<(i64, usize)>> = nums
            .into_iter()
            .enumerate()
            .map(|(i, x)| Reverse((x as i64, i)))
            .collect();

        let mut k = k as i64;
        while let Some(Reverse((val, _))) = v.peek() {
            if *val >= mx || k == 0 {
                break;
            }
            let Reverse((mut min_val, idx)) = v.pop().unwrap();
            min_val *= multiplier as i64;
            v.push(Reverse((min_val, idx)));
            k -= 1;
        }

        let mut result = vec![0; n];
        let mut vec_v = v.into_vec();
        vec_v.sort_unstable_by_key(|Reverse((val, idx))| (*val, *idx));

        for (i, Reverse((val, idx))) in vec_v.iter().enumerate() {
            let e = k / n as i64 + if (i as i64) < k % n as i64 { 1 } else { 0 };
            result[*idx] = ((val % m) * Solution::pow(multiplier as i64, e, m) % m) as i32;
        }
        result
    }
}
