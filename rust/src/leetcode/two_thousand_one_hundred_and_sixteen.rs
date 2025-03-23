pub struct Solution;

impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        if s.len() % 2 != 0 {
            return false;
        }

        let (mut mn, mut mx) = (0, 0);

        for (b, lock) in s.bytes().zip(locked.bytes()) {
            if lock == b'1' {
                let d = if b == b'(' { 1 } else { -1 };
                mx += d;
                if mx < 0 {
                    return false;
                }
                mn += d;
            } else {
                mx += 1;
                mn -= 1;
            }
            if mn < 0 {
                mn = 1;
            }
        }

        mn == 0
    }
}
