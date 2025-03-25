package leetcode

func differenceOfDistinctValues(grid [][]int) [][]int {
	m, n := len(grid), len(grid[0])
	ans := make([][]int, m)
	set := make(map[int]struct{})

	for i := range m {
		ans[i] = make([]int, n)
		for j := range n {
			clear(set)

			for x, y := i-1, j-1; x >= 0 && y >= 0; x, y = x-1, y-1 {
				set[grid[x][y]] = struct{}{}
			}
			topLeft := len(set)

			clear(set)

			for x, y := i+1, j+1; x < m && y < n; x, y = x+1, y+1 {
				set[grid[x][y]] = struct{}{}
			}
			bottomRight := len(set)

			ans[i][j] = abs(topLeft - bottomRight)
		}
	}

	return ans
}
