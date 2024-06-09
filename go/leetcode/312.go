package leetcode

func maxCoins(nums []int) int {
	n := len(nums)
	val := make([]int, n+2)
	for i := 1; i <= n; i++ {
		val[i] = nums[i-1]
	}
	val[0], val[n+1] = 1, 1

	rec := make([][]int, n+2)
	for i := 0; i < n+2; i++ {
		rec[i] = make([]int, n+2)
		for j := 0; j < n+2; j++ {
			rec[i][j] = -1
		}
	}

	var dfs func(int, int) int
	dfs = func(l, r int) int {
		if l >= r-1 {
			return 0
		}
		if rec[l][r] != -1 {
			return rec[l][r]
		}
		for i := l + 1; i < r; i++ {
			sum := val[l] * val[i] * val[r]
			sum += dfs(l, i) + dfs(i, r)
			rec[l][r] = max(rec[l][r], sum)
		}

		return rec[l][r]
	}

	return dfs(0, n+1)
}

func maxCoinsOther(nums []int) int {
	n := len(nums)
	val := make([]int, n+2)
	for i := 1; i <= n; i++ {
		val[i] = nums[i-1]
	}
	val[0], val[n+1] = 1, 1

	rec := make([][]int, n+2)
	for i := 0; i < n+2; i++ {
		rec[i] = make([]int, n+2)
	}
	for i := n - 1; i >= 0; i-- {
		for j := i + 2; j <= n+1; j++ {
			for k := i + 1; k < j; k++ {
				sum := val[i] * val[k] * val[j]
				sum += rec[i][k] + rec[k][j]
				rec[i][j] = max(rec[i][j], sum)
			}
		}
	}
	return rec[0][n+1]
}
