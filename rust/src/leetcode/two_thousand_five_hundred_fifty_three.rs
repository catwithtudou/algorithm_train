pub struct Solution;

impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();

        for x in nums {
            for b in x.to_string().bytes() {
                ans.push((b - b'0') as i32);
            }
        }

        ans
    }
}
