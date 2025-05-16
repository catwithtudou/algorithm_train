package leetcode

func getWordsInLongestSubsequence(words []string, groups []int) []string {
	n := len(words)
	f := make([]int, n)
	from := make([]int, n)
	maxI := n - 1
	for i := n - 1; i >= 0; i-- {
		for j := i + 1; j < n; j++ {
			if f[j] > f[i] && groups[i] != groups[j] && LongestSubsequenceWords(words[i], words[j]) {
				f[i] = f[j]
				from[i] = j
			}
		}
		f[i]++
		if f[i] > f[maxI] {
			maxI = i
		}
	}
	ans := make([]string, f[maxI])
	for k := range ans {
		ans[k] = words[maxI]
		maxI = from[maxI]
	}
	return ans
}

func LongestSubsequenceWords(a, b string) (diff bool) {
	if len(a) != len(b) {
		return false
	}

	for i := range a {
		if a[i] != b[i] {
			if diff {
				return false
			}
			diff = true
		}
	}
	return
}
