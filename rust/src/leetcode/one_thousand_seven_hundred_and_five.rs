pub struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
        let n = apples.len();
        let mut hp: BinaryHeap<(i32, i32)> = BinaryHeap::new();
        let mut ans = 0;
        for (i, (num, day)) in apples.into_iter().zip(days).enumerate() {
            while let Some(&(r_day, _)) = hp.peek() {
                if -r_day > i as i32 {
                    break;
                }
                hp.pop();
            }
            if num > 0 {
                hp.push((-(i as i32 + day), num));
            }
            if let Some(mut p) = hp.peek_mut() {
                ans += 1;
                p.1 -= 1;
            }
            if let Some(&(_, num)) = hp.peek() {
                if num == 0 {
                    hp.pop();
                }
            }
        }

        let mut i = n as i32;
        loop {
            while let Some(&(r_day, _)) = hp.peek() {
                if -r_day > i {
                    break;
                }
                hp.pop();
            }
            if let Some((r_day, num)) = hp.pop() {
                let k = num.min(-r_day - i);
                ans += k;
                i += k;
            } else {
                return ans;
            }
        }
    }
}
