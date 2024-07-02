package leetcode

func maximumPrimeDifference(nums []int) int {
	i, j := 0, len(nums)-1
	for !isPrime(nums[i]) {
		i++
	}
	for !isPrime(nums[j]) {
		j--
	}
	return j - i
}

func isPrime(n int) bool {
	for i := 2; i*i <= n; i++ {
		if n%i == 0 {
			return false
		}
	}
	return n >= 2
}
