package leetcode

import (
	"bytes"
	"strings"
)

func smallestNumberII(num string, t int64) string {

	tmp, cnt := int(t), 0

	for _, p := range []int{2, 3, 5, 7} {
		for tmp%p == 0 {
			tmp /= p
			cnt++
		}
	}

	if tmp > 1 {
		return "-1"
	}

	cnt = max(cnt-len(num)+1, 1)

	num = strings.Repeat("0", cnt) + num

	n := len(num)

	ans := bytes.Repeat([]byte{'0'}, n)

	type pair struct{ i, t int }
	vis := map[pair]bool{}

	var dfs func(int, int, bool) bool

	dfs = func(i, t int, isLimit bool) bool {
		if i == n {
			return t == 1
		}

		if !isLimit {
			p := pair{i, t}
			if vis[p] {
				return false
			}
			vis[p] = true
		}

		if isLimit && i < cnt && dfs(i+1, t, true) {
			return true
		}

		low := 0
		if isLimit {
			low = int(num[i] - '0')
		}
		for d := max(low, 1); d <= 9; d++ {
			if dfs(i+1, t/gcd(t, d), isLimit && d == low) {
				ans[i] = '0' + byte(d)
				return true
			}
		}
		return false
	}

	dfs(0, int(t), true)

	i := bytes.LastIndexByte(ans, '0')
	return string(ans[i+1:])
}
