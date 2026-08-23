pub struct Solution;

impl Solution {
    pub fn sum_game(num: String) -> bool {
        fn calc(part: &[u8]) -> (i32, i32) {
            let mut q = 0;
            let mut sum = 0;

            for &ch in part {
                if ch == b'?' {
                    q += 1;
                } else {
                    sum += (ch - b'0') as i32;
                }
            }

            (q, sum)
        }

        let bytes = num.as_bytes();
        let n = bytes.len();

        let (ql, sum_l) = calc(&bytes[..n / 2]);
        let (qr, sum_r) = calc(&bytes[n / 2..]);

        (ql + qr) % 2 == 1
            || (ql - qr) / 2 * 9 != sum_r - sum_l
    }
}