pub struct Solution;

impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let n = points.len();
        let mut ans = 0;
        for i in 0..n - 2 {
            for j in i + 1..n - 1 {
                for k in j + 1..n {
                    let p1 = &points[i];
                    let p2 = &points[j];
                    let p3 = &points[k];
                    let x1 = p2[0] - p1[0];
                    let y1 = p2[1] - p1[1];
                    let x2 = p3[0] - p1[0];
                    let y2 = p3[1] - p1[1];
                    let area = (x1 * y2 - x2 * y1).abs();
                    ans = ans.max(area);
                }
            }
        }
        ans as f64 / 2.0
    }
}
