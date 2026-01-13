pub struct Solution;

impl Solution {
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let mut total_area: i64 = 0;
        let mut max_y: i32 = 0;
        for sq in &squares {
            let l = sq[2] as i64;
            total_area += l * l;
            max_y = max_y.max(sq[1] + sq[2]);
        }
        let check = |y: f64| -> bool {
            let mut area: f64 = 0.0;
            for sq in &squares {
                let yi = sq[1] as f64;
                if yi < y {
                    let l = sq[2] as f64;
                    let h = (y - yi).min(l); // minFloat64(y-yi, l)
                    area += l * h;
                }
            }
            area >= (total_area as f64) / 2.0
        };

        let (mut left, mut right) = (0.0, max_y as f64);
        let scaled: u64 = ((max_y as f64) * 1e5) as u64;
        let iters: u32 = if scaled == 0 {
            0
        } else {
            64 - scaled.leading_zeros()
        };

        for _ in 0..iters {
            let mid = (left + right) / 2.0;
            if check(mid) {
                right = mid;
            } else {
                left = mid;
            }
        }

        (left + right) / 2.0
    }
}
