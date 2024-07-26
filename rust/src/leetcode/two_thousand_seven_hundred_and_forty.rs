pub struct Solution;


impl Solution {
	pub fn find_value_of_partition(mut nums: Vec<i32>) -> i32 {
		nums.sort_unstable();
		nums.windows(2).map(|w| w[1] - w[0]).min().unwrap()
	}
}