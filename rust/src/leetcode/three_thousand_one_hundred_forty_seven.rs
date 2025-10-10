pub struct Solution;

impl Solution {
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        let n = energy.len();
        let k = k as usize;
        let mut ans = std::i32::MIN;
        for i in (n - k)..n {
            let mut cur_sum = 0;
            for j in (0..=i).rev().step_by(k) {
                cur_sum += energy[j];
                ans = ans.max(cur_sum);
            }
        }
        ans
    }
}
