use std::mem::swap;

pub struct Solution;

impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut p = 0;
        let mut q = 0;

        for (i, &x) in nums.iter().enumerate() {
            if x < nums[p] {
                p = i;
            } else if x > nums[q] {
                q = i;
            }
        }

        if p > q {
            swap(&mut p, &mut q);
        }

        (q+1).min(n-p).min(p+1+n-q) as i32
    }
}
