package leetcode

import (
	"strconv"
	"strings"
)

func maximum69Number(num int) int {
	str := strconv.Itoa(num)
	str = strings.Replace(str, "6", "9", 1)
	res, _ := strconv.Atoi(str)
	return res
}
