package leetcode

import "math"

func maxScore3148(grid [][]int) int {
	m, n := len(grid), len(grid[0])
	f := make([][]int, m+1)
	f[0] = make([]int, n+1)
	for i := range f[0] {
		f[0][i] = math.MaxInt
	}
	ans := math.MinInt
	for i, row := range grid {
		f[i+1] = make([]int, n+1)
		f[i+1][0] = math.MaxInt
		for j, x := range row {
			mn := min(f[i+1][j], f[i][j+1])
			ans = max(ans, x-mn)
			f[i+1][j+1] = min(mn, x)
		}
	}

	return ans
}
