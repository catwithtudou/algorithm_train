package leetcode

func isBalanced(num string) bool {
	count1, count2 := 0, 0
	for i, v := range num {
		if i%2 == 0 {
			count1 += int(v - '0')
		} else {
			count2 += int(v - '0')
		}
	}
	if count1 == count2 {
		return true
	}
	return false
}
