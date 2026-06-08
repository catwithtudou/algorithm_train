pub struct Solution;

impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut a = Vec::new();
        let mut b = Vec::new();
        let mut c = Vec::new();
        for v in nums {
            if v < pivot {
                a.push(v);
            } else if v == pivot {
                b.push(v);
            } else {
                c.push(v);
            }
        }
        a.append(&mut b);
        a.append(&mut c);
        a
    }
}
