pub struct Solution;

impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut pre_state = 0;
        for w in nums.windows(2) {
            let x = w[0];
            let y = w[1];
            if x > y {
                if pre_state == -1 {
                    ans += 1;
                }
                pre_state = 1;
            } else if x < y {
                if pre_state == 1 {
                    ans += 1;
                }
                pre_state = -1;
            }
        }
        ans
    }
}
