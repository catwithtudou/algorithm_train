pub struct Solution;

impl Solution {
    pub fn stone_game_ix(stones: Vec<i32>) -> bool {
        fn check_stone(n: i32, mut cnt: [i32; 3]) -> bool {
            if cnt[1] == 0 {
                return false;
            }

            cnt[1] -= 1;

            let mut rounds = 1 + cnt[1].min(cnt[2]) * 2 + cnt[0];

            if cnt[1] > cnt[2] {
                rounds += 1;
            }

            rounds < n && rounds % 2 == 1
        }

        let mut cnt = [0_i32; 3];

        for x in stones {
            cnt[(x % 3) as usize] += 1;
        }

        let n = cnt.iter().sum::<i32>();

        check_stone(n, cnt)
            || check_stone(n, [cnt[0], cnt[2], cnt[1]])
    }
}