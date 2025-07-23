package leetcode

func maximumGain(s string, x int, y int) (ans int) {
	a, b := 'a', 'b'
	if x < y {
		a, b = b, a
		x, y = y, x
	}

	cnt1, cnt2 := 0, 0

	for _, v := range s {
		if v == a {
			cnt1++
		} else if v == b {
			if cnt1 > 0 {
				cnt1--
				ans += x
			} else {
				cnt2++
			}
		} else {
			ans += min(cnt1, cnt2) * y
			cnt1, cnt2 = 0, 0
		}
	}
	ans += min(cnt1, cnt2) * y
	return
}
