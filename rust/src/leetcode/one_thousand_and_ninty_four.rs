pub struct Solution;

impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut idx_max = 0;
        for trip in &trips {
            idx_max = idx_max.max(trip[2]);
        }

        let mut diff = vec![0; (idx_max + 1) as usize];
        for trip in &trips {
            diff[trip[1] as usize] += trip[0];
            diff[trip[2] as usize] -= trip[0];
        }

        let mut count = 0;
        for i in 0..idx_max {
            count += diff[i as usize];
            if count > capacity {
                return false;
            }
        }
        true
    }
}