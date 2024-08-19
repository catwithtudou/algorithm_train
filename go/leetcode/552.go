package leetcode

func checkRecordII(n int) int {
	const mod = 1_000_000_007
	const mx = 100_001
	var memo [mx][2][3]int
	var dfs func(int, int, int) int
	dfs = func(i, j, k int) int {
		if i == 0 {
			return 1
		}
		p := &memo[i][j][k]
		if *p > 0 {
			return *p
		}
		res := dfs(i-1, j, 0)
		if j == 0 {
			res += dfs(i-1, 1, 0)
		}
		if k < 2 {
			res += dfs(i-1, j, k+1)
		}
		*p = res % mod
		return *p
	}
	return dfs(n, 0, 0)
}
