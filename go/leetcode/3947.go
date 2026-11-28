package leetcode

import (
	"math"
	"slices"
)

func maximumSaleItems3947(items [][]int, budget int) (ans int) {
	n := len(items)
	cntFactor := make([]int, n+1)
	minPrice := math.MaxInt
	for _, p := range items {
		cntFactor[p[0]]++
		minPrice = min(minPrice, p[1])
	}
	cntMulti := make([]int, n+1)
	type pair struct{ price, cnt int }
	a := []pair{}

	for _, p := range items {
		factor, price := p[0], p[1]
		if price >= minPrice*2 {
			continue
		}

		if cntMulti[factor] == 0 { // 之前没有计算过
			for j := factor; j <= n; j += factor {
				cntMulti[factor] += cntFactor[j]
			}
		}

		cnt := cntMulti[factor] - 1 // factor 的倍数不包括该物品
		if cnt > 0 {
			a = append(a, pair{price, cnt})
		}
	}

	slices.SortFunc(a, func(a, b pair) int { return a.price - b.price })

	for _, p := range a {
		if budget < p.price { // 没钱了
			break
		}
		c := min(p.cnt, budget/p.price) // 该物品最多买 c 个
		budget -= p.price * c
		ans += c * 2
	}

	// 剩余的钱买最便宜的物品
	return ans + budget/minPrice
}
