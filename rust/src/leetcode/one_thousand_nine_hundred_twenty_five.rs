pub struct Solution;

impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        let mut ans = 0;
        for a in 1..n {
            for b in 1..a {
                if a * a + b * b > n * n {
                    break;
                }
                let c2 = a * a + b * b;
                let c = (c2 as f64).sqrt() as i32;
                if c * c == c2 {
                    ans += 1;
                }
            }
        }
        ans * 2
    }
}
