package leetcode

import "math"

func largestVariance(s string) (ans int) {
	for a := 'a'; a <= 'z'; a++ {
		for b := 'a'; b <= 'z'; b++ {
			if a == b {
				continue
			}

			f0, f1 := 0, math.MinInt

			for _, ch := range s {
				if ch == a {
					f0 = max(f0, 0) + 1
					f1++
				} else if ch == b {
					f1, f0 = max(f0, 0)-1, max(f0, 0)-1
				}
				ans = max(ans, f1)
			}
		}
	}

	return
}
