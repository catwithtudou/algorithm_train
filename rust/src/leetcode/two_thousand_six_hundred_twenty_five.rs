pub struct Solution;

impl Solution {
    pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
        nums.iter().max().unwrap() * k + (k - 1) * k / 2
    }
}

#[cfg(test)]
mod two_thousand_six_hundred_twenty_five_test{
    use super::*;

    #[test]
    fn test_maximize_sum(){
        assert_eq!(Solution::maximize_sum(vec![1,2,3,4,5],3),18)
    }
}