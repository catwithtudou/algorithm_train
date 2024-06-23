package leetcode

import (
	"fmt"
	"unicode"
)

func detectCapitalUse(word string) bool {
	up := false
	if unicode.IsUpper(rune(word[0])) {
		up = true
	}
	reUp, low := false, false
	if !up {
		for i := 1; i < len(word); i++ {
			if unicode.IsUpper(rune(word[i])) {
				return false
			}
		}
		return true
	}

	for i := 1; i < len(word); i++ {
		if unicode.IsUpper(rune(word[i])) {
			reUp = true
			fmt.Println(low)
			if low {
				return false
			}
			continue
		}
		low = true
		if reUp {
			return false
		}
	}

	return true
}
