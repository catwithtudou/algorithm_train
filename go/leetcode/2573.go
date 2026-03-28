package leetcode

func findTheString(lcp [][]int) string {
	n := len(lcp)
	s := make([]byte, n)
	i := 0
	for c := byte('a'); c <= 'z'; c++ {
		for j := i; j < n; j++ {
			if lcp[i][j] > 0 {
				s[j] = c
			}
		}
		for i < n && s[i] > 0 {
			i++
		}
		if i == n {
			break
		}
	}

	if i < n {
		return ""
	}

	for i := n - 1; i >= 0; i-- {
		for j := n - 1; j >= 0; j-- {
			actLcp := 0
			if s[i] == s[j] {
				if i == n-1 || j == n-1 {
					actLcp = 1
				} else {
					actLcp = lcp[i+1][j+1] + 1
				}
			}
			if lcp[i][j] != actLcp {
				return ""
			}
		}
	}

	return string(s)
}
