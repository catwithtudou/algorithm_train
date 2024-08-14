pub struct Solution;

impl Solution {
    pub fn is_array_special(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = nums.len();
        let mut last_same = vec![0;n];
        for i in 1..n {
            if nums[i-1]%2==nums[i]%2{
                last_same[i]=i
            }else{
                last_same[i]=last_same[i-1]
            }
        }

        let mut ans = vec![false;queries.len()];
        for (i,q) in queries.iter().enumerate() {
            ans[i]= last_same[q[1] as usize] <= q[0] as usize;
        }
        ans
    }
}