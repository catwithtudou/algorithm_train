package leetcode

func differenceOfSums(n int, m int) int {
	sum, num1 := 0, 0
	for i := 1; i <= n; i++ {
		sum += i
		if i%m == 0 {
			num1 += i
		}
	}
	return sum - num1*2
}
