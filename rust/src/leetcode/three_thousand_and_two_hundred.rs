pub struct Solution;

impl Solution {
    pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
        let mut cnt = vec![0; 2];
        let mut i = 1;
        loop {
            cnt[i % 2] += i as i32;
            if (cnt[0] > red || cnt[1] > blue) && (cnt[0] > blue || cnt[1] > red) {
                return (i - 1) as i32;
            }
            i += 1;
        }
    }
}
