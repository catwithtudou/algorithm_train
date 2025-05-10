pub struct Solution;

impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        fn calc(nums: Vec<i32>) -> (i64, bool) {
            let mut sum = 0;
            let mut zero = false;
            for x in nums {
                if x == 0 {
                    zero = true;
                    sum += 1;
                } else {
                    sum += x as i64;
                }
            }
            (sum, zero)
        }

        let (s1, zero1) = calc(nums1);
        let (s2, zero2) = calc(nums2);
        if !zero1 && s1 < s2 || !zero2 && s2 < s1 {
            return -1;
        }
        s1.max(s2)
    }
}
