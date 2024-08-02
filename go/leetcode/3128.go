package leetcode

func numberOfRightTriangles(grid [][]int) int64 {
	var ans int64
	colSum := make([]int, len(grid[0]))
	for _, row := range grid {
		for j, col := range row {
			colSum[j] += col
		}
	}

	for _, row := range grid {
		rowSum := -1
		for _, x := range row {
			rowSum += x
		}
		for j, col := range row {
			if col == 1 {
				ans += int64(rowSum * (colSum[j] - 1))
			}
		}
	}

	return ans
}
