package leetcode

func hasAllCodes(s string, k int) bool {
	setMap := make(map[string]bool, 0)
	for i := k; i <= len(s); i++ {
		setMap[s[i-k:i]] = true
	}
	return len(setMap) == 1<<k
}
