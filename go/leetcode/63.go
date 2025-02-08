package leetcode

func uniquePathsWithObstacles(obstacleGrid [][]int) int {
	m, n := len(obstacleGrid), len(obstacleGrid[0])
	memo := make([][]int, m)
	for i := range memo {
		memo[i] = make([]int, n)
		for j := range memo[i] {
			memo[i][j] = -1
		}
	}

	var dfs func(int, int) int
	dfs = func(i, j int) (ans int) {
		if i < 0 || j < 0 || obstacleGrid[i][j] == 1 {
			return 0
		}
		if i == 0 && j == 0 {
			return 1
		}

		p := &memo[i][j]
		if *p != -1 {
			return *p
		}
		defer func() { *p = ans }()

		return dfs(i-1, j) + dfs(i, j-1)
	}

	return dfs(m-1, n-1)
}
