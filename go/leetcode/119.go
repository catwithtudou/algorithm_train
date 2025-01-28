package leetcode

func getRow(rowIndex int) []int {
	var c [34][]int
	for i := range c {
		c[i] = make([]int, i+1)
		c[i][0], c[i][i] = 1, 1
		for j := 1; j < i; j++ {
			c[i][j] = c[i-1][j-1] + c[i-1][j]
		}
	}
	return c[rowIndex]
}
