pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;
        let x = x as usize;
        if k == 0 || k > n {
            return vec![];
        }

        let mut res = vec![0; n - k + 1];
        let mut cnt: HashMap<i32, i32> = HashMap::new();

        for &v in &nums[0..k] {
            *cnt.entry(v).or_insert(0) += 1;
        }
        res[0] = Self::xsum_by_count(&cnt, x);

        for i in k..n {
            let leave = nums[i - k];
            if let Some(e) = cnt.get_mut(&leave) {
                *e -= 1;
                if *e == 0 {
                    cnt.remove(&leave);
                }
            }
            *cnt.entry(nums[i]).or_insert(0) += 1;
            res[i - k + 1] = Self::xsum_by_count(&cnt, x)
        }

        res
    }

    fn xsum_by_count(cnt: &HashMap<i32, i32>, x: usize) -> i32 {
        let mut items: Vec<(i32, i32)> = cnt.iter().map(|(&v, &f)| (v, f)).collect();
        items.sort_unstable_by(|a, b| match b.1.cmp(&a.1) {
            std::cmp::Ordering::Equal => b.0.cmp(&a.0),
            other => other,
        });
        let mut take = 0;
        let mut sum = 0;
        for (v, f) in items.into_iter() {
            if take == x {
                break;
            }
            sum += v * f;
            take += 1;
        }
        sum
    }
}
