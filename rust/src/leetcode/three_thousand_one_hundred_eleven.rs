pub struct Solution;

impl Solution {
	pub fn min_rectangles_to_cover_points(mut points: Vec<Vec<i32>>, w: i32) -> i32 {
		points.sort_unstable_by(|p, q| p[0].cmp(&q[0]));
		let mut ans = 0;
		let mut x2 = -1;
		for p in points {
			if p[0] > x2 {
				ans += 1;
				x2 = p[0] + w;
			}
		}
		ans
	}
}