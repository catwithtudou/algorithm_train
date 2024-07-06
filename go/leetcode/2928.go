package leetcode

func distributeCandies2928(n int, limit int) int {
	ans := 0
	for i := 0; i <= limit; i++ {
		for j := 0; j <= limit; j++ {
			if i+j > n {
				break
			}
			if n-i-j <= limit {
				ans++
			}
		}
	}
	return ans
}

func distributeCandies292801(n int, limit int) int {
	ans := 0
	for i := 0; i <= min(limit, n); i++ {
		if n-i > 2*limit {
			continue
		}
		ans += min(n-i, limit) - max(0, n-i-limit) + 1
	}
	return ans
}

func distributeCandies292802(n int, limit int) int {
	c := func(i int) int {
		if i < 2 {
			return 0
		}
		return i * (i - 1) / 2
	}

	return c(n+2) - 3*c(n-limit+1) + 3*c(n-2*limit) - c(n-3*limit-1)
}
