package leetcode

import "math"

func maxActiveSectionsAfterTrade(s string) (ans int) {

	mx := 0

	preCnt0 := math.MinInt

	cnt := 0

	for i := range len(s) {
		cnt++
		if i == len(s)-1 || s[i] != s[i+1] {

			if s[i] == '1' {
				ans += cnt
			} else {
				mx = max(mx, preCnt0+cnt)
				preCnt0 = cnt
			}
			cnt = 0
		}
	}

	return ans + mx

}
