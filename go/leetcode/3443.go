package leetcode

func maxDistance3443(s string, k int) (ans int) {
	cnt := ['X']int{}

	for _, ch := range s {
		cnt[ch]++
		left := k
		f := func(a, b int) int {
			d := min(a, min(b, left))
			left -= d
			return abs(a-b) + 2*d
		}
		ans = max(ans, f(cnt['S'], cnt['N'])+f(cnt['E'], cnt['W']))
	}

	return
}
