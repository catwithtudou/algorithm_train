package leetcode

func numberOfWays2147(corridor string) int {
	const mod = 1_000_000_007
	cntS, lastS, ans := 0, 0, 1

	for i, v := range corridor {
		if v != 'S' {
			continue
		}
		cntS++
		if cntS > 2 && cntS%2 > 0 {
			ans = ans * (i - lastS) % mod
		}
		lastS = i
	}

	if cntS == 0 || cntS%2 > 0 {
		return 0
	}

	return ans
}
