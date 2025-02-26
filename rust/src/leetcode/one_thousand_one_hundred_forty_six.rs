use std::collections::HashMap;
#[allow(dead_code)]
struct SnapshotArray {
	cur_id: i32,
	history: HashMap<i32, Vec<(i32, i32)>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl SnapshotArray {
	fn new(_: i32) -> Self {
		Self {
			cur_id: 0,
			history: HashMap::new(),
		}
	}

	fn set(&mut self, index: i32, val: i32) {
		self.history.entry(index).or_insert(vec![]).push((self.cur_id, val));
	}

	fn snap(&mut self) -> i32 {
		self.cur_id += 1;
		self.cur_id - 1
	}

	fn get(&self, index: i32, snap_id: i32) -> i32 {
		if let Some(h) = self.history.get(&index) {
			let i = h.partition_point(|&(id, _)| id <= snap_id);
			if i > 0 {
				return h[i - 1].1;
			}
		}
		0
	}
}

// /**
//  * Your SnapshotArray object will be instantiated and called as such:
//  * let obj = SnapshotArray::new(length);
//  * obj.set(index, val);
//  * let ret_2: i32 = obj.snap();
//  * let ret_3: i32 = obj.get(index, snap_id);
//  */