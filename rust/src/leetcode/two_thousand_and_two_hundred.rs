pub struct Solution;

impl Solution {
    pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
        let n = nums.len();
        let k = k as usize;

        let mut last = n;
        for i in (0..k).rev() {
            if nums[i] == key {
                last = i;
                break;
            }
        }

        let mut ans = vec![];
        for i in 0..n {
            if i + k < n && nums[i + k] == key {
                last = i + k;
            }

            if last < n && last + k >= i {
                ans.push(i as i32);
            }
        }

        ans
    }
}
