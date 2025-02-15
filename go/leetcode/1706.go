package leetcode

func findBall(grid [][]int) []int {
	n := len(grid[0])
	ans := make([]int, n)
	for j := range n {
		curCol := j
		for _, row := range grid {
			d := row[curCol]
			curCol += d
			if curCol < 0 || curCol == n || row[curCol] != d {
				curCol = -1
				break
			}
		}
		ans[j] = curCol
	}

	return ans
}
