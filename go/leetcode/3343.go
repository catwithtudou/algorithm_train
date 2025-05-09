﻿package leetcode

var fac, invF [mx + 1]int

func initCountBalancedPermutations() {
	fac[0] = 1
	for i := 1; i <= mx; i++ {
		fac[i] = fac[i-1] * i % mod
	}
	invF[mx] = quickPow(fac[mx], mod-2, mod)
	for i := mx; i > 0; i-- {
		invF[i-1] = invF[i] * i % mod
	}
}

func countBalancedPermutations(num string) int {
	cnt := [10]int{}
	total := 0
	for _, c := range num {
		cnt[c-'0']++
		total += int(c - '0')
	}

	if total%2 > 0 {
		return 0
	}

	for i := 1; i < 10; i++ {
		cnt[i] += cnt[i-1]
	}

	n := len(num)
	n1 := n / 2
	memo := [10][][]int{}

	for i := range memo {
		memo[i] = make([][]int, n1+1)
		for j := range memo[i] {
			memo[i][j] = make([]int, total/2+1)
			for k := range memo[i][j] {
				memo[i][j][k] = -1
			}
		}
	}

	var dfs func(int, int, int) int
	dfs = func(i, left1, leftS int) int {
		if i < 0 {
			if leftS > 0 {
				return 0
			}
			return 1
		}

		p := &memo[i][left1][leftS]
		if *p != -1 {
			return *p
		}

		res := 0
		c := cnt[i]
		if i > 0 {
			c -= cnt[i-1]
		}
		left2 := cnt[i] - left1
		for k := max(c-left2, 0); k <= min(c, left1) && k*i <= leftS; k++ {
			r := dfs(i-1, left1-k, leftS-k*i)
			res = (res + r*invF[k]%mod*invF[c-k]) % mod
		}
		*p = res
		return res
	}

	return fac[n1] * fac[n-n1] % mod * dfs(9, n1, total/2) % mod
}
