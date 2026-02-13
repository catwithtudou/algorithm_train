package leetcode

func longestBalanced3714(s string) (ans int) {
	n := len(s)

	// 1
	for i := 0; i < n; {
		start := i
		for i++; i < n && s[i] == s[i-1]; i++ {
		}
		ans = max(ans, i-start)
	}

	// 2
	two := func(x, y byte) {
		for i := 0; i < n; i++ {
			pod := map[int]int{0: i - 1}
			d := 0
			for ; i < n && (s[i] == x || s[i] == y); i++ {
				if s[i] == x {
					d++
				} else {
					d--
				}
				if j, ok := pod[d]; ok {
					ans = max(ans, i-j)
				} else {
					pod[d] = i
				}
			}
		}
	}

	two('a', 'b')
	two('a', 'c')
	two('b', 'c')

	// 3
	type pair struct{ diffX, diffY int }
	pos := map[pair]int{{}: -1}
	cnt := [3]int{}
	for i, b := range s {
		cnt[b-'a']++
		p := pair{cnt[0] - cnt[1], cnt[1] - cnt[2]}
		if j, ok := pos[p]; ok {
			ans = max(ans, i-j)
		} else {
			pos[p] = i
		}
	}

	return ans
}
