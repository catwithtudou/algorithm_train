pub struct Solution;

impl Solution {
    pub fn diagonal_prime(nums: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;

        let n = nums.len();

        for (i, row) in nums.iter().enumerate() {
            for x in [row[i], row[n - 1 - i]] {
                if x>ans && Self::is_prime(x) {
                    ans = x;
                }
            }
        }

        ans
    }

    fn is_prime(n: i32) -> bool {
        for i in 2..=((n as f64).sqrt() as i32) {
            if n % i == 0 {
                return false;
            }
        }
        n >= 2
    }
}
