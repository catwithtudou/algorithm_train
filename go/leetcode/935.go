package leetcode

func knightDialer(n int) int {
	if n == 1 {
		return 10
	}

	const mod = 1_000_000_007
	next := [10][]int{{4, 6}, {6, 8}, {7, 9}, {4, 8}, {0, 3, 9}, {}, {0, 1, 7}, {2, 6}, {1, 3}, {2, 4}}
	memo := [5000][10]int{}

	var dfs func(int, int) int
	dfs = func(i, j int) int {
		if i == 0 {
			return 1
		}

		p := &memo[i][j]
		if *p != 0 {
			return *p
		}
		ans := 0
		for _, v := range next[j] {
			ans += dfs(i-1, v)
		}
		*p = ans % mod
		return *p
	}

	ans := 0
	for i := range 10 {
		ans += dfs(n-1, i)
	}
	return ans % mod
}
