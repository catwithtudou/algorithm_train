package leetcode

func uniquePaths(grid [][]int) (ans int) {
	const mod = 1e9 + 7

	m, n := len(grid), len(grid[0])

	memo := make([][][2]int, m)
	for i := range memo {
		memo[i] = make([][2]int, n)
		for j := range memo[i] {
			memo[i][j] = [2]int{-1, -1}
		}
	}

	var dfs func(int, int, int) int

	dfs = func(i, j, k int) (res int) {
		if i < 0 || j < 0 {
			return 0
		}
		if i == 0 && j == 0 {
			return 1
		}
		p := &memo[i][j][k]
		if *p != -1 {
			return *p
		}
		defer func() { *p = res }()

		if grid[i][j] == 0 {
			return (dfs(i, j-1, 0) + dfs(i-1, j, 1)) % mod
		}
		if k == 0 {
			return dfs(i-1, j, 1)
		}
		return dfs(i, j-1, 0)
	}

	return dfs(m-1, n-1, 0)
}
