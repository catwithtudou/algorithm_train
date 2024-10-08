pub struct Solution;

use std::collections::HashSet;


impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut set_a = HashSet::with_capacity(paths.len());
        let mut set_b = HashSet::new();
        for p in paths {
            let (a,b) = (&p[0],&p[1]);
            set_b.remove(a);
            if !set_a.contains(b) {
                set_b.insert(b.clone());
            }
            set_a.insert(a.clone());
        }
        set_b.into_iter().next().unwrap()
    }
}