package leetcode

import "math"

func minCut(s string) int {
	n := len(s)

	palMemo := make([][]int8, n)
	for i := range palMemo {
		palMemo[i] = make([]int8, n)
		for j := range palMemo[i] {
			palMemo[i][j] = -1
		}
	}

	var isPalindrome func(l, r int) bool
	isPalindrome = func(l, r int) bool {
		if l >= r {
			return true
		}

		p := &palMemo[l][r]
		if *p != -1 {
			return *p == 1
		}

		res := s[l] == s[r] && isPalindrome(l+1, r-1)
		if res {
			*p = 1
		} else {
			*p = 0
		}
		return res
	}

	cutMemo := make([]int, n)
	for i := range cutMemo {
		cutMemo[i] = -1
	}

	var dfs func(r int) int

	dfs = func(r int) (res int) {
		if isPalindrome(0, r) {
			return
		}

		p := &cutMemo[r]
		if *p != -1 {
			return *p
		}
		defer func() {
			*p = res
		}()

		res = math.MaxInt
		for l := 1; l <= r; l++ {
			if isPalindrome(l, r) {
				res = min(res, dfs(l-1)+1)
			}
		}
		return
	}

	return dfs(n - 1)
}
