package leetcode

func goodSubsetofBinaryMatrix(grid [][]int) []int {
	numToM := make(map[int]int)
	for i, row := range grid {
		mask := 0
		for j, col := range row {
			mask |= col << j
		}
		if mask == 0 {
			return []int{i}
		}
		numToM[mask] = i
	}

	for x, i := range numToM {
		for y, j := range numToM {
			if x&y == 0 {
				return []int{min(i, j), max(i, j)}
			}
		}
	}

	return []int{}
}
