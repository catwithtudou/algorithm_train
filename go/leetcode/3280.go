package leetcode

import (
	"strconv"
	"strings"
)

func convertDateToBinary(date string) string {
	a := strings.Split(date, "-")
	for i := range a {
		x, _ := strconv.Atoi(a[i])
		a[i] = strconv.FormatUint(uint64(x), 2)
	}
	return strings.Join(a, "-")
}
