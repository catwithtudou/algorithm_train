pub struct Solution;

impl Solution {
    pub fn subsequence_pair_count(nums: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        fn gcd(mut a: usize, mut b: usize) -> usize {
            while b != 0 {
                (a, b) = (b, a % b);
            }
            a
        }

        fn dfs(
            i: usize,
            gcd1: usize,
            gcd2: usize,
            nums: &[usize],
            memo: &mut [Vec<Vec<i32>>],
        ) -> i32 {
            if i == nums.len() {
                return if gcd1 == gcd2 { 1 } else { 0 };
            }

            if memo[i][gcd1][gcd2] != -1 {
                return memo[i][gcd1][gcd2];
            }

            let x = nums[i];

            // nums[i] 有三种选择：
            // 1. 不放入任何子序列
            // 2. 放入第一个子序列
            // 3. 放入第二个子序列
            let skip = dfs(i + 1, gcd1, gcd2, nums, memo) as i64;
            let put_first = dfs(i + 1, gcd(gcd1, x), gcd2, nums, memo) as i64;
            let put_second = dfs(i + 1, gcd1, gcd(gcd2, x), nums, memo) as i64;

            let res = ((skip + put_first + put_second) % MOD) as i32;
            memo[i][gcd1][gcd2] = res;

            res
        }

        let nums: Vec<usize> = nums.into_iter().map(|x| x as usize).collect();
        let n = nums.len();
        let max_num = nums.iter().copied().max().unwrap_or(0);

        let mut memo = vec![vec![vec![-1; max_num + 1]; max_num + 1]; n];

        let result = dfs(0, 0, 0, &nums, &mut memo) as i64;

        // 去掉两个子序列都为空的方案
        ((result - 1 + MOD) % MOD) as i32
    }
}