pub struct Solution;

impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let total = nums.iter().sum::<i32>();
        let mut ans = 0;
        let mut pre_sum = 0;

        for x in nums {
            pre_sum += x;
            if x != 0 {
                continue;
            }
            if pre_sum == (total - pre_sum) {
                ans += 2;
            } else if (pre_sum * 2 - total).abs() == 1 {
                ans += 1;
            }
        }

        ans
    }
}
