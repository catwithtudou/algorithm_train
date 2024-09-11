pub struct Solution;

impl Solution {
    pub fn maximize_win(prize_positions: Vec<i32>, k: i32) -> i32 {
        let n = prize_positions.len();
        if k * 2 + 1 >= prize_positions[n - 1] - prize_positions[0] {
            return n as _;
        }
        let (mut mx, mut right, mut left, mut ans) = (0, 0, 0, 0);
        for (mid, &p) in prize_positions.iter().enumerate() {
            while right < n && prize_positions[right] - p <= k {
                right += 1;
            }
            ans = ans.max(mx + right - mid);
            while p - prize_positions[left] > k {
                left += 1;
            }
            mx = mx.max(mid - left + 1);
        }
        ans as _
    }
}
