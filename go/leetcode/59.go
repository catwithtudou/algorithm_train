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

func generateMatrixII(n int) [][]int {
	ans := make([][]int, n)
	for i := range ans {
		ans[i] = make([]int, n)
	}
	l, r, b, t, val := 0, n-1, n-1, 0, 1
	for val <= n*n {
		for i := l; i <= r; i++ {
			ans[t][i] = val
			val++
		}
		t++
		for i := t; i <= b; i++ {
			ans[i][r] = val
			val++
		}
		r--
		for i := r; i >= l; i-- {
			ans[b][i] = val
			val++
		}
		b--
		for i := b; i >= t; i-- {
			ans[i][l] = val
			val++
		}
		l++
	}
	return ans
}
