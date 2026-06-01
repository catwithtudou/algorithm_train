pub struct Solution;

impl Solution {
    pub fn minimum_cost(mut cost: Vec<i32>) -> i32 {
        cost.sort_unstable();

        let mut ans = 0;
        let mut i = cost.len() as i32 - 1;

        while i>=0 {
            ans+=cost[i as usize];
            if i>0 {
                ans+=cost[i as usize-1];
            }
            i-=3;
        }

        ans
    }
}