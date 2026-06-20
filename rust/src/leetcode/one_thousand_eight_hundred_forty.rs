pub struct Solution;

impl Solution {
    pub fn max_building(n: i32,mut restrictions: Vec<Vec<i32>>) -> i32 {
        let m = restrictions.len();

        if m == 0 {
            return n - 1;
        }

        restrictions.sort_unstable_by_key(|a| a[0]);

        let mut h = vec![0; m];

        h[0] = restrictions[0][1].min(restrictions[0][0] - 1);

        for i in 1..m {
            h[i] = restrictions[i][1].min(h[i - 1] + restrictions[i][0] - restrictions[i - 1][0]);
        }

        for i in (0..m-1).rev() {
            h[i] = h[i].min(h[i+1]+restrictions[i+1][0]-restrictions[i][0]);
        }

        let mut ans = ((restrictions[0][0]-1+h[0])/2).max(h[m-1]+n-restrictions[m-1][0]);

        for i in 0..m-1 {
            ans = ans.max((h[i]+h[i+1]-restrictions[i][0]+restrictions[i+1][0])/2);
        }

        ans
    }
}
