pub struct Solution;

impl Solution {
	pub fn find_peaks(mountain: Vec<i32>) -> Vec<i32> {
		mountain.windows(3)
			.enumerate()
			.filter_map(|(i, w)| {
				if w[0] < w[1] && w[1] > w[2] { Some(i as i32 + 1) } else { None }
			})
			.collect()
	}
}