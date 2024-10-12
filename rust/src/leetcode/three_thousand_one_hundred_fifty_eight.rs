pub struct Solution;

impl Solution {
    pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut vis = 0i64;
        for x in nums {
            if (vis>>x&1) > 0 {
                ans^=x;
            }else{
                vis|=1<<x;
            }
        }
        return ans;
    }
}