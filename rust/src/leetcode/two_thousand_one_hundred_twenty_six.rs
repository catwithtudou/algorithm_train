pub struct Solution;

impl Solution {
    pub fn asteroids_destroyed(mass: i32, mut asteroids: Vec<i32>) -> bool {
        asteroids.sort_unstable();
        let mut mass = mass as i64;
        for a in asteroids {
            if mass < a as i64{
                return false;
            }
            mass += a as i64;
        }
        true
    }
}
