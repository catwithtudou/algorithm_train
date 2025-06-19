pub struct Solution;

impl Solution {
    pub fn partition_array(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut ans = 0;
        let mut mn = i32::MIN / 2;
        for x in nums {
            if x - mn > k {
                ans += 1;
                mn = x;
            }
        }
        ans
    }
}
