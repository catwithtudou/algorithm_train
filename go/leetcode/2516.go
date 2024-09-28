package leetcode

func takeCharacters(s string, k int) int {
	cnt := [3]int{}
	for _, c := range s {
		cnt[c-'a']++
	}

	if cnt[0] < k || cnt[1] < k || cnt[2] < k {
		return -1
	}

	left, mx := 0, 0

	for right, c := range s {
		cnt[c-'a']--
		for cnt[c-'a'] < k {
			cnt[s[left]-'a']++
			left++
		}
		mx = max(mx, right-left+1)
	}

	return len(s) - mx
}
