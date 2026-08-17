package leetcode

func stoneGameV(stoneValue []int) int {
	n := len(stoneValue)
	sum := make([]int, n+1)
	for i, v := range stoneValue {
		sum[i+1] = sum[i] + v
	}

	memo := make([][]int, n)
	for i := range memo {
		memo[i] = make([]int, n+1)
	}

	var dfs func(int, int) int
	dfs = func(i, j int) (res int) {
		if j-i == 1 {
			return 0
		}

		p := &memo[i][j]
		if *p > 0 {
			return *p
		}

		for k := i + 1; k < j; k++ {
			sumL, sumR := sum[k]-sum[i], sum[j]-sum[k]
			score := 0
			if sumL < sumR {
				score = dfs(i, k) + sumL
			} else if sumL > sumR {
				score = dfs(k, j) + sumR
			} else {
				score = max(dfs(i, k), dfs(k, j)) + sumL
			}
			res = max(res, score)
		}

		*p = res
		return
	}

	return dfs(0, n)
}
