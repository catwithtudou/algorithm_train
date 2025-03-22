package leetcode

func rowAndMaximumOnes(mat [][]int) []int {
	rowIdx := 0
	rowCount := 0

	for i, row := range mat {
		count := 0
		for _, cell := range row {
			if cell == 1 {
				count++
			}
		}
		if count > rowCount {
			rowIdx = i
			rowCount = count
		}
	}

	return []int{rowIdx, rowCount}
}
