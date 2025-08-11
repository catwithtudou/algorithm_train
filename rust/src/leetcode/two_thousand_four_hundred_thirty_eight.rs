pub struct Solution;

impl Solution {
    pub fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        const MOD: i64 = 1_000_000_007;

        let mut powers = Vec::new();
        let mut n = n;

        while n > 0 {
            let lowbit = n & -n;
            powers.push(lowbit as i64);
            n ^= lowbit;
        }

        queries
            .iter()
            .map(|query| {
                let (left, right) = (query[0] as usize, query[1] as usize);
                powers[left..=right]
                    .iter()
                    .fold(1i64, |acc, &x| (acc * x) % MOD) as i32
            })
            .collect()
    }
}
