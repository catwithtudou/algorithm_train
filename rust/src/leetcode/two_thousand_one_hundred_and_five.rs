pub struct Solution;

impl Solution {
	pub fn minimum_refill(plants: Vec<i32>, capacity_a: i32, capacity_b: i32) -> i32 {
		let mut ans = 0;
		let (mut a, mut b) = (capacity_a, capacity_b);
		let (mut i, mut j) = (0, plants.len() - 1);

		while i < j {
			if a < plants[i] {
				ans += 1;
				a = capacity_a;
			}
			a -= plants[i];
			i += 1;

			if b < plants[j] {
				ans += 1;
				b = capacity_b;
			}
			b -= plants[j];
			j -= 1;
		}

		if i == j && a.max(b) < plants[i] {
			ans += 1;
		}


		ans
	}
}