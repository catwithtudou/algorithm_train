pub struct Solution;


impl Solution {
    pub fn next_beautiful_number(n: i32) -> i32 {
        let mut x = n + 1;
        loop {
            let mut cnt = [0; 10];
            let mut y = x;
            while y > 0 {
                cnt[(y % 10) as usize] += 1;
                y /= 10;
            }
            let mut ok = true;
            y = x;
            while y > 0 {
                if y % 10 != cnt[(y % 10) as usize] {
                    ok = false;
                    break;
                }
                y /= 10;
            }
            if ok {
                return x;
            }
            x += 1;
        }
    }
}