package leetcode

func closestTarget(words []string, target string, startIndex int) int {
	n := len(words)
	ans := n
	for i, x := range words {
		if x == target {
			idx := abs(i - startIndex)
			ans = min(ans, min(idx, n-idx))
		}
	}

	if ans == n {
		return -1
	}

	return ans
}
