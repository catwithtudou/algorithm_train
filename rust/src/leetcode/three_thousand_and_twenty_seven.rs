pub struct Solution;

impl Solution {
    pub fn number_of_pairs(mut points: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;

        points.sort_unstable_by_key(|p| (p[0], -p[1]));

        for (i, p) in points.iter().enumerate() {
            let mut max_y = i32::MIN;
            let y1 = p[1];
            for q in &points[i + 1..] {
                let y2 = q[1];
                if y2 <= y1 && y2 > max_y {
                    max_y = y2;
                    ans += 1;
                }
                if max_y == y1 {
                    break;
                }
            }
        }

        ans
    }
}
