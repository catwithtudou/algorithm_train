pub struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![0; n];
        let (mut i, mut j) = (0, n - 1);
        for p in (0..n).rev() {
            let x = nums[i]*nums[i];
            let y = nums[j]*nums[j];
            if x > y {
                ans[p]=x;
                i+=1;
            }else{
                ans[p]=y;
                j-=1;
            }

        }

        ans
    }
}
