pub struct Solution;

impl Solution {
    pub fn winning_player_count(n: i32, pick: Vec<Vec<i32>>) -> i32 {
        let mut cnts = vec![vec![0; 11]; n as usize];
        for p in pick {
            cnts[p[0] as usize][p[1] as usize] += 1;
        }

        cnts.iter()
            .enumerate()
            .filter(|(i, cnt)| cnt.iter().any(|c| c > i))
            .count() as _
    }
}
