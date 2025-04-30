pub struct Solution;

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut ans = 0;

        for mut x in nums {
            while x >= 100 {
                x /= 100;
            }

            if x >= 10 {
                ans += 1;
            }
        }

        ans
    }
}
