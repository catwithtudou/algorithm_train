package leetcode

func maxCollectedFruits(fruits [][]int) (ans int) {
	n := len(fruits)
	memo := make([][]int, n)
	for i := range memo {
		memo[i] = make([]int, n)
		for j := range memo[i] {
			memo[i][j] = -1
		}
	}

	var dfs func(int, int) int
	dfs = func(i, j int) int {
		if j >= n || j < n-i-1 {
			return 0
		}

		if i == 0 {
			return fruits[i][j]
		}

		p := &memo[i][j]
		if *p < 0 {
			*p = max(max(dfs(i-1, j), dfs(i-1, j-1)), dfs(i-1, j+1)) + fruits[i][j]
		}
		return *p
	}

	for i, row := range fruits {
		ans += row[i]
	}

	ans += dfs(n-2, n-1)

	for i := range fruits {
		for j := range i {
			fruits[j][i] = fruits[i][j]
		}
	}

	for i := range memo {
		for j := range memo[i] {
			memo[i][j] = -1
		}
	}

	return ans + dfs(n-2, n-1)
}
