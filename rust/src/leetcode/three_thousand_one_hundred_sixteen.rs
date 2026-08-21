pub struct Solution;

impl Solution {
    pub fn find_kth_smallest(coins: Vec<i32>, k: i32) -> i64 {
        fn gcd(mut a: i64, mut b: i64) -> i64 {
            while b != 0 {
                (a, b) = (b, a % b);
            }
            a
        }

        fn lcm(a: i64, b: i64) -> i64 {
            a / gcd(a, b) * b
        }

        let n = coins.len();

        // 预处理每个子集的最小公倍数
        let mut subset_lcm = vec![1_i64; 1 << n];

        for i in 0..n {
            let bit = 1 << i;
            let x = coins[i] as i64;

            for mask in 0..bit {
                subset_lcm[bit | mask] = lcm(subset_lcm[mask], x);
            }
        }

        let min_coin = *coins.iter().min().unwrap() as i64;

        let mut left = 1_i64;
        let mut right = min_coin * k as i64;

        while left < right {
            let mid = (left + right) / 2;

            let mut count = 0_i64;

            for mask in 1..(1 << n) {
                let mut c = mid / subset_lcm[mask];

                // 容斥：
                // 奇数个集合取正，偶数个集合取负
                if (mask as u32).count_ones() % 2 == 0 {
                    c = -c;
                }

                count += c;
            }

            if count >= k as i64 {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
    }
}