package leetcode

func numSub(s string) (ans int) {
	const mmod = 1_000_000_007

	last0 := -1

	for i, ch := range s {
		if ch == '0' {
			last0 = i
		} else {
			ans += i - last0
		}
	}
	return ans % mmod
}
