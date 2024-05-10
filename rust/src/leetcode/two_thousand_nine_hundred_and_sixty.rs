pub struct Solution;


impl Solution {
	pub fn count_tested_devices(battery_percentages: Vec<i32>) -> i32 {
		let mut dec = 0;
		for &x in &battery_percentages {
			if x - dec > 0 {
				dec += 1;
			}
		}

		dec
	}
}