package leetcode

import (
	"strconv"
	"strings"
)

func getNoZeroIntegers(n int) []int {
	for a := 1; ; a++ {
		if !strings.ContainsRune(strconv.Itoa(a), '0') &&
			!strings.ContainsRune(strconv.Itoa(n-a), '0') {
			return []int{a, n - a}
		}
	}
}
