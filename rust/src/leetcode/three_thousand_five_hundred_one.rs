pub struct Solution;

#[derive(Clone, Copy)]
struct ZeroRange {
    l: i32,
    r: i32, // 半开区间 [l, r)
}

impl ZeroRange {
    fn len(self) -> i32 {
        self.r - self.l
    }
}

struct SparseTable {
    table: Vec<Vec<i32>>,
}

impl SparseTable {
    fn new(ranges: &[ZeroRange]) -> Self {
        let n = ranges.len() - 1;
        let levels = usize::BITS as usize - n.leading_zeros() as usize;

        let mut table = vec![vec![0; n]; levels];

        for i in 0..n {
            table[0][i] = ranges[i].len() + ranges[i + 1].len();
        }

        for level in 1..levels {
            let len = 1usize << level;
            let half = len >> 1;

            for i in 0..=n - len {
                table[level][i] =
                    table[level - 1][i].max(table[level - 1][i + half]);
            }
        }

        Self { table }
    }

    // 查询半开区间 [l, r) 的最大值
    fn query(&self, l: usize, r: usize) -> i32 {
        if l >= r {
            return 0;
        }

        let len = r - l;
        let level =
            usize::BITS as usize - 1 - len.leading_zeros() as usize;
        let block = 1usize << level;

        self.table[level][l].max(self.table[level][r - block])
    }
}

impl Solution {
    pub fn max_active_sections_after_trade(
        s: String,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let bytes = s.as_bytes();
        let n = bytes.len() as i32;

        let mut total_ones = 0_i32;

        // 左侧哨兵
        let mut zero_ranges = vec![ZeroRange { l: -1, r: -1 }];

        let mut start = 0usize;

        for i in 0..bytes.len() {
            let is_segment_end =
                i + 1 == bytes.len() || bytes[i] != bytes[i + 1];

            if !is_segment_end {
                continue;
            }

            if bytes[i] == b'1' {
                total_ones += (i - start + 1) as i32;
            } else {
                zero_ranges.push(ZeroRange {
                    l: start as i32,
                    r: i as i32 + 1,
                });
            }

            start = i + 1;
        }

        // 右侧哨兵
        zero_ranges.push(ZeroRange {
            l: n + 1,
            r: n + 1,
        });

        let sparse_table = SparseTable::new(&zero_ranges);
        let range_count = zero_ranges.len();

        fn merge(left: i32, right: i32) -> i32 {
            if left > 0 && right > 0 {
                left + right
            } else {
                0
            }
        }

        let mut ans = Vec::with_capacity(queries.len());

        for query in queries {
            let ql = query[0];
            let qr = query[1] + 1;

            // 第一个左端点 >= ql 的零段
            let i = zero_ranges.partition_point(|range| range.l < ql);

            // 最后一个右端点 <= qr 的零段
            let j = zero_ranges
                .partition_point(|range| range.r <= qr)
                - 1;

            let mut max_gain = 0;

            if i <= j {
                // 查询区间中至少存在一个完整零段
                max_gain = sparse_table
                    .query(i, j)
                    .max(merge(
                        zero_ranges[i - 1].r - ql,
                        zero_ranges[i].len(),
                    ))
                    .max(merge(
                        qr - zero_ranges[j + 1].l,
                        zero_ranges[j].len(),
                    ));
            } else if i == j + 1 {
                // 查询区间横跨两个被边界截断的零段
                max_gain = merge(
                    zero_ranges[i - 1].r - ql,
                    qr - zero_ranges[j + 1].l,
                );
            }

            ans.push(total_ones + max_gain);
        }

        ans
    }
}