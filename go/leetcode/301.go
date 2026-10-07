package leetcode

func removeInvalidParentheses(s string) (ans []string) {
	left, right := 0, 0
	for i := range s {
		if s[i] == '(' {
			left++
		} else if s[i] == ')' {
			if left > 0 {
				left--
			} else {
				right++
			}
		}
	}
	seen := map[string]bool{}
	path := make([]byte, 0, len(s))
	var dfs func(int, int, int, int)
	dfs = func(i, balance, left, right int) {
		// 剩余字符必须足够完成删除，并闭合已保留的左括号。
		if len(s)-i < left+right+balance {
			return
		}
		if i == len(s) {
			result := string(path)
			if !seen[result] {
				seen[result] = true
				ans = append(ans, result)
			}
			return
		}
		ch := s[i]
		if ch == '(' && left > 0 {
			dfs(i+1, balance, left-1, right)
		}
		if ch == ')' && right > 0 {
			dfs(i+1, balance, left, right-1)
		}
		if ch == ')' && balance == 0 {
			return
		}
		if ch == '(' {
			balance++
		} else if ch == ')' {
			balance--
		}
		path = append(path, ch)
		dfs(i+1, balance, left, right)
		path = path[:len(path)-1]
	}
	dfs(0, 0, left, right)
	return
}
