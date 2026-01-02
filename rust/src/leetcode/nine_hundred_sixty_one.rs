pub struct Solution;

impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut hp = 0;

        for i in 1..nums.len() {
            if nums[i] == nums[0] {
                return nums[i];
            }
            if hp == 0 {
                ans = nums[i];
                hp = 1;
            } else if nums[i] == ans {
                hp += 1;
            } else {
                hp -= 1;
            }
        }

        ans
    }
}
