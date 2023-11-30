package leetcode

import "golang.org/x/exp/slices"

func closeStrings(word1 string, word2 string) bool {
	if len(word1) != len(word2) {
		return false
	}

	var iMark, jMark int
	var iList, jList [26]int
	for _, c := range word1 {
		iMark |= 1 << (c - 'a')
		iList[c-'a']++
	}

	for _, c := range word2 {
		jMark |= 1 << (c - 'a')
		jList[c-'a']++
	}

	slices.Sort(iList[:])
	slices.Sort(jList[:])
	return iMark == jMark && slices.Equal(iList[:], jList[:])
}
