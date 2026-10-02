package leetcode

func generateParenthesis(n int) (ans []string) {
	path := make([]byte, 0, 2*n)
	var dfs func(int, int)
	dfs = func(left, right int) {
		if right == n {
			ans = append(ans, string(path))
			return
		}
		if left < n {
			path = append(path, '(')
			dfs(left+1, right)
			path = path[:len(path)-1]
		}
		// 右括号只能匹配已经放入的左括号。
		if right < left {
			path = append(path, ')')
			dfs(left, right+1)
			path = path[:len(path)-1]
		}
	}
	dfs(0, 0)
	return
}
