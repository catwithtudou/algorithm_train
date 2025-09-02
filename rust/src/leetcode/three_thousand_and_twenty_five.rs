pub struct Solution;

impl Solution {
    pub fn number_of_pairs(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by(|a, b| {
            if a[0] != b[0] {
                a[0].cmp(&b[0])
            } else {
                b[1].cmp(&a[1])
            }
        });
        let mut ans = 0;
        for i in 0..points.len() {
            let p = &points[i];
            let mut max_y = i32::MIN;
            for j in i + 1..points.len() {
                let q = &points[j];
                if q[1] <= p[1] && q[1] > max_y {
                    max_y = q[1];
                    ans += 1;
                }
            }
        }
        ans
    }
}
