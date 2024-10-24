pub struct Solution;

impl Solution {
    pub fn find_winning_player(skills: Vec<i32>, k: i32) -> i32 {
        let (mut win,mut max_i) = (0,0);
        for i in 1..skills.len() {
            if skills[i] > skills[max_i] {
                max_i = i;
                win=0;
            }
            win+=1;
            if win==k {
                return max_i as _;
            }
        }
        return max_i as _;
    }
}