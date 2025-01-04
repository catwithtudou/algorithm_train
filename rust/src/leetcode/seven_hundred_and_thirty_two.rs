pub struct Solution;

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

struct MyCalendarThree {
    tree: NodeRef,
}

impl MyCalendarThree {
    fn new() -> Self {
        Self {
            tree: TreeNode::new(0, 1e9 as i32),
        }
    }

    fn book(&mut self, start_time: i32, end_time: i32) -> i32 {
        self.tree.modify((start_time, end_time - 1), 1);
        return self.tree.query((0, 1e9 as i32))
    }
}
