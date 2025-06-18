pub struct Solution;

impl Solution {
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut ans = vec![];
        for a in nums.chunks(3) {
            if a[2] - a[0] > k {
                return vec![];
            }
            ans.push(a.to_vec());
        }
        ans
    }
}
