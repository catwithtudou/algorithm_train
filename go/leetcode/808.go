package leetcode

func soupServings(n int) float64 {
	if n >= 4451 {
		return 1
	}

	n = (n + 24) / 25

	memo := make([][]float64, n+1)
	for i := range memo {
		memo[i] = make([]float64, n+1)
	}

	var dfs func(int, int) float64

	dfs = func(a, b int) float64 {
		if a <= 0 && b <= 0 {
			return 0.5
		}
		if a <= 0 {
			return 1
		}
		if b <= 0 {
			return 0
		}

		p := &memo[a][b]
		if *p == 0 {
			*p = (dfs(a-4, b) + dfs(a-3, b-1) + dfs(a-2, b-2) + dfs(a-1, b-3)) / 4
		}

		return *p
	}

	return dfs(n, n)
}
