package leetcode

var sumPow10 = [100_001]int{1}

func initSumAndMultiplyII() {
	for i := 1; i < 100_001; i++ {
		sumPow10[i] = sumPow10[i-1] * 10 % mod
	}
}

func sumAndMultiplyII(s string, queries [][]int) []int {
	n := len(s)
	sumD := make([]int, n+1)
	preNum := make([]int, n+1)
	sumNonZero := make([]int, n+1)

	for i, ch := range s {
		d := int(ch - '0')
		sumD[i+1] = sumD[i] + d
		preNum[i+1] = preNum[i]
		sumNonZero[i+1] = sumNonZero[i]
		if d > 0 {
			preNum[i+1] = (preNum[i]*10 + d) % mod
			sumNonZero[i+1]++
		}
	}

	ans := make([]int, len(queries))

	for i, q := range queries {
		l, r := q[0], q[1]+1
		length := sumNonZero[r] - sumNonZero[l]
		x := preNum[r] - preNum[l]*sumPow10[length]%mod + mod
		ans[i] = x * (sumD[r] - sumD[l]) % mod
	}

	return ans
}
