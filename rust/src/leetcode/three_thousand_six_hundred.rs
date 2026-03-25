pub struct Solution;

struct UnionFind {
    fa: Vec<usize>,
    cc: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            fa: (0..n).collect(),
            cc: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.fa[x] != x {
            // 先递归找到根，再做路径压缩，避免借用冲突
            let root = self.find(self.fa[x]);
            self.fa[x] = root;
        }
        self.fa[x]
    }

    fn merge(&mut self, from: usize, to: usize) -> bool {
        let x = self.find(from);
        let y = self.find(to);
        if x == y {
            return false;
        }
        self.fa[x] = y;
        self.cc -= 1;
        true
    }
}

impl Solution {
    pub fn max_stability(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = n as usize;
        let mut must_uf = UnionFind::new(n); // 必选边
        let mut all_uf = UnionFind::new(n);  // 所有边
        let mut min_s = i32::MAX;
        let mut max_s = 0i32;

        for e in &edges {
            let (x, y, s, must) = (e[0] as usize, e[1] as usize, e[2], e[3]);
            if must > 0 && !must_uf.merge(x, y) {
                return -1;
            }
            all_uf.merge(x, y);
            min_s = min_s.min(s);
            max_s = max_s.max(s);
        }

        if all_uf.cc > 1 {
            // 图不连通
            return -1;
        }

        let check = |low: i32| -> bool {
            let mut u = UnionFind::new(n);
            for e in &edges {
                let (x, y, s, must) = (e[0] as usize, e[1] as usize, e[2], e[3]);
                if must > 0 && s < low {
                    // 必选边的边权太小
                    return false;
                }
                if must > 0 || s >= low {
                    u.merge(x, y);
                }
            }

            let mut left_k = k;
            for e in &edges {
                if left_k == 0 || u.cc == 1 {
                    break;
                }
                let (x, y, s, must) = (e[0] as usize, e[1] as usize, e[2], e[3]);
                if must == 0 && s < low && s * 2 >= low && u.merge(x, y) {
                    left_k -= 1;
                }
            }
            u.cc == 1
        };

        let mut left = min_s;
        let mut right = max_s * 2 + 1;
        while left + 1 < right {
            let mid = left + (right - left) / 2;
            if check(mid) {
                left = mid;
            } else {
                right = mid;
            }
        }

        left
    }
}