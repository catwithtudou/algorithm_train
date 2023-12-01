use std::collections::HashMap;

pub struct Solution;


impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut idx = HashMap::new();
        for i in 0..m {
            for j in 0..n {
                idx.insert(mat[i][j], [i, j]);
            }
        }

        let mut rowCnt = vec![0; m];
        let mut colCnt = vec![0; n];
        for k in 0..arr.len() {
            let x = idx.get(&arr[k]).unwrap();
            let i = x[0];
            let j = x[1];
            rowCnt[i] += 1;
            colCnt[j] += 1;
            if rowCnt[i] == n || colCnt[j] == m {
                return k as i32;
            }
        }

        -1
    }
}