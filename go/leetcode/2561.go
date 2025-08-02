package leetcode

import (
	"math"
	"slices"
)

func minCost2561(basket1, basket2 []int) (ans int64) {
	cnt := map[int]int{}
	for i, x := range basket1 {
		cnt[x]++
		cnt[basket2[i]]-- // 交集元素互相抵消
	}

	var a, b []int
	mn := math.MaxInt // 中介
	for x, c := range cnt {
		if c%2 != 0 { // 奇数无法均分
			return -1
		}
		mn = min(mn, x)
		// 剩余元素的一半放入 a 或者 b
		if c > 0 {
			for range c / 2 {
				a = append(a, x)
			}
		} else {
			for range -c / 2 {
				b = append(b, x)
			}
		}
	}

	slices.Sort(a)
	slices.SortFunc(b, func(a, b int) int { return b - a })

	for i, x := range a {
		ans += int64(min(min(x, b[i]), mn*2)) // 累加最小交换代价
	}
	return
}
