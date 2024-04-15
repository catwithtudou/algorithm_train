
use std::collections::LinkedList;

struct MyHashMap {
	data: Vec<LinkedList<(i32, i32)>>,
	base: usize,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
	fn new() -> Self {
		Self {
			data: vec![LinkedList::new(); 769],
			base: 769,
		}
	}

	fn hash(&self, key: i32) -> usize {
		key as usize % self.base
	}

	fn put(&mut self, key: i32, value: i32) {
		let h = self.hash(key);
		for i in self.data[h].iter_mut() {
			if i.0 == key {
				i.1 = value;
				return;
			}
		}
		self.data[h].push_back((key, value));
	}

	fn get(&self, key: i32) -> i32 {
		let h = self.hash(key);
		for i in self.data[h].iter() {
			if i.0 == key {
				return i.1;
			}
		}
		-1
	}

	fn remove(&mut self, key: i32) {
		let h = self.hash(key);
		for (i, &item) in self.data[h].iter().enumerate() {
			if item.0 == key {
				let mut split_list = self.data[h].split_off(i);
				split_list.pop_front();
				self.data[h].append(&mut split_list);
				return;
			}
		}
	}
}

/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */