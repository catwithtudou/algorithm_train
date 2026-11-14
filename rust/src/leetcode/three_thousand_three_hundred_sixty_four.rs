pub struct Solution;

impl Solution {
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let n = nums.len();
        let (l, r) = (l as usize, r as usize);
        let mut ans = i32::MAX;

        for i in 0..=n - l {
            let mut sum = 0;

            for j in i..(i + r).min(n) {
                sum += nums[j];

                if sum > 0 && j - i + 1 >= l {
                    ans = ans.min(sum);
                }
            }
        }

        if ans == i32::MAX { -1 } else { ans }
    }
}