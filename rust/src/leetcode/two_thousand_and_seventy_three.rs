pub struct Solution;

impl Solution {
    pub fn time_required_to_buy(tickets: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let tk = tickets[k];
        tickets.iter().enumerate().map(|(i,&t)| t.min(if i<=k {tk}else{tk-1})).sum()
    }
}