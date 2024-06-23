pub struct Solution;


impl Solution {
	pub fn number_of_employees_who_met_target(hours: Vec<i32>, target: i32) -> i32 {
		hours.iter().filter(|&&h| h >= target).count() as i32
	}
}