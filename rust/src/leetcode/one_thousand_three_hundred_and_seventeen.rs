pub struct Solution;

use rand::Rng;

impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        loop {
            let a = rng.gen_range(1..n);
            if !a.to_string().contains('0') && !(n - a).to_string().contains('0') {
                return vec![a, n - a];
            }
        }
    }
}
