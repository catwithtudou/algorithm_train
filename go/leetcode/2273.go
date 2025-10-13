package leetcode

import (
	"bytes"

	"golang.org/x/exp/slices"
)

func removeAnagrams(words []string) []string {
	var base []byte
	k := 0
	for _, word := range words {
		s := []byte(word)
		slices.Sort(s)
		if !bytes.Equal(s, base) {
			base = s
			words[k] = word
			k++
		}
	}
	return words[:k]
}
