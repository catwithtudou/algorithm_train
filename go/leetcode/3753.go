package leetcode

import (
	"cmp"
	"strconv"
)

func totalWaviness3753(num1 int64, num2 int64) int64 {
	lowS := strconv.FormatInt(num1, 10)
	highS := strconv.FormatInt(num2, 10)
	n := len(highS)
	diffLH := n - len(lowS)
	memo := make([][][3][10]int, n)
	for i := range memo {
		memo[i] = make([][3][10]int, n-1)
	}

	var dfs func(int, int, int, int, bool, bool) int

	dfs = func(i, waveNum, lastCmp, lastDig int, isLow, isHigh bool) (res int) {
		if i == n {
			return waveNum
		}

		if !isLow && !isHigh {
			p := &memo[i][waveNum][lastCmp+1][lastDig]
			if *p > 0 {
				return *p - 1
			}
			defer func() { *p = res + 1 }()
		}

		lo := 0
		if isLow && i >= diffLH {
			lo = int(lowS[i-diffLH] - '0')
		}

		hi := 9
		if isHigh {
			hi = int(highS[i] - '0')
		}

		isNum := !isLow || i > diffLH

		for d := lo; d <= hi; d++ {
			c := 0
			if isNum {
				c = cmp.Compare(d, lastDig)
			}
			w := waveNum
			if c*lastCmp < 0 {
				w++
			}
			res += dfs(i+1, w, c, d, isLow && d == lo, isHigh && d == hi)
		}

		return
	}

	return int64(dfs(0, 0, 0, 0, true, true))
}
