pub struct Solution;

impl Solution {
    pub fn minimum_subarray_length(mut nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = usize::MAX;

        for i in 0..nums.len() {
            if nums[i] >= k {
                return 1;
            }
            let mut j = i - 1;
            while j < nums.len() && nums[j] | nums[i] != nums[j] {
                nums[j] |= nums[i];
                if nums[j]>=k {
                    ans = ans.min(i-j+1);
                }
                j-=1;
            }
        }

        if ans == usize::MAX {
            return -1;
        } else {
            ans as _
        }
    }
}
