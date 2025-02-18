use std::collections::HashMap;

pub struct Solution;

struct RangeFreqQuery {
    pos:HashMap<i32,Vec<usize>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RangeFreqQuery {

    fn new(arr: Vec<i32>) -> Self {
        let mut pos = HashMap::new();
        for (i,&x) in arr.iter().enumerate() {
            pos.entry(x).or_insert(vec![]).push(i);
        }
        Self{pos}
    }

    fn query(&self, left: i32, right: i32, value: i32) -> i32 {
        if let Some(a) = self.pos.get(&value) {
            let p = a.partition_point(|&i| i< left as usize);
            let q = a.partition_point(|&i| i<= right as usize);
            (q-p) as _
        }else{
            0
        }
    }
}