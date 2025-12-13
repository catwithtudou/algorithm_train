package leetcode

import (
	"unicode"

	"golang.org/x/exp/slices"
)

var businessLineToCategory = map[string]int{
	"electronics": 0,
	"grocery":     1,
	"pharmacy":    2,
	"restaurant":  3,
}

func validateCoupons(code []string, businessLine []string, isActive []bool) (ans []string) {
	groups := [4][]string{}
	for i, s := range code {
		category, ok := businessLineToCategory[businessLine[i]]
		if ok && isActive[i] && isValidCode(s) {
			groups[category] = append(groups[category], s)
		}
	}

	for _, g := range groups {
		slices.Sort(g)
		ans = append(ans, g...)
	}

	return
}

func isValidCode(s string) bool {
	for _, c := range s {
		if c != '_' && !unicode.IsLetter(c) && !unicode.IsDigit(c) {
			return false
		}
	}
	return s != ""
}
