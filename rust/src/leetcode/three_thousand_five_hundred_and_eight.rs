use std::collections::{HashMap, HashSet, VecDeque};

pub struct Solution;

struct Router {
    memory_limit: usize,
    data_list: VecDeque<(i32, i32, i32)>,
    data_set: HashSet<(i32, i32, i32)>,
    dest_to_ts: HashMap<i32, (Vec<i32>, usize)>,
}

impl Router {
    fn new(memory_limit: i32) -> Self {
        Self {
            memory_limit: memory_limit as usize,
            data_list: VecDeque::new(),
            data_set: HashSet::new(),
            dest_to_ts: HashMap::new(),
        }
    }

    fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        let data = (source, destination, timestamp);
        if !self.data_set.insert(data) {
            return false;
        }

        if self.data_list.len() == self.memory_limit {
            self.forward_packet();
        }

        self.data_list.push_back(data);
        self.dest_to_ts
            .entry(destination)
            .or_insert_with(|| (vec![], 0))
            .0
            .push(timestamp);

        true
    }

    fn forward_packet(&mut self) -> Vec<i32> {
        if let Some(data) = self.data_list.pop_front() {
            self.data_set.remove(&data);
            let (src, dest, ts) = data;
            self.dest_to_ts.get_mut(&dest).unwrap().1 += 1;
            vec![src, dest, ts]
        } else {
            vec![]
        }
    }

    fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        if let Some((ts, head)) = self.dest_to_ts.get(&destination) {
            let left = ts[*head..].partition_point(|&x| x < start_time);
            let right = ts[*head..].partition_point(|&x| x <= end_time);
            (right - left) as _
        } else {
            0
        }
    }
}
