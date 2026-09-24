pub struct Solution;

impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        for (i, &num) in nums.iter().enumerate() {
            let mut num = num;
            let mut di = 0;
            while num > 0 {
                di += num % 10;
                num /= 10;
            }
            if di == i as i32 {
                return i as i32;
            }
        }
        -1
    }
}
