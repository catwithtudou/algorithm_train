pub struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row_has = [[false; 9]; 9];
        let mut col_has = [[false; 9]; 9];
        let mut sub_box_has = [[[false; 9]; 3]; 3];

        for (i, row) in board.into_iter().enumerate() {
            for (j, b) in row.into_iter().enumerate() {
                if b == '.' {
                    continue;
                }
                let x = (b as u8 - b'1') as usize;
                if row_has[i][x] || col_has[j][x] || sub_box_has[i / 3][j / 3][x] {
                    return false;
                }
                row_has[i][x] = true;
                col_has[j][x] = true;
                sub_box_has[i / 3][j / 3][x] = true;
            }
        }

        true
    }
}
