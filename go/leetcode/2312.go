package leetcode

func sellingWood(m int, n int, prices [][]int) int64 {
	f := make([][]int64, m+1)
	for i := range f {
		f[i] = make([]int64, n+1)
	}

	for _, price := range prices {
		f[price[0]][price[1]] = int64(price[2])
	}

	for i := 1; i <= m; i++ {
		for j := 1; j <= n; j++ {
			for k := 1; k <= j/2; k++ { // 垂直切割，高度不变
				f[i][j] = maxInt64(f[i][j], f[i][k]+f[i][j-k])
			}
			for k := 1; k <= i/2; k++ { // 水平切割，宽度不变
				f[i][j] = maxInt64(f[i][j], f[k][j]+f[i-k][j])
			}
		}

	}

	return f[m][n]
}
