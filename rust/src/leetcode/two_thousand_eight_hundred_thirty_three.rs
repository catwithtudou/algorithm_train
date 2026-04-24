pub struct Solution;

impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let l_cnt = moves.chars().filter(|&c| c == 'L').count() as i32;
        let r_cnt = moves.chars().filter(|&c| c == 'R').count() as i32;
        (l_cnt - r_cnt).abs() + moves.len() as i32 - l_cnt - r_cnt
    }
}