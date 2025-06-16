pub struct Solution;

impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut pre_min = i32::MAX;
        for x in nums {
            ans = ans.max(x - pre_min);
            pre_min = pre_min.min(x);
        }
        if ans == 0 {
            -1
        } else {
            ans
        }
    }
}
