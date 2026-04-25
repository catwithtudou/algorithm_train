pub struct Solution;

impl Solution {
    pub fn max_distance(side: i32, points: Vec<Vec<i32>>, k: i32) -> i32 {
        let side = side as i64;
        let k = k as usize;

        let mut a: Vec<i64> = points
            .iter()
            .map(|p| {
                let x = p[0] as i64;
                let y = p[1] as i64;

                if x == 0 {
                    y
                } else if y == side {
                    side + x
                } else if x == side {
                    3 * side - y
                } else {
                    4 * side - x
                }
            })
            .collect();

        a.sort_unstable();

        let perimeter = 4 * side;

        let can_place = |dist: i64| -> bool {
            'next_start: for start_idx in 0..a.len() {
                let start = a[start_idx];
                let mut cur = start;
                let mut idx = start_idx;

                for _ in 1..k {
                    let search_start = idx + 1;

                    let offset = a[search_start..].partition_point(|&v| v - cur < dist);
                    idx = search_start + offset;

                    if idx == a.len() || a[idx] > start + perimeter - dist {
                        continue 'next_start;
                    }

                    cur = a[idx];
                }

                return true;
            }

            false
        };

        let mut left = 1_i64;
        let mut right = perimeter / k as i64 + 1;

        while left < right {
            let mid = (left + right) / 2;

            if can_place(mid) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        (left - 1) as i32
    }
}