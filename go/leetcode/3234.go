package leetcode

func numberOfSubstrings(s string) (ans int) {
	pos0 := []int{-1}
	total1 := 0
	for r, c := range s {
		if c == '0' {
			pos0 = append(pos0, r)
		} else {
			total1++
			ans += r - pos0[len(pos0)-1]
		}

		m := len(pos0)
		for i := m - 1; i > 0 && (m-i)*(m-i) <= total1; i-- {
			p, q := pos0[i-1], pos0[i]
			cnt0 := m - i
			cnt1 := r - q + 1 - cnt0
			ans += max(q-max(cnt0*cnt0-cnt1, 0)-p, 0)
		}
	}

	return
}
