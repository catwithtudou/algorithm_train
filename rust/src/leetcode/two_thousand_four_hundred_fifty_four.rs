pub struct Solution;

impl Solution {
    pub fn second_greater_element(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans: Vec<i32> = vec![-1; n];
        let mut s = Vec::new();
        let mut t = Vec::new();
        for (i, &x) in nums.iter().enumerate() {
            while !t.is_empty() && x > nums[*t.last().unwrap()] {
                ans[t.pop().unwrap()] = x;
            }

            let mut j = s.len();
            while j > 0 && x > nums[s[j-1]] {
                j -= 1;
            }
            t.extend(s.drain(j..));
            s.push(i);
        }
        ans
    }
}