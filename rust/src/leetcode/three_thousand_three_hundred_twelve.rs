pub struct Solution;

impl Solution {
    pub fn gcd_values(nums: Vec<i32>, queries: Vec<i64>) -> Vec<i32> {
        let max_value = nums.iter().copied().max().unwrap() as usize;

        // cnt_x[x]：数字 x 在 nums 中出现的次数
        let mut cnt_x = vec![0_i64; max_value + 1];

        for x in nums {
            cnt_x[x as usize] += 1;
        }

        // cnt_gcd[g]：GCD 恰好等于 g 的数对数量
        let mut cnt_gcd = vec![0_i64; max_value + 1];

        for g in (1..=max_value).rev() {
            let mut count = 0_i64;

            // 统计 nums 中有多少个数是 g 的倍数
            for multiple in (g..=max_value).step_by(g) {
                count += cnt_x[multiple];

                // 减去 GCD 为 2g、3g……的数对
                cnt_gcd[g] -= cnt_gcd[multiple];
            }

            // 所有两个数都能被 g 整除的数对数量
            cnt_gcd[g] += count * (count - 1) / 2;
        }

        // 转成前缀和：
        // cnt_gcd[g] 表示 GCD <= g 的数对数量
        for g in 1..=max_value {
            cnt_gcd[g] += cnt_gcd[g - 1];
        }

        queries
            .into_iter()
            .map(|q| {
                // 找到第一个 cnt_gcd[g] >= q + 1 的位置
                // 即排序后第 q 个数对 GCD 的值
                cnt_gcd.partition_point(|&count| count <= q) as i32
            })
            .collect()
    }
}