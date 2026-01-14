pub struct Solution;

use std::cmp::min;

impl Solution {
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        #[derive(Clone, Copy)]
        struct Event {
            y: i32,
            lx: i32,
            rx: i32,
            delta: i32,
        }

        #[derive(Clone, Copy, Default)]
        struct Node {
            l: i32,
            r: i32,
            min_cover: i32,
            min_cover_len: i64,
            todo: i32,
        }

        struct SegTree {
            tr: Vec<Node>,
        }

        impl SegTree {
            fn new(n: usize) -> Self {
                // 4*n 足够
                SegTree {
                    tr: vec![Node::default(); 4 * n + 5],
                }
            }

            fn maintain(&mut self, o: usize) {
                let lo = self.tr[o << 1];
                let ro = self.tr[o << 1 | 1];
                let mn = min(lo.min_cover, ro.min_cover);

                self.tr[o].min_cover = mn;
                let mut len = 0i64;
                if lo.min_cover == mn {
                    len += lo.min_cover_len;
                }
                if ro.min_cover == mn {
                    len += ro.min_cover_len;
                }
                self.tr[o].min_cover_len = len;
            }

            fn do_add(&mut self, o: usize, v: i32) {
                self.tr[o].min_cover += v;
                self.tr[o].todo += v;
            }

            fn spread(&mut self, o: usize) {
                let v = self.tr[o].todo;
                if v != 0 {
                    self.do_add(o << 1, v);
                    self.do_add(o << 1 | 1, v);
                    self.tr[o].todo = 0;
                }
            }

            fn build(&mut self, xs: &Vec<i32>, o: usize, l: i32, r: i32) {
                self.tr[o].l = l;
                self.tr[o].r = r;
                self.tr[o].todo = 0;

                if l == r {
                    // 叶子代表 [xs[l], xs[l+1]) 这一段
                    let li = l as usize;
                    self.tr[o].min_cover = 0;
                    self.tr[o].min_cover_len = (xs[li + 1] - xs[li]) as i64;
                    return;
                }

                let m = (l + r) >> 1;
                self.build(xs, o << 1, l, m);
                self.build(xs, o << 1 | 1, m + 1, r);
                self.maintain(o);
            }

            fn update(&mut self, o: usize, l: i32, r: i32, v: i32) {
                let nl = self.tr[o].l;
                let nr = self.tr[o].r;

                if l <= nl && nr <= r {
                    self.do_add(o, v);
                    return;
                }

                self.spread(o);
                let m = (nl + nr) >> 1;
                if l <= m {
                    self.update(o << 1, l, r, v);
                }
                if m < r {
                    self.update(o << 1 | 1, l, r, v);
                }
                self.maintain(o);
            }
        }

        fn lower_bound(arr: &Vec<i32>, x: i32) -> usize {
            let mut lo = 0usize;
            let mut hi = arr.len();
            while lo < hi {
                let mid = (lo + hi) >> 1;
                if arr[mid] < x {
                    lo = mid + 1;
                } else {
                    hi = mid;
                }
            }
            lo
        }

        // --- 构造 xs + events ---
        let n_sq = squares.len();
        let m = n_sq * 2;
        let mut xs: Vec<i32> = Vec::with_capacity(m);
        let mut events: Vec<Event> = Vec::with_capacity(m);

        for sq in squares {
            let lx = sq[0];
            let y = sq[1];
            let l = sq[2];
            let rx = lx + l;

            xs.push(lx);
            xs.push(rx);
            events.push(Event {
                y,
                lx,
                rx,
                delta: 1,
            });
            events.push(Event {
                y: y + l,
                lx,
                rx,
                delta: -1,
            });
        }

        xs.sort_unstable();
        xs.dedup();

        // len(xs) 个横坐标点 -> len(xs)-1 个小段
        let seg_n = xs.len() - 1;
        let mut st = SegTree::new(seg_n);
        st.build(&xs, 1, 0, (seg_n as i32) - 1);

        // 扫描线：按 y 排序
        events.sort_unstable_by(|a, b| a.y.cmp(&b.y));

        // records[i] = (tot_area_before, covered_len_at_segment_i)
        let mut records: Vec<(i128, i64)> = vec![(0, 0); m - 1];

        let total_len: i64 = (xs[xs.len() - 1] - xs[0]) as i64;
        let mut tot_area: i128 = 0;

        for i in 0..(m - 1) {
            let e = events[i];

            // 离散化到叶子段索引： [lx, rx) -> [l, r]（闭区间，表示若干段）
            let l = lower_bound(&xs, e.lx) as i32;
            let r = (lower_bound(&xs, e.rx) as i32) - 1;

            st.update(1, l, r, e.delta);

            // 当前被至少覆盖一次的总长度
            let mut covered_len = total_len;
            if st.tr[1].min_cover == 0 {
                covered_len -= st.tr[1].min_cover_len;
            }

            records[i] = (tot_area, covered_len);

            // 新增面积 = covered_len * (events[i+1].y - events[i].y)
            let dy = (events[i + 1].y - e.y) as i128;
            tot_area += (covered_len as i128) * dy;
        }

        // 在 records 里找最后一个 area < tot_area/2 的段
        // 等价：找第一个 records[i].area*2 >= tot_area，再 -1
        let mut lo = 0usize;
        let mut hi = records.len();
        while lo < hi {
            let mid = (lo + hi) >> 1;
            if records[mid].0 * 2 >= tot_area {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        let idx = if lo == 0 { 0 } else { lo - 1 };

        // 在线性段内插值：
        // y = events[idx].y + (tot_area - 2*area_before) / (2*covered_len)
        let base_y = events[idx].y as f64;
        let numerator = (tot_area - records[idx].0 * 2) as f64;
        let denom = (records[idx].1 as f64) * 2.0;

        base_y + numerator / denom
    }
}
