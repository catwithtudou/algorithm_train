pub struct Solution;

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let mut seg_tree = SegmentTree::new(baskets);
        let mut ans = 0;

        for fruit in fruits {
            if seg_tree.find_first_and_update(1, 0, seg_tree.n - 1, fruit) < 0 {
                ans += 1;
            }
        }

        ans
    }
}

struct SegmentTree {
    tree: Vec<i32>,
    n: usize,
}

impl SegmentTree {
    fn new(baskets: Vec<i32>) -> Self {
        let n = baskets.len();
        // 修正线段树大小计算：对于n个元素，需要4*n的空间就足够了
        let size = if n == 0 { 1 } else { 4 * n };
        let tree = vec![-1; size];

        let mut seg_tree = SegmentTree { tree, n };
        if n > 0 {
            seg_tree.build(&baskets, 1, 0, n - 1);
        }
        seg_tree
    }

    fn maintain(&mut self, node: usize) {
        self.tree[node] = self.tree[node << 1].max(self.tree[node << 1 | 1]);
    }

    fn build(&mut self, baskets: &[i32], node: usize, left: usize, right: usize) {
        if left == right {
            self.tree[node] = baskets[left];
            return;
        }

        let mid = (left + right) >> 1;
        self.build(baskets, node << 1, left, mid);
        self.build(baskets, node << 1 | 1, mid + 1, right);
        self.maintain(node);
    }

    fn find_first_and_update(&mut self, node: usize, left: usize, right: usize, x: i32) -> i32 {
        if self.tree[node] < x {
            return -1;
        }

        if left == right {
            self.tree[node] = -1;
            return left as i32;
        }

        let mid = (left + right) >> 1;
        let mut result = self.find_first_and_update(node << 1, left, mid, x);

        if result < 0 {
            result = self.find_first_and_update(node << 1 | 1, mid + 1, right, x);
        }

        self.maintain(node);
        result
    }
}
