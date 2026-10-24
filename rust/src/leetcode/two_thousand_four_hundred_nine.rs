pub struct Solution;

impl Solution {
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        const DAYS_PRE_SUM: [i32; 13] =
            [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365];

        fn day_of_year(s: &str) -> i32 {
            let bytes = s.as_bytes();

            let month = ((bytes[0] - b'0') * 10 + bytes[1] - b'0') as usize;
            let day = ((bytes[3] - b'0') * 10 + bytes[4] - b'0') as i32;

            DAYS_PRE_SUM[month - 1] + day
        }

        let left = day_of_year(arrive_alice.as_str().max(arrive_bob.as_str()));
        let right = day_of_year(leave_alice.as_str().min(leave_bob.as_str()));

        (right - left + 1).max(0)
    }
}