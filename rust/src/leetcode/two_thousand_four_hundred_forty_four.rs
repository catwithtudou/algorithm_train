pub struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let (mut ans,mut max_i,mut min_i,mut i0) = (0,-1,-1,-1);

        for (i,x) in nums.into_iter().enumerate() {
            let i = i as i32;
            if i == min_k {
                min_i = i;
            }
            if i == max_k {
                max_i = i;
            }

            if x < min_k || x > max_k {
                i0 = i;
            }

            ans += 0.max(min_i.min(max_i)-i0) as i64
        }

        ans
    }
}