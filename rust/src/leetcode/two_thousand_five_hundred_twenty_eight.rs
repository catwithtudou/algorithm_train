pub struct Solution;

impl Solution {
    pub fn max_power(stations: Vec<i32>, r: i32, k: i32) -> i64 {
        let n = stations.len();
        let r = r as usize;
        let k = k as i64;

        let mut pref = vec![0i64; n + 1];
        for (i, &x) in stations.iter().enumerate() {
            pref[i + 1] = pref[i] + x as i64;
        }

        let mut power = vec![0i64; n];
        let mut mn = i64::MAX;
        for i in 0..n {
            let l = i.saturating_sub(r);
            let rr = (i + r + 1).min(n);
            let p = pref[rr] - pref[l];
            power[i] = p;
            if p < mn {
                mn = p;
            }
        }

        let left = mn + (k / n as i64);
        let right = mn + k;

        let feasible = |target: i64| -> bool {
            // 差分数组技巧：对区间 [i, i+2r] 加同一增量 m
            let mut diff = vec![0i64; n + 1];
            let mut sum_d = 0i64;
            let mut built = 0i64;

            for i in 0..n {
                sum_d += diff[i];
                let cur = power[i] + sum_d;
                if cur >= target {
                    continue;
                }

                let m = target - cur;
                built += m;
                if built > k {
                    return false;
                }

                sum_d += m;
                let stop = (i + 2 * r + 1).min(n);
                diff[stop] -= m;
            }
            true
        };

        let mut lo = left;
        let mut hi = right;
        let mut ans = left;
        while lo <= hi {
            let mid = lo + ((hi - lo) >> 1);
            if feasible(mid) {
                ans = mid;
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        ans
    }
}
