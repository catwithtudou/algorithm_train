package leetcode

func isAcronym(words []string, s string) bool {
	n := len(s)
	if n != len(words) {
		return false
	}
	for k, word := range words {
		if k >= n || word[0] != s[k] {
			return false
		}
	}
	return true
}
