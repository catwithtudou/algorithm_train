use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn maximum_robots(charge_times: Vec<i32>, running_costs: Vec<i32>, budget: i64) -> i32 {
        let (mut ans,mut left,mut sum) = (0,0,0i64);
        let mut q = VecDeque::new();
        for right in 0..charge_times.len(){

            while !q.is_empty() && charge_times[right] >= charge_times[*q.back().unwrap()] {
                q.pop_back();
            }

            q.push_back(right);
            sum+=running_costs[right] as i64;

            while !q.is_empty() && charge_times[*q.front().unwrap()] as i64 + sum * (right-left+1)as i64 > budget {
                if left == *q.front().unwrap() {
                    q.pop_front();
                }
                sum-=running_costs[left] as i64;
                left+=1;
            }

            ans=ans.max(right-left+1);
        }
        ans as _
    }
}