pub struct Solution;

impl Solution {
    pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
        let mut is_not_zero = true;

        let mut xor = 0;

        for &x in &nums {
            xor ^= x;
            if x > 0 {
                is_not_zero = false;
            }
        }

        if is_not_zero {
            return 0;
        }

        let  n = nums.len() as i32 ;

        if xor == 0 {
            n-1
        }else{
            n
        }
    }
}
