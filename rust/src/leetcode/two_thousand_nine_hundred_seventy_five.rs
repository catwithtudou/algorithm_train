use std::collections::HashMap;

pub struct Solution;

impl Solution {
    fn area_f(a: &mut Vec<i32>, mx: i32) -> HashMap<i32, bool> {
        a.push(1);
        a.push(mx);
        a.sort();
        let mut area = HashMap::new();

        for i in 0..a.len() {
            for j in i + 1..a.len() {
                area.insert(a[j] - a[i], true);
            }
        }

        area
    }

    pub fn maximize_square_area(
        m: i32,
        n: i32,
        mut h_fences: Vec<i32>,
        mut v_fences: Vec<i32>,
    ) -> i32 {
        let ha = Self::area_f(&mut h_fences, m);
        let va = Self::area_f(&mut v_fences, n);

        let mut ans = 0;
        for x in ha.keys() {
            if va.contains_key(x) {
                ans = ans.max(*x as i64);
            }
        }

        if ans == 0 {
            -1
        } else {
            ((ans * ans) % 1000000007) as i32
        }
    }
}
