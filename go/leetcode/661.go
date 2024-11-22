package leetcode

func imageSmoother(img [][]int) [][]int {
	m, n := len(img), len(img[0])
	sum := make([][]int, m+1)
	for i := range sum {
		sum[i] = make([]int, n+1)
	}

	for i := 1; i <= m; i++ {
		for j := 1; j <= n; j++ {
			sum[i][j] = sum[i-1][j] + sum[i][j-1] - sum[i-1][j-1] + img[i-1][j-1]
		}
	}

	ans := make([][]int, m)
	for i := 0; i < m; i++ {
		ans[i] = make([]int, n)
		for j := 0; j < n; j++ {
			a, b, c, d := max(0, i-1), max(0, j-1), min(m-1, i+1), min(n-1, j+1)
			cnt := (c - a + 1) * (d - b + 1)
			total := sum[c+1][d+1] - sum[a][d+1] - sum[c+1][b] + sum[a][b]
			ans[i][j] = total / cnt
		}
	}

	return ans
}
