package leetcode

func numberOfStableArrays3130(zero int, one int, limit int) int {
	const mod = 1_000_000_007
	memo := make([][][2]int, zero+1)
	for i := range memo {
		memo[i] = make([][2]int, one+1)
		for j := range memo[i] {
			memo[i][j] = [2]int{-1, -1}
		}
	}

	var dfs func(i, j, k int) int
	dfs = func(i, j, k int) (res int) {
		if i <= 0 {
			if k == 1 && j <= limit {
				return 1
			}
			return
		}

		if j <= 0 {
			if k == 0 && i <= limit {
				return 1
			}
			return
		}

		p := &memo[i][j][k]
		if *p != -1 {
			return *p
		}
		defer func() { *p = res }()

		if k == 0 {
			res = (dfs(i-1, j, 0) + dfs(i-1, j, 1)) % mod
			if i > limit {
				res = (res - dfs(i-limit-1, j, 1) + mod) % mod
			}
		} else {
			res = (dfs(i, j-1, 0) + dfs(i, j-1, 1)) % mod
			if j > limit {
				res = (res - dfs(i, j-limit-1, 0) + mod) % mod
			}
		}
		return
	}

	return (dfs(zero, one, 0) + dfs(zero, one, 1)) % mod
}
