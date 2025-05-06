pub struct Solution;

impl Solution {
    pub fn build_array(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() {
            let x = nums[i];

            if x < 0 {
                continue;
            }

            let mut cur = i;

            while nums[cur] as usize != i {
                let nxt = nums[cur] as usize;
                nums[cur] = !nums[nxt];
                cur = nxt;
            }
            nums[cur] = !x;
        }

        for i in 0..nums.len() {
            nums[i] = !nums[i];
        }

        nums
    }
}
