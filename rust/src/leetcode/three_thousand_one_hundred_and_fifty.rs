pub struct Solution;

impl Solution {
	pub fn maximum_prime_difference(nums: Vec<i32>) -> i32 {
		let (mut i, mut j) = (0, nums.len() as i32 - 1);
		while !Self::is_prime(nums[i as usize]) {
			i += 1;
		}
		while !Self::is_prime(nums[j as usize]) {
			j -= 1;
		}
		j - i
	}

	pub fn is_prime(n: i32) -> bool {
		if n < 2 {
			return false;
		}
		for i in 2..=(n as f64).sqrt() as i32 {
			if n % i == 0 {
				return false;
			}
		}
		true
	}
}