pub struct Solution;

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
         let n = a.len();
        let mut ans = vec![0; n];
        let mut seen = vec![0; n + 1];
        let mut common = 0;

        for i in 0..n {
            let x = a[i] as usize;
            seen[x]+=1;
            if seen[x]==2 {
                common+=1;
            }

            let y = b[i] as usize;
            seen[y]+=1;
            if seen[y]==2 {
                common+=1;
            }

            ans[i]=common;
        }

        ans
    }
}