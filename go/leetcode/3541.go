package leetcode

import "strings"

func maxFreqSum(s string) int {
	cnt := [26]int{}
	maxVowelCnt, maxConsonantCnt := 0, 0
	for _, ch := range s {
		cnt[ch-'a']++
		if strings.ContainsRune("aeiou", ch) {
			maxVowelCnt = max(maxVowelCnt, cnt[ch-'a'])
		} else {
			maxConsonantCnt = max(maxConsonantCnt, cnt[ch-'a'])
		}
	}
	return maxConsonantCnt + maxVowelCnt
}
