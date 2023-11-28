use std::collections::VecDeque;

pub struct Solution;

struct FrontMiddleBackQueue {
    left: VecDeque<i32>,
    right: VecDeque<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FrontMiddleBackQueue {
    fn new() -> Self {
        FrontMiddleBackQueue {
            left: VecDeque::new(),
            right: VecDeque::new(),
        }
    }

    fn balance(&mut self) {
        if self.left.len() > self.right.len() {
            self.right.push_front(self.left.pop_back().unwrap());
        } else if self.right.len() > self.left.len() + 1 {
            self.left.push_back(self.right.pop_front().unwrap());
        }
    }

    fn push_front(&mut self, val: i32) {
        self.left.push_front(val);
        self.balance();
    }

    fn push_middle(&mut self, val: i32) {
        if self.left.len() < self.right.len() {
            self.left.push_back(val);
        } else {
            self.right.push_front(val);
        }
    }

    fn push_back(&mut self, val: i32) {
        self.right.push_back(val);
        self.balance();
    }

    fn pop_front(&mut self) -> i32 {
        if self.right.is_empty() {
            return -1;
        }
        let val = if self.left.is_empty() {
            self.right.pop_front().unwrap()
        } else {
            self.left.pop_front().unwrap()
        };
        self.balance();
        val
    }

    fn pop_middle(&mut self) -> i32 {
        if self.right.is_empty() {
            return -1;
        }
        if self.left.len() == self.right.len() {
            self.left.pop_back().unwrap()
        } else {
            self.right.pop_front().unwrap()
        }
    }

    fn pop_back(&mut self) -> i32 {
        if self.right.is_empty() {
            return -1;
        }
        let val = self.right.pop_back().unwrap();
        self.balance();
        val
    }
}
