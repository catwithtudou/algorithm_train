package leetcode

import (
	"slices"
	"strconv"
)

var powTwoSortedStrSet = map[string]bool{}

func init869() {
	const maxN = 1_000_000_000
	for i := 1; i <= maxN; i <<= 1 {
		s := intToSortdStr(i)
		powTwoSortedStrSet[s] = true
	}
}

func intToSortdStr(n int) string {
	s := []byte(strconv.Itoa(n))
	slices.Sort(s)
	return string(s)
}

func reorderedPowerOf2(n int) bool {
	s := intToSortdStr(n)
	return powTwoSortedStrSet[s]
}
