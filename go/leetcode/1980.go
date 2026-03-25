package leetcode

import (
	"fmt"
	"strconv"
)

func findDifferentBinaryString(nums []string) string {
	n := len(nums)
	has := make(map[int]bool, n)
	for _, s := range nums {
		num, _ := strconv.ParseInt(s, 2, 64)
		has[int(num)] = true
	}

	ans := 0
	for has[ans] {
		ans++
	}

	return fmt.Sprintf("%0*b", n, ans)
}
