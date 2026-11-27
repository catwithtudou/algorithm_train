package leetcode

import "math"

func maximumSaleItems(items [][]int, budget int) (ans int) {

	f := make([]int, budget+1)
	minP := math.MaxInt

	for _, p := range items {
		fac, pri := p[0], p[1]
		minP = min(minP, pri)

		cnt := 0
		for _, x := range items {
			if x[0]%fac == 0 {
				cnt++
			}
		}

		for j := budget; j >= pri; j-- {
			f[j] = max(f[j], f[j-pri]+cnt)
		}
	}

	for i, cnt := range f {
		ans = max(ans, cnt+(budget-i)/minP)
	}

	return
}
