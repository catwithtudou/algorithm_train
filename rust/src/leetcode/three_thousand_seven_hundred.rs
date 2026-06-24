pub struct Solution;

const MOD: i64 = 1_000_000_007;

impl Solution {
    pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
        fn new_matrix(n: usize, m: usize) -> Vec<Vec<i64>> {
            vec![vec![0; m]; n]
        }

        fn mul(a: &Vec<Vec<i64>>, b: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
            let n = a.len();
            let m = b[0].len();
            let mut c = new_matrix(n, m);

            for i in 0..n {
                for k in 0..a[i].len() {
                    let x = a[i][k];

                    if x == 0 {
                        continue;
                    }

                    for j in 0..m {
                        let y = b[k][j];

                        c[i][j] = (c[i][j] + x * y) % MOD;
                    }
                }
            }

            c
        }

        fn pow_mul(mut a: Vec<Vec<i64>>, mut n: i32, mut res: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
            while n > 0 {
                if n % 2 > 0 {
                    res = mul(&a, &res);
                }

                a = mul(&a, &a);
                n /= 2;
            }

            res
        }

        let k = (r - l + 1) as usize;

        let mut m = new_matrix(k * 2, k * 2);

        for i in 0..k {
            for j in 0..i {
                m[i][k + j] = 1;
            }

            for j in i + 1..k {
                m[k + i][j] = 1;
            }
        }

        let mut f1 = new_matrix(k * 2, 1);

        for i in 0..f1.len() {
            f1[i][0] = 1;
        }

        let fn_matrix = pow_mul(m, n - 1, f1);

        let mut ans = 0_i64;

        for row in fn_matrix {
            ans = (ans + row[0]) % MOD;
        }

        ans as i32
    }
}
