package leetcode

func mapWordWeights(words []string, weights []int) string {
	ans := make([]byte, len(words))
	for i, w := range words {
		sum := 0
		for _, c := range w {
			sum += weights[c-'a']
		}
		ans[i] = 'z' - byte(sum%26)
	}
	return string(ans)
}
