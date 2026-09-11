pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn total_numbers(digits: Vec<i32>) -> i32 {
        let mut set = HashSet::new();

         for (i, &x) in digits.iter().enumerate() {
            if x % 2 != 0 {
                continue;
            }

            for (j, &y) in digits.iter().enumerate() {
                if i == j {
                    continue;
                }

                for (k, &z) in digits.iter().enumerate() {
                    if z == 0 || k == i || k == j {
                        continue;
                    }

                    set.insert(100 * z + 10 * y + x);
                }
            }
        }


        set.len() as i32
    }
}