package leetcode

func smallestBeautifulString(S string, k int) string {
	limit := 'a' + byte(k)
	s := []byte(S)
	n := len(s)
	i := n - 1
	s[i]++
	for i < n {
		if s[i] == limit {
			if i == 0 {
				return ""
			}
			s[i] = 'a'
			i--
			s[i]++
		} else if (i > 0 && s[i] == s[i-1]) || (i > 1 && s[i] == s[i-2]) {
			s[i]++
		} else {
			i++
		}
	}

	return string(s)
}
