package leetcode

func shiftGrid(grid [][]int, k int) [][]int {
	m, n := len(grid), len(grid[0])

	ans := make([][]int, m)
	for i := range ans {
		ans[i] = make([]int, n)
	}

	size := m * n
	for i, row := range grid {
		for j, x := range row {
			p := (i*n + j + k) % size
			ans[p/n][p%n] = x
		}
	}

	return ans
}
