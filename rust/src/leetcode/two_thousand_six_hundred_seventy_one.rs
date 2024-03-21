use std::collections::HashMap;

struct FrequencyTracker {
	cnt: HashMap<i32, i32>,
	freq: HashMap<i32, i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrequencyTracker {
	fn new() -> Self {
		Self { cnt: HashMap::new(), freq: HashMap::new() }
	}

	fn update(&mut self, number: i32, delta: i32) {
		let c = self.cnt.entry(number).or_insert(0);
		*self.freq.entry(*c).or_insert(0) -= 1;
		*c += delta;
		*self.freq.entry(*c).or_insert(0) += 1;
	}

	fn add(&mut self, number: i32) {
		self.update(number, 1);
	}

	fn delete_one(&mut self, number: i32) {
		if *self.cnt.get(&number).unwrap_or(&0) > 0 {
			self.update(number, -1)
		}
	}

	fn has_frequency(&self, frequency: i32) -> bool {
		*self.freq.get(&frequency).unwrap_or(&0) > 0
	}
}

/**
 * Your FrequencyTracker object will be instantiated and called as such:
 * let obj = FrequencyTracker::new();
 * obj.add(number);
 * obj.delete_one(number);
 * let ret_3: bool = obj.has_frequency(frequency);
 */