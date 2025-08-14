use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn check_powers_of_three(n: i32) -> bool {
        let mut powers = Vec::new();
        let mut x = 1;
        while x <= n {
            powers.push(x);
            if x > n / 3 {
                break;
            }
            x *= 3;
        }

        let mut queue = VecDeque::new();
        queue.push_back(n);

        for power in powers {
            let size = queue.len();
            for _ in 0..size {
                if let Some(cur) = queue.pop_front() {
                    if cur - power == 0 {
                        return true;
                    }
                    queue.push_back(cur);
                    if cur >= power {
                        queue.push_back(cur - power);
                    }
                }
            }
        }

        false
    }
}
