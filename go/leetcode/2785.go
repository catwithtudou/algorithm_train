package leetcode

import (
	"slices"
	"strings"
	"unicode"
)

func sortVowels(s string) string {
	vowels := []byte{}
	for _, ch := range s {
		c := unicode.ToLower(ch)
		if strings.ContainsRune("aeiou", c) {
			vowels = append(vowels, byte(ch))
		}
	}

	slices.Sort(vowels)

	t := []byte(s)
	j := 0
	for i, ch := range t {
		c := unicode.ToLower(rune(ch))
		if strings.ContainsRune("aeiou", c) {
			t[i] = vowels[j]
			j++
		}
	}
	return string(t)
}
