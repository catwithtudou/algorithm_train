package leetcode

func countOperations(num1 int, num2 int) (ans int) {
	if num1 == 0 || num2 == 0 {
		return 0
	}

	for num1 != num2 {
		if num1 > num2 {
			num1 -= num2
		} else {
			num2 -= num1
		}
		ans++
	}

	return ans + 1
}
