pub struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let (mut ans, mut s, mut min_s) = (0, 0, 0);

        for (i, (g, c)) in gas.iter().zip(cost.iter()).enumerate() {
            s += g - c;
            if s < min_s {
                min_s = s;
                ans = i + 1;
            }
        }

        if s < 0 {
            -1
        } else {
            ans as _
        }
    }
}
