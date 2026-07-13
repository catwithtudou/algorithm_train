pub struct Solution;

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut ans = Vec::new();

        for start in 1..=9 {
            let mut x = start;

            for next in start + 1..=9 {
                x = x * 10 + next;

                if x > high {
                    break;
                }

                if x >= low {
                    ans.push(x);
                }
            }
        }

        ans.sort_unstable();
        ans
    }
}