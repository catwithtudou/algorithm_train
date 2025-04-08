pub struct Solution;

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut cnt = vec![0; 101];
        for (i , x) in nums.into_iter().enumerate().rev() {
            if cnt[x as usize] > 0 {
                return i as i32 /3+1;
            }
            cnt[x as usize]+=1;
        }
        0
    }
}
