pub struct Solution;

impl Solution {
    pub fn path_existence_queries(
        n: i32,
        nums: Vec<i32>,
        max_diff: i32,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = n as usize;

        // idx[i] 表示排序后第 i 个位置对应的原数组下标
        let mut idx: Vec<usize> = (0..n).collect();
        idx.sort_unstable_by_key(|&i| nums[i]);

        // rank[i] 表示原数组下标 i 在排序后的位置
        let mut rank = vec![0usize; n];
        for (i, &original_index) in idx.iter().enumerate() {
            rank[original_index] = i;
        }

        // n 的二进制位数
        let log = (usize::BITS - n.leading_zeros()) as usize;

        // pa[i][j]：
        // 从排序位置 i 出发，连续执行 2^j 次“跳到最左可达位置”后的位置
        let mut pa = vec![vec![0usize; log]; n];

        let mut left = 0usize;

        for (i, &original_index) in idx.iter().enumerate() {
            while nums[original_index] - nums[idx[left]] > max_diff {
                left += 1;
            }

            pa[i][0] = left;
        }

        // 倍增预处理
        for j in 0..log - 1 {
            for i in 0..n {
                pa[i][j + 1] = pa[pa[i][j]][j];
            }
        }

        let mut ans = vec![0i32; queries.len()];

        for (query_index, query) in queries.iter().enumerate() {
            let x = query[0] as usize;
            let y = query[1] as usize;

            if x == y {
                continue;
            }

            let mut l = rank[x];
            let mut r = rank[y];

            if l > r {
                std::mem::swap(&mut l, &mut r);
            }

            let mut steps = 0usize;

            // 尽可能多跳，但不能跳到 l 或 l 的左边
            for j in (0..log).rev() {
                if pa[r][j] > l {
                    r = pa[r][j];
                    steps += 1usize << j;
                }
            }

            if pa[r][0] > l {
                ans[query_index] = -1;
            } else {
                ans[query_index] = (steps + 1) as i32;
            }
        }

        ans
    }
}