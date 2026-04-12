pub struct Solution;

impl Solution {
    pub fn minimum_distance(word: String) -> i32 {
        let bytes = word.as_bytes();
        let n = bytes.len();
        if n <= 1 {
            return 0;
        }

        let mut dp = vec![[0i32; 26]; n];

        for i in 0..(n - 1) {
            let x = (bytes[i] - b'A') as usize;
            let y = (bytes[i + 1] - b'A') as usize;

            for another_finger in 0..26 {
                dp[i + 1][another_finger] = (dp[i][another_finger] + Self::distance(y, x))
                    .min(dp[i][y] + Self::distance(another_finger, x));
            }
        }

        *dp[n - 1].iter().min().unwrap()
    }

    fn distance(a: usize, b: usize) -> i32 {
        const COLUMN: usize = 6;
        let ax = a / COLUMN;
        let ay = a % COLUMN;
        let bx = b / COLUMN;
        let by = b % COLUMN;

        (ax.abs_diff(bx) + ay.abs_diff(by)) as i32
    }
}