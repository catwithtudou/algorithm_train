use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        let float_key = |f: f64| -> u64 {
            if f == 0.0 {
                (0.0f64).to_bits()
            } else {
                f.to_bits()
            }
        };

        let mut groups: HashMap<u64, Vec<f64>> = HashMap::new(); // 斜率->截距
        let mut groups2: HashMap<(i32, i32), Vec<f64>> = HashMap::new(); // 中点->斜率

        for (i, p) in points.iter().enumerate() {
            let (x, y) = (p[0], p[1]);

            for q in &points[..i] {
                let (x2, y2) = (q[0], q[1]);

                let (dy, dx) = ((y2 - y) as f64, (x2 - x) as f64);

                let mut k = f64::MAX;
                let mut b = x as f64;

                if dx != 0.0 {
                    k = dy / dx;
                    b = (y as f64 * dx - dy * x as f64) / dx;
                }

                groups.entry(float_key(k)).or_default().push(b);

                let mid = (x + x2, y + y2);
                groups2.entry(mid).or_default().push(k);
            }
        }

        let mut ans = 0;

        for (_, g) in groups {
            if g.len() < 2 {
                continue;
            }

            let mut cnt: HashMap<u64, i32> = HashMap::new();
            for &val in &g {
                *cnt.entry(float_key(val)).or_insert(0) += 1;
            }

            let mut s = 0;
            for c in cnt.values() {
                ans += s * c;
                s += c;
            }
        }

        for (_, g) in groups2 {
            if g.len() < 2 {
                continue;
            }
            // 统计每个斜率出现的次数
            let mut cnt: HashMap<u64, i32> = HashMap::new();
            for &val in &g {
                *cnt.entry(float_key(val)).or_insert(0) += 1;
            }

            let mut s = 0;
            for c in cnt.values() {
                ans -= s * c; // 减去多统计的部分
                s += c;
            }
        }

        ans
    }
}
