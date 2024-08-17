package leetcode

func minimumOperationsToMakeKPeriodic(word string, k int) int {
	n := len(word)
	cnt := make(map[string]int, 0)
	for i := k; i <= n; i += k {
		cnt[word[i-k:i]]++
	}

	ans := 0
	for _, v := range cnt {
		ans = max(ans, v)
	}

	return n/k - ans
}
