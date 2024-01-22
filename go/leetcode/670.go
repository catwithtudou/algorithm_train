package leetcode

import "strconv"

func maximumSwap(num int) int {
	numStr := strconv.Itoa(num)
	maxIdx := len(numStr) - 1
	p, q := -1, 0
	for i := len(numStr) - 2; i >= 0; i-- {
		if numStr[i] < numStr[maxIdx] {
			p = i
			q = maxIdx
		} else if numStr[i] > numStr[maxIdx] {
			maxIdx = i
		}
	}

	if p == -1 {
		return num
	}

	t := []byte(numStr)
	t[p], t[q] = t[q], t[p]
	ans, _ := strconv.Atoi(string(t))

	return ans
}
