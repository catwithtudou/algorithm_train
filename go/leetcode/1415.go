package leetcode

func getHappyString(n int, k int) string {
	if k > 3<<(n-1) {
		return ""
	}

	k--
	ans := make([]byte, n)
	ans[0] = 'a' + byte(k>>(n-1))
	for i := 1; i < n; i++ {
		ans[i] = 'a' + byte(k>>(n-1-i)&1)
		if ans[i] >= ans[i-1] {
			ans[i]++
		}
	}

	return string(ans)
}
