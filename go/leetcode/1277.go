package leetcode

func countSquares(matrix [][]int) (ans int) {
	m, n := len(matrix), len(matrix[0])
	f := make([][]int, m+1)
	for i := range f {
		f[i] = make([]int, n+1)
	}

	for i, row := range matrix {
		for j, v := range row {
			if v == 1 {
				f[i+1][j+1] = min(f[i][j], min(f[i+1][j], f[i][j+1])) + 1
				ans += f[i+1][j+1]
			}
		}
	}

	return
}
