pub struct Solution;

impl Solution {
    pub fn count_permutations(complexity: Vec<i32>) -> i32 {
        const MOD: i64 = 1000000007;
        let mut ans = 1;
        for i in 1..complexity.len() {
            if complexity[i] <= complexity[0] {
                return 0;
            }
            ans = (ans * i as i64) % MOD;
        }
        ans as i32
    }
}
