pub struct Solution;

impl Solution {
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        let mut a = vec![nums[0]];
        let mut b = vec![nums[1]];

        for &x in &nums[2..] {
            if a.last() > b.last() {
                a.push(x);
            } else {
                b.push(x);
            }
        }

        a.extend(b);
        a
    }
}