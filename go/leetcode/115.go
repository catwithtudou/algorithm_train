package leetcode

func numDistinct(s string, t string) int {
	n, m := len(s), len(t)
	memo := make([][]int, n)
	for i := range memo {
		memo[i] = make([]int, m)
		for j := range memo[i] {
			memo[i][j] = -1
		}
	}
	var dfs func(int, int) int
	dfs = func(i, j int) (res int) {
		if i < j {
			return
		}
		if j < 0 {
			return 1
		}
		p := &memo[i][j]
		if *p != -1 {
			return *p
		}
		defer func() { *p = res }()
		res = dfs(i-1, j)
		if s[i] == t[j] {
			res += dfs(i-1, j-1)
		}
		return
	}

	return dfs(n-1, m-1)
}
