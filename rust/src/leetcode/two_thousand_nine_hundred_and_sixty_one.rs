pub struct Solution;


impl Solution {
	pub fn get_good_indices(variables: Vec<Vec<i32>>, target: i32) -> Vec<i32> {
		let pow = |mut x, mut n, m| {
			let mut res = 1;
			while n > 0 {
				if n % 2 > 0 {
					res = res * x % m;
				}
				x = x * x % m;
				n /= 2;
			}
			res
		};

		let check = |v: &Vec<_>| pow(pow(v[0], v[1], 10), v[2], v[3]) == target;

		variables.iter().enumerate().filter_map(|(i, v)| check(v).then_some(i as i32)).collect()
	}
}