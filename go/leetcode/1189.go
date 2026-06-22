package leetcode

func maxNumberOfBalloons(text string) int {
	cnt := ['z' + 1]int{}

	for _, c := range text {
		cnt[c]++
	}

	return min(min(cnt['a'], cnt['b']), min(min(cnt['l']/2, cnt['o']/2), cnt['n']))
}
