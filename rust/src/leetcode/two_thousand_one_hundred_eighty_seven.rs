pub struct Solution;

impl Solution {
    pub fn minimum_time(time: Vec<i32>, total_trips: i32) -> i64 {
        let (min_t, max_t) = (
            *time.iter().min().unwrap() as i64,
            *time.iter().max().unwrap() as i64,
        );
        let total_trips = total_trips as i64;
        let avg = (total_trips - 1) / time.len() as i64 + 1;
        let mut left = min_t * avg - 1;
        let mut right = (max_t * avg).min(min_t * total_trips);
        while left + 1 < right {
            let mid = (left + right) / 2;
            let mut sum = 0;
            for &t in &time {
                sum += mid / t as i64;
            }
            if sum >= total_trips {
                right = mid
            } else {
                left = mid
            }
        }
        right
    }
}
