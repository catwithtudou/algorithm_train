pub struct Solution;

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        let (mut fi, mut se) = (i32::MAX, i32::MAX);
        for &num in nums.iter().skip(1) {
            if num < fi {
                se = fi;
                fi = num;
            } else if num < se {
                se = num;
            }
        }
        nums[0] + fi + se
    }
}
