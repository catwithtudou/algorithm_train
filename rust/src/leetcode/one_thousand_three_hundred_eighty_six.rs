pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_number_of_families(
        n: i32,
        reserved_seats: Vec<Vec<i32>>,
    ) -> i32 {
        let mut reserved = HashMap::<i32, i32>::new();

        for seat in reserved_seats {
            let row = seat[0];
            let col = seat[1];

            // 只有 2~9 号座位会影响四人家庭
            if (2..=9).contains(&col) {
                *reserved.entry(row).or_insert(0) |= 1 << (col - 2);
            }
        }

        // 完全没有相关预约的行，可以坐两个家庭
        let mut ans = (n - reserved.len() as i32) * 2;

        for mask in reserved.values() {
            // 2~5
            let left = mask & 0b0000_1111 == 0;

            // 4~7
            let middle = mask & 0b0011_1100 == 0;

            // 6~9
            let right = mask & 0b1111_0000 == 0;

            if left || middle || right {
                ans += 1;
            }
        }

        ans
    }
}