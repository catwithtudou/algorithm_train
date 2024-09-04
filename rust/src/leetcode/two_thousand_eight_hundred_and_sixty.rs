pub struct Solution;

impl Solution {
    pub fn count_ways(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut ans = (nums[0]>0) as i32;
        for i in 1..nums.len(){
            if nums[i-1]<i as i32 && nums[i]>i as i32{
                ans+=1;
            }
        }
        ans+1
    }
}