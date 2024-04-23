pub struct Solution;


impl Solution {
	pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
		let mut s = vec![0; 2];
		let mut max_s1 = 0;
		let k = minutes as usize;
		for (i, (&v, &g)) in customers.iter().zip(grumpy.iter()).enumerate() {
			s[g as usize] += v;
			if i < k - 1 {
				continue;
			}
			max_s1 = max_s1.max(s[1]);
			if grumpy[i - k + 1] > 0 {
				s[1] -= customers[i - k + 1];
			}
		}


		s[0] + max_s1
	}
}