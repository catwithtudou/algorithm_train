pub struct Solution;

impl Solution {
    pub fn path_existence_queries(n: i32, nums: Vec<i32>, max_diff: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = n as usize;
        let mut id = vec![0;n];
        for i in 1..n {
            id[i]=id[i-1];
            if nums[i]-nums[i-1]>max_diff{
                id[i]+=1;
            }
        }

        let mut ans = vec![false;queries.len()];

        for (i, q) in queries.iter().enumerate() {
            ans[i] = id[q[1] as usize] == id[q[0] as usize];
        }


        ans
    }
}