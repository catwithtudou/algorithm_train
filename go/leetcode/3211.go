package leetcode

func validStrings(n int) []string {
	ans := make([]string, 0)
	path := make([]byte, n)
	var dfs func(i int)
	dfs = func(i int) {
		if i == n {
			ans = append(ans, string(path))
			return
		}
		path[i] = '1'
		dfs(i + 1)

		if i == 0 || path[i-1] == '1' {
			path[i] = '0'
			dfs(i + 1)
		}
	}
	dfs(0)
	return ans
}
