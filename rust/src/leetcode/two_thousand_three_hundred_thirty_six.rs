use std::collections::BTreeSet;

struct SmallestInfiniteSet {
    idx: i32,
    coll: BTreeSet<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {
    fn new() -> Self {
        SmallestInfiniteSet { idx: 1, coll: BTreeSet::new() }
    }

    fn pop_smallest(&mut self) -> i32 {
        if self.coll.is_empty() {
            let ans = self.idx;
            self.idx += 1;
            return ans;
        }
        let x = *self.coll.iter().next().unwrap();
        self.coll.remove(&x);
        x
    }

    fn add_back(&mut self, num: i32) {
        if num < self.idx {
            self.coll.insert(num);
        }
    }
}

/**
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * let obj = SmallestInfiniteSet::new();
 * let ret_1: i32 = obj.pop_smallest();
 * obj.add_back(num);
 */