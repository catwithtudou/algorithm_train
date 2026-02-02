pub struct Solution;

use std::collections::BTreeMap;

struct Container {
    k: usize,
    st1: BTreeMap<i32, i32>,
    st2: BTreeMap<i32, i32>,
    sm: i64,
    st1_size: usize,
    st2_size: usize,
}

impl Container {
    fn new(k: usize) -> Self {
        Self {
            k,
            st1: BTreeMap::new(),
            st2: BTreeMap::new(),
            sm: 0,
            st1_size: 0,
            st2_size: 0,
        }
    }

    fn remove_one(map: &mut BTreeMap<i32, i32>, key: i32) -> bool {
        if let Some(count) = map.get_mut(&key) {
            *count -= 1;
            if *count == 0 {
                map.remove(&key);
            }
            true
        } else {
            false
        }
    }

    fn add_one(map: &mut BTreeMap<i32, i32>, key: i32) {
        *map.entry(key).or_insert(0) += 1;
    }

    fn first_key(map: &BTreeMap<i32, i32>) -> Option<i32> {
        map.keys().next().copied()
    }

    fn last_key(map: &BTreeMap<i32, i32>) -> Option<i32> {
        map.keys().next_back().copied()
    }

    fn adjust(&mut self) {
        while self.st1_size < self.k && !self.st2.is_empty() {
            if let Some(x) = Self::first_key(&self.st2) {
                Self::add_one(&mut self.st1, x);
                Self::remove_one(&mut self.st2, x);
                self.sm += x as i64;
                self.st1_size += 1;
                self.st2_size -= 1;
            }
        }

        while self.st1_size > self.k {
            if let Some(x) = Self::last_key(&self.st1) {
                Self::add_one(&mut self.st2, x);
                Self::remove_one(&mut self.st1, x);
                self.sm -= x as i64;
                self.st1_size -= 1;
                self.st2_size += 1;
            }
        }
    }

    // 插入元素 x
    fn add(&mut self, x: i32) {
        if !self.st2.is_empty() && x >= *self.st2.keys().next().unwrap() {
            Self::add_one(&mut self.st2, x);
            self.st2_size += 1;
        } else {
            Self::add_one(&mut self.st1, x);
            self.sm += x as i64;
            self.st1_size += 1;
        }
        self.adjust();
    }

    // 删除元素 x
    fn erase(&mut self, x: i32) {
        if Self::remove_one(&mut self.st1, x) {
            self.sm -= x as i64;
            self.st1_size -= 1;
        } else if Self::remove_one(&mut self.st2, x) {
            self.st2_size -= 1;
        }
        self.adjust();
    }

    // 前 k 小元素的和
    fn sum(&self) -> i64 {
        self.sm
    }
}

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
        let n = nums.len();
        let k = k as usize;
        let dist = dist as usize;

        let mut cnt = Container::new(k - 2);
        for i in 1..k - 1 {
            cnt.add(nums[i]);
        }

        let mut ans = cnt.sum() + nums[k - 1] as i64;
        for i in k..n {
            let j = i as i32 - dist as i32 - 1;
            if j > 0 {
                cnt.erase(nums[j as usize]);
            }
            cnt.add(nums[i - 1]);
            ans = ans.min(cnt.sum() + nums[i] as i64);
        }

        ans + nums[0] as i64
    }
}