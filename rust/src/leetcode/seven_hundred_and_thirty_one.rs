pub struct Solution;

use std::collections::BTreeMap;
#[allow(dead_code)]
struct MyCalendarTwo {
    tm: BTreeMap<i32, i32>,
}
#[allow(dead_code)]
impl MyCalendarTwo {
    fn new() -> Self {
        MyCalendarTwo {
            tm: BTreeMap::new(),
        }
    }

    fn book(&mut self, start_time: i32, end_time: i32) -> bool {
        *self.tm.entry(start_time).or_default() += 1;
        *self.tm.entry(end_time).or_default() -= 1;
        if self
            .tm
            .iter()
            .fold((0, false), |(acc, ok), (_, v)| (acc + v, ok || acc + v > 2))
            .1
        {
            *self.tm.entry(start_time).or_default() -= 1;
            *self.tm.entry(end_time).or_default() += 1;
            return false;
        }
        true
    }
}

type NodeRef = Box<TreeNode>;
struct TreeNode {
    l: i32,
    r: i32,
    lazy: i32,
    max: i32,
    left: Option<NodeRef>,
    right: Option<NodeRef>,
}

impl TreeNode {
    fn new(l: i32, r: i32) -> NodeRef {
        Box::new(Self {
            l,
            r,
            lazy: 0,
            max: 0,
            left: None,
            right: None,
        })
    }

    fn left(&mut self) -> &mut NodeRef {
        self.left.get_or_insert(TreeNode::new(self.l, self.mid()))
    }

    fn right(&mut self) -> &mut NodeRef {
        self.right
            .get_or_insert(TreeNode::new(self.mid() + 1, self.r))
    }

    fn mid(&self) -> i32 {
        ((self.r - self.l) >> 1) + self.l
    }

    fn push_down(&mut self) {
        self.left().lazy += self.lazy;
        self.right().lazy += self.lazy;
        self.max += self.lazy;
        self.lazy = 0;
    }

    fn modify(&mut self, range: (i32, i32), k: i32) -> i32 {
        if range.0 <= self.l && self.r <= range.1 {
            self.lazy += k;
            self.max + self.lazy
        } else {
            self.push_down();
            if range.0 <= self.mid() {
                self.max = self.max.max(self.left().modify(range, k));
            }
            if self.mid() < range.1 {
                self.max = self.max.max(self.right().modify(range, k));
            }
            self.max
        }
    }

    fn query(&mut self, range: (i32, i32)) -> i32 {
        if range.0 <= self.l && self.r <= range.1 {
            self.max + self.lazy
        } else {
            self.push_down();
            let mut ret = i32::MIN;
            if range.0 <= self.mid() {
                ret = ret.max(self.left().query(range));
            }
            if self.mid() < range.1 {
                ret = ret.max(self.right().query(range));
            }
            ret
        }
    }
}

struct MyCalendarTwoBySegmentTree {
    tree: NodeRef,
}

impl MyCalendarTwoBySegmentTree {
    fn new() -> Self {
        Self {
            tree: TreeNode::new(0, 1e9 as i32),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        if self.tree.query((start, end - 1)) >= 2 {
            false
        } else {
            self.tree.modify((start, end - 1), 1);
            true
        }
    }
}
