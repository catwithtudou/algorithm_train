pub struct Solution;

impl Solution {
    pub fn number_of_points(nums: Vec<Vec<i32>>) -> i32 {
        let max_len = nums.iter().map(|interval|interval[1]).max().unwrap() as usize;

        let mut diff =vec![0;max_len+2];
        for interval in nums {
            diff[interval[0] as usize]+=1;
            diff[(interval[1]+1) as usize]-=1;
        }

        let mut ans = 0;
        let mut s = 0;
        for d in diff{
            s+=d;
            if s>0{
                ans+=1;
            }
        }

        ans
    }
}