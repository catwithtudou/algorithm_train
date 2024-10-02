pub struct Solution;

impl Solution {
    pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
        let n = dist.len();
        let h100 = (hour * 100.0).round() as i64;
        let delta = h100 - (n as i64 - 1) * 100;
        if delta <= 0 {
            return -1;
        }

        let max_dist = *dist.iter().max().unwrap();
        if h100 <= n as i64 * 100 {
            return max_dist.max(((dist[n - 1] * 100 - 1) as i64 / delta) as i32 + 1);
        }

        let check = |v:i32|->bool{
            let mut t = 0i64;
            for &d in &dist[..n-1] {
                t+=((d-1)/v+1) as i64;
            }
            (t*v as i64 + dist[n-1] as i64) * 100 <=h100*v as i64
        };

        let mut left = 0;
        let h = (h100/(n*100)as i64) as i32;
        let mut right = (max_dist-1)/h+1;
        while left +1 < right {
            let mid = (left+right)/2;
            if check(mid){
                right=mid;
            }else{
                left =mid;
            }
        }
        right
    }
}
