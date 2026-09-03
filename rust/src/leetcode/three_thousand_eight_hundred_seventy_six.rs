pub struct Solution;

impl Solution {
    pub fn uniform_array(nums1: Vec<i32>) -> bool {
        let mut mn = vec![i32::MAX,i32::MAX];

        for (_,&x) in nums1.iter().enumerate() {
            mn[(x&1) as usize] = mn[(x&1) as usize].min(x)
        }

        mn[1] == i32::MAX || mn[0]>mn[1]
    }
}