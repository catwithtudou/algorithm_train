pub struct Solution;

impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let (mut s,mut t) = (start as usize,destination as usize);
        if s > t {
            std::mem::swap(&mut s, &mut t)
        }
        let d1 = distance[s..t].iter().sum::<i32>();
        let d2 = distance[..s].iter().sum::<i32>() + distance[t..].iter().sum::<i32>();
        d1.min(d2)
    }
}