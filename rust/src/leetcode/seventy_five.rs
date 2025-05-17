pub struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut i, mut k, mut j) = (-1, 0, nums.len());

        while k < j {
            if nums[k] == 0 {
                i += 1;
                nums.swap(i as usize, k);
                k += 1;
            } else if nums[k] == 1 {
                k += 1;
            } else if nums[k] == 2 {
                j -= 1;
                nums.swap(k, j);
            }
        }
    }
}
