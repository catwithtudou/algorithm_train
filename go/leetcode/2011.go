package leetcode

func finalValueAfterOperations(operations []string) int {
	subNum, addNum := 0, 0

	for _, op := range operations {
		if op[1] == '-' {
			subNum++
		} else {
			addNum++
		}
	}

	return addNum - subNum
}
