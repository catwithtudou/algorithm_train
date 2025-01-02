use std::collections::LinkedList;

struct MyHashSet {
	data: Vec<LinkedList<i32>>,
	capacity: usize,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
	fn new() -> Self {
		let capacity = 769;
		MyHashSet {
			data: vec![LinkedList::new(); capacity],
			capacity,
		}
	}

	fn hash(&self, key: i32) -> usize {
		(key as usize) % self.capacity
	}

	fn add(&self, key: i32) {
		if self.contains(key) {
			return;
		}
		// self.data[self.hash(key)].push_back(key);
	}

	fn remove(&mut self, key: i32) {
		if !self.contains(key) {
			return;
		}

		let index = self.hash(key);
		let list = &mut self.data[index];
		// let mut cursor = list.cursor_front_mut();
		// while let Some(&mut value) = cursor.current() {
		// 	if value == key {
		// 		cursor.remove_current();
		// 		break;
		// 	}
		// 	cursor.move_next();
		// }
	}

	fn contains(&self, key: i32) -> bool {
		self.data[self.hash(key)].iter().any(|&k| k == key)
	}
}

// /**
//  * Your MyHashSet object will be instantiated and called as such:
//  * let obj = MyHashSet::new();
//  * obj.add(key);
//  * obj.remove(key);
//  * let ret_3: bool = obj.contains(key);
//  */