package leetcode

func similarPairs(words []string) (ans int) {
	cnt := make(map[int]int)
	for _, word := range words {
		mask := 0
		for _, c := range word {
			mask |= 1 << (c - 'a')
		}
		ans += cnt[mask]
		cnt[mask]++
	}

	return ans
}
