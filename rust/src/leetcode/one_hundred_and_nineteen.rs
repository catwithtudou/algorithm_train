pub struct Solution;

static mut INITIALIZED: bool = false;
static mut C: [[i32; 34]; 34] = [[0; 34]; 34];

unsafe fn init_once() {
    if INITIALIZED {
        return;
    }

    INITIALIZED = true;

    for i in 0..C.len() {
        C[i][0] = 1;
        C[i][i] = 1;
        for j in 1..i {
            C[i][j] = C[i - 1][j - 1] + C[i - 1][j];
        }
    }
}

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let row_index = row_index as usize;
        unsafe {
            init_once();
            C[row_index][..=row_index].to_vec()
        }
    }
}
