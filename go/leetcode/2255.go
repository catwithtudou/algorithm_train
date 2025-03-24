package leetcode

func countPrefixes(words []string, s string) (ans int) {

	for _, word := range words {

		if len(s) >= len(word) && s[:len(word)] == word {
			ans++
		}

	}

	return
}
