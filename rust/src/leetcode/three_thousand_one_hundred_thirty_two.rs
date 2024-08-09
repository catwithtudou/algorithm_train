pub struct Solution;


impl Solution {
    pub fn minimum_added_integer(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
        nums1.sort_unstable();
        nums2.sort_unstable();
        for i in (1..3).rev() {
            let x = nums2[0] - nums1[i];
            let mut j = 0;
            for &v in &nums1[i..] {
                if nums2[j] == v + x && {
                    j += 1;
                    j == nums2.len()
                } {
                    return x;
                }
            }
        }
        nums2[0] - nums1[0]
    }
}