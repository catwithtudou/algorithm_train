package leetcode

func numberOfPaths(grid [][]int, k int) int {
	const mod = 1_000_000_007

	m, n := len(grid), len(grid[0])

	memo := make([][][]int, m)

	for i := range memo {
		memo[i] = make([][]int, n)
		for j := range memo[i] {
			memo[i][j] = make([]int, k)
			for s := range memo[i][j] {
				memo[i][j][s] = -1
			}
		}
	}

	var dfs func(int, int, int) int

	dfs = func(i, j, s int) int {
		if i < 0 || j < 0 {
			return 0
		}

		preS := ((s-grid[i][j])%k + k) % k
		if i == 0 && j == 0 {
			if preS == 0 {
				return 1
			}
			return 0
		}

		p := &memo[i][j][s]
		if *p != -1 {
			return *p
		}
		*p = (dfs(i-1, j, preS) + dfs(i, j-1, preS)) % mod
		return *p
	}

	return dfs(m-1, n-1, 0)
}
