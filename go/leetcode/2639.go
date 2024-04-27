package leetcode

func findColumnWidth(grid [][]int) []int {
	ans := make([]int, len(grid[0]))
	for j := range grid[0] {
		ma, mi := 0, 0
		for _, row := range grid {
			ma = max(ma, row[j])
			mi = min(mi, row[j])
		}

		xLen := 1
		for x := max(ma/10, -mi); x > 0; x /= 10 {
			xLen++
		}
		ans[j] = xLen
	}
	return ans
}
