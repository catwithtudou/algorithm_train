package leetcode

func validSequence(word1 string, word2 string) []int {
	n, m := len(word1), len(word2)
	suf := make([]int, n+1)
	suf[n] = m
	for i, j := n-1, m-1; i >= 0; i-- {
		if j >= 0 && word1[i] == word2[j] {
			j--
		}
		suf[i] = j + 1
	}

	ans := make([]int, m)
	changed := false
	j := 0
	for i := range word1 {
		if word1[i] == word2[j] || !changed && suf[i+1] <= j+1 {
			if word1[i] != word2[j] {
				changed = true
			}
			ans[j] = i
			j++
			if j == m {
				return ans
			}
		}
	}

	return nil
}
