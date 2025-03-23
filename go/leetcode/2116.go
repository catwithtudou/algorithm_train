package leetcode

func canBeValid(s string, locked string) bool {
	if len(s)%2 != 0 {
		return false
	}

	mn, mx := 0, 0

	for i, locked := range locked {
		if locked == '1' {
			d := 1 - int(s[i]%2*2)
			mx += d
			if mx < 0 {
				return false
			}
			mn += d
		} else {
			mx++
			mn--
		}
		if mn < 0 {
			mn = 1

		}
	}

	return mn == 0
}
