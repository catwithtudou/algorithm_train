pub struct Solution;

use std::collections::BinaryHeap;

#[derive(Clone, Copy)]
struct PairMinMax {
    min: i32,
    max: i32,
}

fn op(a: PairMinMax, b: PairMinMax) -> PairMinMax {
    PairMinMax {
        min: a.min.min(b.min),
        max: a.max.max(b.max),
    }
}

struct SparseTable {
    st: Vec<Vec<PairMinMax>>,
}

impl SparseTable {
    fn bit_len(x: usize) -> usize {
        usize::BITS as usize - x.leading_zeros() as usize
    }

    fn log2(x: usize) -> usize {
        usize::BITS as usize - 1 - x.leading_zeros() as usize
    }

    fn new(nums: &[i32]) -> Self {
        let n = nums.len();
        let w = Self::bit_len(n);

        let default = PairMinMax { min: 0, max: 0 };
        let mut st = vec![vec![default; n]; w];

        for i in 0..n {
            st[0][i] = PairMinMax {
                min: nums[i],
                max: nums[i],
            };
        }

        for j in 1..w {
            let len = 1usize << j;
            let half = 1usize << (j - 1);

            for i in 0..=n - len {
                st[j][i] = op(st[j - 1][i], st[j - 1][i + half]);
            }
        }

        Self { st }
    }

    // 查询半开区间 [l, r) 的 max - min
    fn query(&self, l: usize, r: usize) -> i32 {
        let len = r - l;
        let k = Self::log2(len);
        let block = 1usize << k;

        let p = op(self.st[k][l], self.st[k][r - block]);

        p.max - p.min
    }
}

impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let st = SparseTable::new(&nums);

        // BinaryHeap 默认就是最大堆
        // 元素含义：(d, l, r)
        // d = nums[l..r] 的 max - min
        let mut init = Vec::with_capacity(n);

        for i in 0..n {
            init.push((st.query(i, n), i, n));
        }

        let mut heap = BinaryHeap::from(init);

        let mut ans = 0_i64;
        let mut k = k;

        while k > 0 {
            let Some((d, l, r)) = heap.pop() else {
                break;
            };

            if d <= 0 {
                break;
            }

            ans += d as i64;
            k -= 1;

            let new_r = r - 1;

            if new_r > l {
                let new_d = st.query(l, new_r);

                if new_d > 0 {
                    heap.push((new_d, l, new_r));
                }
            }
        }

        ans
    }
}