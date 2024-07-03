package leetcode

func sumOfTheDigitsOfHarshadNumber(x int) int {
	ans := x
	sum := 0
	for x > 0 {
		sum += x % 10
		x /= 10
	}
	if ans%sum == 0 {
		return sum
	}
	return -1
}
