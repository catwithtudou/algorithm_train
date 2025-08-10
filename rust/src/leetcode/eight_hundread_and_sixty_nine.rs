pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        fn get_power_of_two_sorted_set() -> HashSet<String> {
            let mut set = HashSet::new();
            let mut power = 1i64; // 使用 i64 避免溢出

            while power <= 1_000_000_000 {
                let sorted_str = int_to_sorted_str(power as i32);
                set.insert(sorted_str);
                power <<= 1;
            }

            set
        }

        fn int_to_sorted_str(n: i32) -> String {
            let mut chars: Vec<char> = n.to_string().chars().collect();
            chars.sort_unstable();
            chars.into_iter().collect()
        }

        let power_of_two_set = get_power_of_two_sorted_set();
        let sorted_n = int_to_sorted_str(n);

        power_of_two_set.contains(&sorted_n)
    }
}
