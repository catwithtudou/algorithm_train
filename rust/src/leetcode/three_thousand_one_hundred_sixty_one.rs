pub struct Solution;

use std::collections::BTreeSet;

struct SegTree {
    tree: Vec<i32>,
}

impl SegTree {
    fn new(n: usize) -> Self {
        Self {
            tree: vec![0; n * 4 + 5],
        }
    }

    fn update(&mut self, o: usize, l: usize, r: usize, i: usize, val: i32) {
        if l == r {
            self.tree[o] = val;
            return;
        }

        let mid = (l + r) / 2;

        if i <= mid {
            self.update(o * 2, l, mid, i, val);
        } else {
            self.update(o * 2 + 1, mid + 1, r, i, val);
        }

        self.tree[o] = self.tree[o * 2].max(self.tree[o * 2 + 1]);
    }

    fn query(&self, o: usize, l: usize, r: usize, right: usize) -> i32 {
        if r <= right {
            return self.tree[o];
        }

        let mid = (l + r) / 2;

        if right <= mid {
            return self.query(o * 2, l, mid, right);
        }

        self.tree[o * 2].max(self.query(o * 2 + 1, mid + 1, r, right))
    }
}

impl Solution {
    pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut m = 0;

        for q in &queries {
            m = m.max(q[1]);
        }

        m += 1;

        let m_usize = m as usize;

        let mut set = BTreeSet::new();
        set.insert(0);
        set.insert(m);

        let mut seg = SegTree::new(m_usize + 1);
        let mut ans = Vec::new();

        for q in queries {
            let x = q[1];

            // 等价于 Go 里的 set.Floor(x - 1)
            // 也就是找严格小于 x 的最大障碍位置
            let pre = *set.range(..x).next_back().unwrap();

            if q[0] == 1 {
                // 等价于 Go 里的 set.Ceiling(x)
                let nxt = *set.range(x..).next().unwrap();

                set.insert(x);

                seg.update(
                    1,
                    0,
                    m_usize,
                    x as usize,
                    x - pre,
                );

                seg.update(
                    1,
                    0,
                    m_usize,
                    nxt as usize,
                    nxt - x,
                );
            } else {
                let max_gap = seg
                    .query(1, 0, m_usize, pre as usize)
                    .max(x - pre);

                ans.push(max_gap >= q[2]);
            }
        }

        ans
    }
}