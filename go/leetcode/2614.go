package leetcode

func diagonalPrime(nums [][]int) (ans int) {
	n := len(nums)
	for i, row := range nums {
		if num := row[i]; num > ans && isPrime(num) {
			ans = num
		}
		if num := row[n-i-1]; num > ans && isPrime(num) {
			ans = num
		}
	}

	return
}
