package leetcode

func countNegatives(grid [][]int) (ans int) {

	m, n := len(grid), len(grid[0])

	i, j := 0, n-1

	for i < m && j >= 0 {
		if grid[i][j] < 0 {
			ans += m - i
			j--
		} else {
			i++
		}
	}

	return
}
