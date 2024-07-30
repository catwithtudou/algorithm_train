package leetcode

func getGoodIndices(variables [][]int, target int) []int {
	ans := []int{}
	for i, v := range variables {
		if quickPow(quickPow(v[0], v[1], 10), v[2], v[3]) == target {
			ans = append(ans, i)
		}
	}
	return ans
}

func quickPow(x, n, mod int) int {
	ans := 1
	for ; n > 0; n /= 2 {
		if n%2 > 0 {
			ans = ans * x % mod
		}
		x = x * x % mod
	}
	return ans
}
