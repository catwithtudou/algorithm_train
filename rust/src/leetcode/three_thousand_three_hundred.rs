pub struct Solution;

impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        let mut ans = i32::MAX;
        for mut num in nums {
            let mut x = 0;
            while num > 0 {
                x += num % 10;
                num = num /10;
            }
            ans = ans.min(x);
        }
        ans
    }
}