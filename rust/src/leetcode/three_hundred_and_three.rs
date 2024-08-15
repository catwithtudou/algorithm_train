struct NumArray {
	s: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
	fn new(nums: Vec<i32>) -> Self {
		let mut s = vec![0; nums.len() + 1];
		for (i, &x) in nums.iter().enumerate() {
			s[i + 1] = s[i] + x;
		}
		Self { s }
	}

	fn sum_range(&self, left: i32, right: i32) -> i32 {
		self.s[right as usize + 1] - self.s[left as usize]
	}
}

// /**
//  * Your NumArray object will be instantiated and called as such:
//  * let obj = NumArray::new(nums);
//  * let ret_1: i32 = obj.sum_range(left, right);
//  */