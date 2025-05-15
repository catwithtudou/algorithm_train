package leetcode

func getLongestSubsequence(words []string, groups []int) (ans []string) {

	pre := groups[0]
	ans = append(ans, words[0])
	for i := 1; i < len(words); i++ {
		if groups[i] != pre {
			ans = append(ans, words[i])
			pre = groups[i]
		}
	}

	return
}
