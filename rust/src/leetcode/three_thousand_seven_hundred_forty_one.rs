pub struct Solution;

impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let mut last = vec![-n; n as usize + 1];
        let mut last2 = vec![-n; n as usize + 1];
        let mut ans = n;
        for (i, &x) in nums.iter().enumerate() {
            ans = ans.min(i as i32 - last2[x as usize]);
            last2[x as usize] = last[x as usize];
            last[x as usize] = i as i32;
        }
        if ans == n {
            -1
        } else {
            ans * 2
        }
    }
}