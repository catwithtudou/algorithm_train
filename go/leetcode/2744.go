package leetcode

func maximumNumberOfStringPairs(words []string) (ans int) {
	wordIndex := [26][26]bool{}
	for _, word := range words {
		x, y := word[0]-'a', word[1]-'a'
		if wordIndex[y][x] {
			ans++
			continue
		}
		wordIndex[x][y] = true
	}

	return
}

func maximumNumberOfStringPairsOther(words []string) int {
	wordMap := make(map[string]bool, len(words))
	for _, word := range words {
		wordMap[word] = true
	}

	res := 0
	for _, word := range words {
		reS := reverString(word)
		if reS == word {
			continue
		}
		if wordMap[reS] {
			res++
			wordMap[word] = false
		}
	}

	return res
}

func reverString(s string) string {
	reS := make([]byte, len(s))
	n := len(s)
	for i := 0; i < n; i++ {
		reS[i] = s[n-i-1]
	}
	return string(reS)
}
