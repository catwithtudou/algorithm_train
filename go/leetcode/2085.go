package leetcode

func countWords(words1 []string, words2 []string) int {
	wordMap := make(map[string]int)
	for _, word := range words1 {
		wordMap[word]++
	}
	for _, word := range words2 {
		if num, ok := wordMap[word]; ok && num > 1 {
			continue
		}
		wordMap[word]--
	}
	ans := 0
	for _, num := range wordMap {
		if num == 0 {
			ans++
		}
	}
	return ans
}
