﻿package leetcode

import (
	"strings"
	"unicode"
)

func isValid3136(word string) bool {
	if len(word) < 3 {
		return false
	}

	var f0, f1 bool
	for _, c := range word {
		if unicode.IsLetter(c) {
			if strings.ContainsRune("aeiou", unicode.ToLower(c)) {
				f1 = true
			} else {
				f0 = true
			}
		} else if !unicode.IsDigit(c) {
			return false
		}
	}
	return f0 && f1
}
