use std::collections::HashSet;

pub struct Solution;

impl Solution {
	pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
		let set1 = nums1.iter().cloned().collect::<HashSet<_>>();
		let set2 = nums2.iter().cloned().collect::<HashSet<_>>();
		let cnt1 = nums1.iter().filter(|&x| set2.contains(x)).count() as i32;
		let cnt2 = nums2.iter().filter(|&x| set1.contains(x)).count() as i32;
		vec![cnt1, cnt2]
	}
}
