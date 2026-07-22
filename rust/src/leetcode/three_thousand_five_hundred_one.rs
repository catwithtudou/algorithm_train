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
        // ranges 中包含左右两个哨兵，
        // 因此一共有 ranges.len() - 1 对相邻零段
        let n = ranges.len() - 1;
        let levels = (usize::BITS - n.leading_zeros()) as usize;

        let mut table = vec![vec![0; n]; levels];

        // table[0][i] 表示第 i、i+1 个零段长度之和
        for i in 0..n {
            table[0][i] = ranges[i].len() + ranges[i + 1].len();
        }

        for level in 1..levels {
            let half = 1usize << (level - 1);
            let len = 1usize << level;

            for i in 0..=n - len {
                table[level][i] =
                    table[level - 1][i].max(table[level - 1][i + half]);
            }
        }

        Self { table }
    }

    // 查询半开区间 [l, r) 中的最大值
    fn query(&self, l: usize, r: usize) -> i32 {
        if l >= r {
            return 0;
        }

        let len = r - l;
        let level = (usize::BITS - 1 - len.leading_zeros()) as usize;
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

        let mut total_ones = 0;

        // 左哨兵
        let mut zero_ranges = vec![ZeroRange { l: -1, r: -1 }];

        let mut start = 0usize;

        // 找出所有连续的零段
        for i in 0..bytes.len() {
            let is_end = i + 1 == bytes.len() || bytes[i] != bytes[i + 1];

            if !is_end {
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

        // 右哨兵
        zero_ranges.push(ZeroRange {
            l: n + 1,
            r: n + 1,
        });

        let sparse_table = SparseTable::new(&zero_ranges);

        // 只有两段都实际落在查询区间内，才能合并
        let merge = |left: i32, right: i32| -> i32 {
            if left > 0 && right > 0 {
                left + right
            } else {
                0
            }
        };

        queries
            .into_iter()
            .map(|query| {
                let ql = query[0];
                let qr = query[1] + 1; // 转换成半开区间 [ql, qr)

                // 第一个左端点 >= ql 的零段
                let i = zero_ranges.partition_point(|range| range.l < ql);

                // 最后一个右端点 <= qr 的零段
                let first_right_outside =
                    zero_ranges.partition_point(|range| range.r <= qr);
                let j = first_right_outside - 1;

                let max_gain = if i <= j {
                    // 至少有一个完整零段位于查询区间内

                    // 完整零段之间的最佳组合
                    let inner = sparse_table.query(i, j);

                    // 查询区间左边界截断了一个零段
                    let left_boundary = merge(
                        zero_ranges[i - 1].r - ql,
                        zero_ranges[i].len(),
                    );

                    // 查询区间右边界截断了一个零段
                    let right_boundary = merge(
                        qr - zero_ranges[j + 1].l,
                        zero_ranges[j].len(),
                    );

                    inner.max(left_boundary).max(right_boundary)
                } else {
                    // 查询区间内没有完整零段，
                    // 只能尝试合并左右两个被截断的零段
                    merge(
                        zero_ranges[i - 1].r - ql,
                        qr - zero_ranges[j + 1].l,
                    )
                };

                total_ones + max_gain
            })
            .collect()
    }
}