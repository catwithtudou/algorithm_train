package leetcode

func generateMatrix(n int) [][]int {
	result := make([][]int, n)
	for i, _ := range result {
		result[i] = make([]int, n)
	}
	startX, startY := 0, 0
	mid := n / 2
	count, offset := 1, 1
	for loop := n / 2; loop > 0; loop-- {
		i, j := startX, startY

		for j = startY; j < n-offset; j++ {
			result[startX][j] = count
			count++
		}
		for i = startX; i < n-offset; i++ {
			result[i][j] = count
			count++
		}
		for ; j > startY; j-- {
			result[i][j] = count
			count++
		}
		for ; i > startX; i-- {
			result[i][j] = count
			count++
		}

		startX++
		startY++
		offset++
	}

	if n%2 > 0 {
		result[mid][mid] = count
	}
	return result
}
