package leetcode

func countServers1267(grid [][]int) (ans int) {
	rowCnt := make([]int, len(grid))
	colCnt := make([]int, len(grid[0]))
	for i, row := range grid {
		for j, x := range row {
			if x > 0 {
				rowCnt[i]++
				colCnt[j]++
			}
		}
	}

	for i, row := range grid {
		for j, x := range row {
			if x > 0 && (rowCnt[i] > 1 || colCnt[j] > 1) {
				ans++
			}
		}
	}
	return
}
