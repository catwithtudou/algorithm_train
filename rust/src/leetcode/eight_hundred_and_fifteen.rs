pub struct Solution;

use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn num_buses_to_destination(mut routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        let mut bus_stop = HashMap::new();
        for (i, route) in routes.iter().enumerate() {
            for &x in route {
                bus_stop.entry(x).or_insert(vec![]).push(i);
            }
        }

        if !bus_stop.contains_key(&source) || !bus_stop.contains_key(&target) {
            return if source != target { -1 } else { 0 };
        }

        let mut dis = HashMap::new();
        dis.insert(source, 0);
        let mut q = VecDeque::new();
        q.push_back(source);
        while let Some(x) = q.pop_front() {
            let dis_x = dis[&x];
            for &i in &bus_stop[&x]{
                for &y in &routes[i]{
                    if !dis.contains_key(&y){
                        dis.insert(y, dis_x+1);
                        q.push_back(y);
                    }
                }
                routes[i].clear();
            }
        }




        dis.get(&target).copied().unwrap_or(-1)
    }
}
