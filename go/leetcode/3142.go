package leetcode

func satisfiesConditions(grid [][]int) bool {
	for i, row := range grid {
		for j, v := range row {
			if i > 0 && v != grid[i-1][j] {
				return false
			}
			if j > 0 && v == grid[i][j-1] {
				return false
			}
		}
	}

	return true

}
