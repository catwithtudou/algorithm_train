pub struct Solution;

impl Solution {
    pub fn difference_of_sum(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for mut x in nums {
            ans += x;
            while x > 0 {
                ans -= x % 10;
                x /= 10;
            }
        }
        ans
    }
}
