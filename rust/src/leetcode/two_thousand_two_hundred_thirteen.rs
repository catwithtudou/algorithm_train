pub struct Solution;

#[derive(Clone, Copy, Default)]
struct SegData {
    mx: usize,
    pre: usize,
    suf: usize,
}

struct SegmentTree {
    s: Vec<u8>,
    tree: Vec<SegData>,
}

impl SegmentTree {
    fn new(s: Vec<u8>) -> Self {
        let n = s.len();

        let mut seg = Self {
            s,
            tree: vec![SegData::default(); n * 4],
        };

        seg.build(1, 0, n - 1);
        seg
    }

    fn maintain(
        &mut self,
        node: usize,
        l: usize,
        mid: usize,
        r: usize,
    ) {
        let left = self.tree[node * 2];
        let right = self.tree[node * 2 + 1];

        let mut mx = left.mx.max(right.mx);
        let mut pre = left.pre;
        let mut suf = right.suf;

        // 左右两个区间交界处字符相同，可以拼接
        if self.s[mid] == self.s[mid + 1] {
            mx = mx.max(left.suf + right.pre);

            // 左区间整体都是同一个字符
            if pre == mid - l + 1 {
                pre += right.pre;
            }

            // 右区间整体都是同一个字符
            if suf == r - mid {
                suf += left.suf;
            }
        }

        self.tree[node] = SegData { mx, pre, suf };
    }

    fn build(
        &mut self,
        node: usize,
        l: usize,
        r: usize,
    ) {
        if l == r {
            self.tree[node] = SegData {
                mx: 1,
                pre: 1,
                suf: 1,
            };
            return;
        }

        let mid = (l + r) / 2;

        self.build(node * 2, l, mid);
        self.build(node * 2 + 1, mid + 1, r);

        self.maintain(node, l, mid, r);
    }

    fn update(
        &mut self,
        node: usize,
        l: usize,
        r: usize,
        index: usize,
        ch: u8,
    ) {
        if l == r {
            self.s[index] = ch;
            return;
        }

        let mid = (l + r) / 2;

        if index <= mid {
            self.update(node * 2, l, mid, index, ch);
        } else {
            self.update(node * 2 + 1, mid + 1, r, index, ch);
        }

        self.maintain(node, l, mid, r);
    }

    fn max_repeating(&self) -> usize {
        self.tree[1].mx
    }
}

impl Solution {
    pub fn longest_repeating(
        s: String,
        query_characters: String,
        query_indices: Vec<i32>,
    ) -> Vec<i32> {
        let n = s.len();
        let mut seg = SegmentTree::new(s.into_bytes());

        let query_chars = query_characters.as_bytes();

        let mut ans = Vec::with_capacity(query_indices.len());

        for (k, &index) in query_indices.iter().enumerate() {
            seg.update(
                1,
                0,
                n - 1,
                index as usize,
                query_chars[k],
            );

            ans.push(seg.max_repeating() as i32);
        }

        ans
    }
}