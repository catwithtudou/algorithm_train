package leetcode

func maximumScore(grid [][]int) (ans int64) {
	n := len(grid)
	colSum := make([][]int64, n)
	for j := range colSum {
		colSum[j] = make([]int64, n+1)
		for i, row := range grid {
			colSum[j][i+1] = colSum[j][i] + int64(row[j])
		}
	}

	memo := make([][][2]int64, n-1)
	for i := range memo {
		memo[i] = make([][2]int64, n+1)
		for j := range memo[i] {
			memo[i][j] = [2]int64{-1, -1}
		}
	}

	var dfs func(int, int, int) int64

	dfs = func(j, pre, dec int) int64 {
		if j < 0 {
			return 0
		}

		p := &memo[j][pre][dec]
		if *p != -1 {
			return *p
		}

		res := int64(0)
		for cur := 0; cur <= n; cur++ {
			if cur == pre { // 情况一：相等
				res = max(res, dfs(j-1, cur, 0))
			} else if cur < pre { // 情况二：右边黑格多
				res = max(res, dfs(j-1, cur, 1)+colSum[j][pre]-colSum[j][cur])
			} else if dec == 0 { // 情况三：cur > pre >= 第 j+2 列的黑格个数
				res = max(res, dfs(j-1, cur, 0)+colSum[j+1][cur]-colSum[j+1][pre])
			} else if pre == 0 { // 情况四（凹形）：cur > pre < 第 j+2 列的黑格个数
				res = max(res, dfs(j-1, cur, 0))
			}
		}

		*p = res
		return res
	}

	for i := 0; i <= n; i++ {
		ans = max(ans, dfs(n-2, i, 0))
	}

	return
}
