pub struct Solution;

impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        let corridor = corridor.as_bytes();
        let mut cnt_s = 0;
        let mut last_s = 0;
        let mut ans = 1;

        for (i, &v) in corridor.iter().enumerate() {
            if v != b'S' {
                continue;
            }
            cnt_s += 1;
            if cnt_s > 2 && cnt_s % 2 > 0 {
                ans = (ans * (i - last_s) as i64) % 1_000_000_007;
            }
            last_s = i;
        }
        if cnt_s == 0 || cnt_s % 2 > 0 {
            return 0;
        }
        ans as i32
    }
}
