package leetcode

import (
	"math"
	"slices"
	"strconv"
)

func countGoodIntegers(n int, k int) (ans int64) {

	factorial := make([]int, n+1)
	factorial[0] = 1
	for i := 1; i <= n; i++ {
		factorial[i] = factorial[i-1] * i
	}

	vis := make(map[string]bool)
	base := int(math.Pow10((n - 1) / 2))
	for i := base; i <= base*10; i++ {
		x, t := i, i
		if n%2 > 0 {
			t /= 10
		}
		for ; t > 0; t /= 10 {
			x = x*10 + t%10
		}
		if x%k > 0 {
			continue
		}

		bs := []byte(strconv.Itoa(x))
		slices.Sort(bs)
		s := string(bs)
		if vis[s] {
			continue
		}
		vis[s] = true

		cnt := [10]int{}
		for _, c := range bs {
			cnt[c-'0']++
		}
		res := (n - cnt[0]) * factorial[n-1]
		for _, c := range cnt {
			res /= factorial[c]
		}
		ans += int64(res)
	}

	return
}
