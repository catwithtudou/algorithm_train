pub struct Solution;

impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let s: i32 = nums.iter().sum();
        if s % 3 == 0 {
            return s;
        }

        let mut a: Vec<Vec<i32>> = vec![Vec::new(), Vec::new(), Vec::new()];
        for x in nums {
            a[(x % 3) as usize].push(x);
        }

        // 小的在前
        a[1].sort_unstable();
        a[2].sort_unstable();

        let mut ans = 0;

        match s % 3 {
            1 => {
                // 删 1 个余 1，或者删 2 个余 2
                if !a[1].is_empty() {
                    ans = s - a[1][0];
                }
                if a[2].len() >= 2 {
                    ans = ans.max(s - a[2][0] - a[2][1]);
                }
            }
            2 => {
                // 删 1 个余 2，或者删 2 个余 1
                if !a[2].is_empty() {
                    ans = s - a[2][0];
                }
                if a[1].len() >= 2 {
                    ans = ans.max(s - a[1][0] - a[1][1]);
                }
            }
            _ => {}
        }

        ans
    }
}
