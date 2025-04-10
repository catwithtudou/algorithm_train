package leetcode

import "strconv"

func numberOfPowerfulInt(start int64, finish int64, limit int, s string) int64 {
	low := strconv.FormatInt(start, 10)
	high := strconv.FormatInt(finish, 10)
	n := len(high)
	diffH := n - len(low)
	diff := n - len(s)

	memo := make([]int64, n)
	for i := range memo {
		memo[i] = -1
	}

	var dfs func(int, bool, bool) int64
	dfs = func(i int, limitLow, limitHigh bool) (res int64) {
		if i == n {
			return 1
		}

		if !limitLow && !limitHigh {
			dv := &memo[i]
			if *dv != -1 {
				return *dv
			}
			defer func() {
				*dv = res
			}()
		}

		lo := 0
		if limitLow && i >= diffH {
			lo = int(low[i-diffH] - '0')
		}

		hi := 9
		if limitHigh {
			hi = int(high[i] - '0')
		}

		d := lo
		if limitLow && i < min(diffH, diff) {
			res = dfs(i+1, true, false)
			d = 1
		}

		if i < diff {
			for ; d <= min(hi, limit); d++ {
				res += dfs(i+1, limitLow && d == lo, limitHigh && d == hi)
			}
		} else {
			x := int(s[i-diff] - '0')
			if lo <= x && x <= min(hi, limit) {
				res += dfs(i+1, limitLow && x == lo, limitHigh && x == hi)
			}
		}

		return
	}

	return dfs(0, true, true)
}
