package leetcode

func splitWordsBySeparator(words []string, separator byte) []string {
	var res []string
	for _, word := range words {
		n := len(word)
		for i, start := 0, 0; i < n; i++ {
			if word[i] == separator {
				if i > start {
					res = append(res, word[start:i])
				}
				start = i + 1
			} else if i == n-1 {
				if i+1 > start {
					res = append(res, word[start:i+1])
				}
			}
		}
	}

	return res
}
