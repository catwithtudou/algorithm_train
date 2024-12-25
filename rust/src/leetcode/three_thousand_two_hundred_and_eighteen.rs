pub struct Solution;

impl Solution {
    pub fn minimum_cost(
        m: i32,
        n: i32,
        mut horizontal_cut: Vec<i32>,
        mut vertical_cut: Vec<i32>,
    ) -> i32 {
        let (m, n) = (m as usize, n as usize);
        horizontal_cut.sort_unstable();
        vertical_cut.sort_unstable();
        let mut ans = 0;
        let (mut i, mut j) = (0, 0);
        for _ in 0..m + n - 2 {
            if j==n-1 || i<m-1 && horizontal_cut[i] < vertical_cut[j] {
                ans+= horizontal_cut[i]*(n-j) as i32;
                i+=1;
            }else{
                ans+= vertical_cut[j]*(m-i) as i32;
                j+=1;
            }
        }
        ans
    }
}
