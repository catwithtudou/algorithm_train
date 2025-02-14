pub struct Solution;

impl Solution {
    pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
        position.sort_unstable();

        let check = |d: i32| -> i32 {
            let mut cnt = 1;
            let mut pre = position[0];
            for &x in &position {
                if x >= pre + d {
                    cnt += 1;
                    pre = x;
                }
            }
            cnt
        };

        let mut left = 0;
        let mut right = (position.last().unwrap() - position[0]) / (m - 1) + 1;
        while left + 1 < right {
            let mid = left + (right - left) / 2;
            if check(mid) >= m {
                left = mid;
            } else {
                right = mid;
            }
        }
        left
    }
}
