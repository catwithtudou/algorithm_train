package leetcode

func minimumWhiteTiles(floor string, numCarpets int, carpetLen int) int {
	n := len(floor)
	memo := make([][]int, numCarpets+1)
	for i := range memo {
		memo[i] = make([]int, n)
		for j := range memo[i] {
			memo[i][j] = -1
		}
	}

	var dfs func(int, int) int
	dfs = func(i, j int) (res int) {
		if j < i*carpetLen {
			return
		}

		p := &memo[i][j]
		if *p != -1 {
			return *p
		}
		defer func() { *p = res }()

		res = dfs(i, j-1) + int(floor[j]-'0')
		if i > 0 {
			res = min(res, dfs(i-1, j-carpetLen))
		}
		return
	}

	return dfs(numCarpets, n-1)
}
