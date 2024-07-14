package leetcode

func maxIncreaseKeepingSkyline(grid [][]int) int {
	n := len(grid)
	rowMax := make([]int, n)
	colMax := make([]int, n)
	for i, row := range grid {
		for j, h := range row {
			rowMax[i] = max(rowMax[i], h)
			colMax[j] = max(colMax[j], h)
		}
	}

	ans := 0
	for i, row := range grid {
		for j, h := range row {
			ans += min(rowMax[i], colMax[j]) - h
		}
	}

	return ans
}
