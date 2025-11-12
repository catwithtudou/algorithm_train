pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut gcd_all = 0;
        let mut cnt1 = 0usize;

        for &x in &nums {
            gcd_all = Self::gcd(gcd_all, x);
            if x == 1 {
                cnt1 += 1;
            }
        }

        if gcd_all > 1 {
            return -1;
        }

        if cnt1 > 0 {
            return (n - cnt1) as i32;
        }

        let mut min_size = n;
        for i in 0..n {
            let mut g = 0;
            for (j, &x) in nums[i..].iter().enumerate() {
                g = Self::gcd(g, x);
                if g == 1 {
                    min_size = min_size.min(j);
                    break;
                }
            }
        }

        (min_size + n - 1) as i32
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let t = a % b;
            a = b;
            b = t;
        }
        a
    }
}
