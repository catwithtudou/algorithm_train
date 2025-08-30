package leetcode

func isValidSudoku(board [][]byte) bool {
	rowHas := [9][9]bool{}
	colHas := [9][9]bool{}
	subBoxHas := [3][3][9]bool{}

	for i, row := range board {
		for j, b := range row {
			if b == '.' {
				continue
			}
			x := b - '1'
			if rowHas[i][x] || colHas[j][x] || subBoxHas[i/3][j/3][x] {
				return false
			}
			rowHas[i][x] = true
			colHas[j][x] = true
			subBoxHas[i/3][j/3][x] = true
		}
	}
	return true
}
