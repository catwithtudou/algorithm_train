pub struct Solution;

impl Solution {
    pub fn min_sum_square_diff(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        k1: i32,
        k2: i32,
    ) -> i64 {
        let mut diffs: Vec<i64> = nums1
            .into_iter()
            .zip(nums2)
            .map(|(a, b)| (a as i64 - b as i64).abs())
            .collect();

        let sum: i64 = diffs.iter().sum();
        let mut k = k1 as i64 + k2 as i64;

        if sum <= k {
            return 0;
        }

        let mut ans: i64 = diffs.iter().map(|&x| x * x).sum();

        diffs.sort_unstable_by(|a, b| b.cmp(a));
        diffs.push(0);

        for i in 0..diffs.len() - 1 {
            let v = diffs[i];
            let count = (i + 1) as i64;

            // 当前这个差值已经属于前 count 个需要统一处理的元素
            ans -= v * v;

            // 把前 count 个元素全部降低到下一个高度所需的操作数
            let cost = count * (v - diffs[i + 1]);

            if cost < k {
                k -= cost;
                continue;
            }

            // 剩余 k 次操作平均分配给前 count 个元素
            let v = v - k / count;
            let rem = k % count;

            // rem 个变成 v - 1，其余变成 v
            ans += rem * (v - 1) * (v - 1)
                + (count - rem) * v * v;

            break;
        }

        ans
    }
}