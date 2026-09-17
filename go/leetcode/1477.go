package leetcode

func minSumOfLengths(arr []int, target int) int {
	n := len(arr)
	sufMin := make([]int, n)
	minLen := n + 1
	sum, r := 0, n-1

	for l := n - 1; l > 0; l-- {
		sum += arr[l]
		for sum > target {
			sum -= arr[r]
			r--
		}
		if sum == target {
			minLen = min(minLen, r-l+1)
		}
		sufMin[l] = minLen
	}

	ans, sum, l := n+1, 0, 0
	for r, x := range arr[:n-1] {
		sum += x
		for sum > target {
			sum -= arr[l]
			l++
		}
		if sum == target {
			ans = min(ans, r-l+1+sufMin[r+1])
		}
	}

	if ans > n {
		return -1
	}

	return ans
}
