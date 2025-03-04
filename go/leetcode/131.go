package leetcode

func isPalindrome(s string, left, right int) bool {
	for left < right {
		if s[left] != s[right] {
			return false
		}
		left++
		right--
	}
	return true
}

func partition(s string) (ans [][]string) {
	n := len(s)
	path := []string{}

	var dfs func(int, int)
	dfs = func(i, start int) {
		if i == n {
			ans = append(ans, append([]string(nil), path...))
			return
		}
		if i < n-1 {
			dfs(i+1, start)
		}
		if isPalindrome(s, start, i) {
			path = append(path, s[start:i+1])
			dfs(i+1, i+1)
			path = path[:len(path)-1]
		}
		return

	}
	dfs(0, 0)
	return
}
