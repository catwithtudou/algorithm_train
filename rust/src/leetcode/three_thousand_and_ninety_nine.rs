pub struct Solution;

impl Solution {
	pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
		let mut s = 0;
		let mut v = x;
		while v > 0 {
			s += v % 10;
			v /= 10;
		}
		if x % s == 0 { s } else { -1 }
	}
}