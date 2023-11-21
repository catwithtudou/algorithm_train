pub struct Solution;

impl Solution {
    pub fn min_deletion(nums: Vec<i32>) -> i32 {
        let mut cnt = 0;
        let mut check = true;
        for i in 0..nums.len() - 1 {
            if nums[i] == nums[i + 1] && check {
                cnt += 1;
                continue;
            }
            check = !check;
        }

        if (nums.len() - cnt) % 2 != 0 {
            cnt += 1;
        }

        cnt as i32
    }
}